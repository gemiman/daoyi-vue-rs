use crate::infra_entity::{infra_codegen_column, infra_codegen_table, prelude::*};
use crate::infra_service::infra_codegen_builder::CodegenBuilder;
use crate::infra_service::infra_codegen_engine::CodegenEngine;
use crate::infra_service::infra_data_source_config_service;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::CodegenTemplateTypeEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::infra_vo::{
    CodegenCreateListReqVO, CodegenTablePageReqVO, CodegenTableRespVO, CodegenUpdateReqVO,
    DatabaseTableRespVO,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, Database, DbBackend, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QueryTrait, Set, Statement,
};
use std::collections::HashSet;
use std::io::Write;

pub async fn get_database_table_list(
    data_source_config_id: &str,
    name: Option<String>,
    comment: Option<String>,
) -> ApiResult<Vec<DatabaseTableRespVO>> {
    let conn = infra_data_source_config_service::get_db_conn(data_source_config_id).await?;
    let sql = match conn.get_database_backend() {
        DbBackend::Postgres => Ok(r#"
        SELECT t.table_name           as table_name,
           obj_description(c.oid) as table_comment
            FROM information_schema.tables t
                     JOIN pg_class c ON c.relname = t.table_name
            WHERE t.table_type = 'BASE TABLE'
              and t.table_schema = current_schema()
        "#),
        _ => Err(ApiError::biz("不支持的数据库类型")),
    }?;
    let stmt = Statement::from_string(conn.get_database_backend(), sql.to_owned());
    let results = conn
        .query_all(stmt)
        .await
        .map_err(|e| ApiError::biz(format!("查询表失败: {}", e)))?;
    let exists_tables = get_codegen_table_name_set(data_source_config_id).await?;
    let mut tables: Vec<DatabaseTableRespVO> = results
        .into_iter()
        .map(|res| {
            let name: String = res.try_get("", "table_name").unwrap_or_default();
            let comment: String = res.try_get("", "table_comment").unwrap_or_default();
            DatabaseTableRespVO { name, comment }
        })
        .filter(|table| !exists_tables.contains(&table.name))
        .collect();

    // Filter
    if let Some(n) = name {
        tables.retain(|t| t.name.contains(&n));
    }
    if let Some(c) = comment {
        tables.retain(|t| t.comment.contains(&c));
    }
    conn.close().await?;
    Ok(tables)
}

async fn get_codegen_table_name_set(data_source_config_id: &str) -> ApiResult<HashSet<String>> {
    get_codegen_table_list(data_source_config_id)
        .await
        .map(|list| list.into_iter().map(|item| item.table_name).collect())
}

async fn get_codegen_table_list(
    data_source_config_id: &str,
) -> ApiResult<Vec<infra_codegen_table::Model>> {
    let db = database::get_db_async().await;
    let list = InfraCodegenTable::find_perm_with_tenant()
        .await
        .filter(infra_codegen_table::Column::DataSourceConfigId.eq(data_source_config_id))
        .order_by_asc(infra_codegen_table::Column::CreateTime)
        .all(&db)
        .await?;
    Ok(list)
}

pub async fn get_codegen_table_page(
    params: &CodegenTablePageReqVO,
) -> ApiResult<PageResult<CodegenTableRespVO>> {
    let db = database::get_db_async().await;
    let paginator = InfraCodegenTable::find_perm_with_tenant()
        .await
        .apply_if(params.table_name.as_deref(), |query, val| {
            query.filter(infra_codegen_table::Column::TableName.contains(val))
        })
        .apply_if(params.table_comment.as_deref(), |query, val| {
            query.filter(infra_codegen_table::Column::TableComment.contains(val))
        })
        .apply_if(params.class_name.as_deref(), |query, val| {
            query.filter(infra_codegen_table::Column::ClassName.contains(val))
        })
        .apply_if(params.create_time.as_deref(), |query, val| {
            if val.len() == 2 {
                query.filter(infra_codegen_table::Column::CreateTime.between(val[0], val[1]))
            } else {
                query
            }
        })
        .order_by_desc(infra_codegen_table::Column::CreateTime)
        .paginate(&db, params.pagination.page_size);

    let total = paginator.num_items().await?;
    let list = paginator
        .fetch_page(params.pagination.page_no - 1)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(PageResult::from_pagination(&params.pagination, total, list))
}

pub async fn create_codegen_list(req: CodegenCreateListReqVO) -> ApiResult<Vec<String>> {
    let config = infra_data_source_config_service::validate_data_source_config_exists(
        &req.data_source_config_id,
    )
    .await?;
    let target_db = Database::connect(&config.url)
        .await
        .map_err(|e| ApiError::biz(format!("连接数据库失败: {}", e)))?;

    let main_db = database::get_db_async().await;
    let mut ids = Vec::new();

    for table_name in req.table_names {
        // 1. Get Table Info
        let sql_table = format!(
            r#"
            SELECT table_name, table_comment
            FROM information_schema.tables
            WHERE table_schema = (SELECT DATABASE()) AND table_name = '{}'
        "#,
            table_name
        );

        let stmt_table = Statement::from_string(target_db.get_database_backend(), sql_table);
        let table_res = target_db
            .query_one(stmt_table)
            .await
            .map_err(|e| ApiError::biz(format!("查询表 {} 失败: {}", table_name, e)))?
            .ok_or_else(|| ApiError::biz(format!("表 {} 不存在", table_name)))?;

        let comment: String = table_res.try_get("", "table_comment").unwrap_or_default();

        // 2. Build Table Model
        let table_model =
            CodegenBuilder::build_table(&req.data_source_config_id, &table_name, &comment);
        // Insert Table
        let table_res = table_model.insert(&main_db).await?;
        let table_id = table_res.id.clone();
        ids.push(table_id.clone());

        // 3. Get Columns Info
        let sql_columns = format!(
            r#"
            SELECT column_name, data_type, column_comment, is_nullable, column_key, ordinal_position
            FROM information_schema.columns
            WHERE table_schema = (SELECT DATABASE()) AND table_name = '{}'
            ORDER BY ordinal_position
        "#,
            table_name
        );

        let stmt_columns = Statement::from_string(target_db.get_database_backend(), sql_columns);
        let columns_res = target_db
            .query_all(stmt_columns)
            .await
            .map_err(|e| ApiError::biz(format!("查询表 {} 字段失败: {}", table_name, e)))?;

        // 4. Build and Insert Columns
        for col_row in columns_res {
            let col_name: String = col_row.try_get("", "column_name").unwrap_or_default();
            let data_type: String = col_row.try_get("", "data_type").unwrap_or_default();
            let col_comment: String = col_row.try_get("", "column_comment").unwrap_or_default();
            let is_nullable_str: String = col_row.try_get("", "is_nullable").unwrap_or_default();
            let column_key: String = col_row.try_get("", "column_key").unwrap_or_default();
            let ordinal_position: i32 = col_row.try_get("", "ordinal_position").unwrap_or(0);

            let is_nullable = is_nullable_str.to_uppercase() == "YES";
            let is_primary = column_key == "PRI";

            let column_model = CodegenBuilder::build_column(
                &table_id,
                &col_name,
                &data_type,
                &col_comment,
                is_nullable,
                is_primary,
                ordinal_position,
            );

            column_model.insert(&main_db).await?;
        }
    }
    Ok(ids)
}

pub async fn get_codegen_table(table_id: &str) -> ApiResult<Option<infra_codegen_table::Model>> {
    let db = database::get_db_async().await;
    let table = InfraCodegenTable::find_by_id(table_id).one(&db).await?;
    Ok(table)
}

pub async fn get_codegen_columns(table_id: &str) -> ApiResult<Vec<infra_codegen_column::Model>> {
    let db = database::get_db_async().await;
    let columns = InfraCodegenColumn::find()
        .filter(infra_codegen_column::Column::TableId.eq(table_id))
        .order_by_asc(infra_codegen_column::Column::OrdinalPosition)
        .all(&db)
        .await?;
    Ok(columns)
}

pub async fn update_codegen(req: CodegenUpdateReqVO) -> ApiResult<()> {
    let db = database::get_db_async().await;

    // 1. Update Table
    let table_id = req.table.id.clone();
    let mut table_model: infra_codegen_table::ActiveModel =
        InfraCodegenTable::find_by_id(&table_id)
            .one(&db)
            .await?
            .ok_or_else(|| ApiError::biz("表不存在"))?
            .into();

    table_model.table_name = Set(req.table.table_name);
    table_model.table_comment = Set(req.table.table_comment);
    table_model.class_name = Set(req.table.class_name);
    table_model.module_name = Set(req.table.module_name);
    table_model.business_name = Set(req.table.business_name);
    table_model.scene = Set(req.table.scene);
    table_model.template_type = Set(req.table.template_type);
    table_model.master_table_id = Set(req.table.master_table_id);
    table_model.sub_join_column_id = Set(req.table.sub_join_column_id);
    table_model.sub_join_many = Set(req.table.sub_join_many);
    table_model.tree_parent_column_id = Set(req.table.tree_parent_column_id);
    table_model.tree_name_column_id = Set(req.table.tree_name_column_id);
    table_model.update(&db).await?;

    // 2. Update Columns
    for col_req in req.columns {
        let mut col_model: infra_codegen_column::ActiveModel =
            InfraCodegenColumn::find_by_id(&col_req.id)
                .one(&db)
                .await?
                .ok_or_else(|| ApiError::biz("字段不存在"))?
                .into();

        col_model.column_comment = Set(col_req.column_comment);
        col_model.java_type = Set(col_req.java_type);
        col_model.java_field = Set(col_req.java_field);
        col_model.dict_type = Set(col_req.dict_type);
        col_model.example = Set(col_req.example);
        col_model.create_operation = Set(col_req.create_operation);
        col_model.update_operation = Set(col_req.update_operation);
        col_model.list_operation = Set(col_req.list_operation);
        col_model.list_operation_condition = Set(col_req.list_operation_condition);
        col_model.list_operation_result = Set(col_req.list_operation_result);
        col_model.html_type = Set(col_req.html_type);

        col_model.update(&db).await?;
    }

    Ok(())
}

pub async fn sync_codegen_from_db(table_id: &str) -> ApiResult<()> {
    let db = database::get_db_async().await;
    let table = InfraCodegenTable::find_by_id(table_id)
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::biz("表不存在"))?;

    let config = infra_data_source_config_service::validate_data_source_config_exists(
        &table.data_source_config_id,
    )
    .await?;
    let target_db = Database::connect(&config.url)
        .await
        .map_err(|e| ApiError::biz(format!("连接数据库失败: {}", e)))?;

    // Get current DB columns
    let sql_columns = format!(
        r#"
        SELECT column_name, data_type, column_comment, is_nullable, column_key, ordinal_position
        FROM information_schema.columns
        WHERE table_schema = (SELECT DATABASE()) AND table_name = '{}'
        ORDER BY ordinal_position
    "#,
        table.table_name
    );

    let stmt = Statement::from_string(target_db.get_database_backend(), sql_columns);
    let columns_res = target_db
        .query_all(stmt)
        .await
        .map_err(|e| ApiError::biz(format!("查询表 {} 字段失败: {}", table.table_name, e)))?;

    // Get existing codegen columns
    let existing_columns = InfraCodegenColumn::find()
        .filter(infra_codegen_column::Column::TableId.eq(table_id))
        .all(&db)
        .await?;

    let existing_map: std::collections::HashMap<_, _> = existing_columns
        .into_iter()
        .map(|c| (c.column_name.clone(), c))
        .collect();

    let mut new_col_names = Vec::new();

    for col_row in columns_res {
        let col_name: String = col_row.try_get("", "column_name").unwrap_or_default();
        let data_type: String = col_row.try_get("", "data_type").unwrap_or_default();
        let col_comment: String = col_row.try_get("", "column_comment").unwrap_or_default();
        let is_nullable_str: String = col_row.try_get("", "is_nullable").unwrap_or_default();
        let column_key: String = col_row.try_get("", "column_key").unwrap_or_default();
        let ordinal_position: i32 = col_row.try_get("", "ordinal_position").unwrap_or(0);

        let is_nullable = is_nullable_str.to_uppercase() == "YES";
        let is_primary = column_key == "PRI";

        new_col_names.push(col_name.clone());

        if let Some(existing_col) = existing_map.get(&col_name) {
            // Update existing if types changed
            let type_changed = existing_col.data_type != data_type
                || existing_col.nullable != is_nullable
                || existing_col.primary_key != is_primary;

            if type_changed {
                // Re-build column to get new Java/Rust types
                let new_model = CodegenBuilder::build_column(
                    table_id,
                    &col_name,
                    &data_type,
                    &col_comment,
                    is_nullable,
                    is_primary,
                    ordinal_position,
                );

                // Update core fields only, preserve customizations
                let mut active_model: infra_codegen_column::ActiveModel =
                    existing_col.clone().into();
                active_model.data_type = Set(new_model.data_type.unwrap());
                active_model.column_comment = Set(new_model.column_comment.unwrap());
                active_model.nullable = Set(new_model.nullable.unwrap());
                active_model.primary_key = Set(new_model.primary_key.unwrap());
                active_model.ordinal_position = Set(new_model.ordinal_position.unwrap());
                active_model.java_type = Set(new_model.java_type.unwrap());
                active_model.java_field = Set(new_model.java_field.unwrap());

                active_model.update(&db).await?;
            } else if existing_col.column_comment != col_comment
                || existing_col.ordinal_position != ordinal_position
            {
                // Only comment or order changed
                let mut active_model: infra_codegen_column::ActiveModel =
                    existing_col.clone().into();
                active_model.column_comment = Set(col_comment);
                active_model.ordinal_position = Set(ordinal_position);
                active_model.update(&db).await?;
            }
        } else {
            // Add new
            let column_model = CodegenBuilder::build_column(
                table_id,
                &col_name,
                &data_type,
                &col_comment,
                is_nullable,
                is_primary,
                ordinal_position,
            );
            column_model.insert(&db).await?;
        }
    }

    // Delete removed columns
    for (name, col) in existing_map {
        if !new_col_names.contains(&name) {
            InfraCodegenColumn::delete_by_id(col.id).exec(&db).await?;
        }
    }

    Ok(())
}

pub async fn delete_codegen(table_id: &str) -> ApiResult<()> {
    let db = database::get_db_async().await;
    // Delete columns first
    InfraCodegenColumn::delete_many()
        .filter(infra_codegen_column::Column::TableId.eq(table_id))
        .exec(&db)
        .await?;

    // Delete table
    InfraCodegenTable::delete_by_id(table_id).exec(&db).await?;
    Ok(())
}

/// Helper to get sub tables if configured
async fn get_sub_tables(
    table: &infra_codegen_table::Model,
    db: &database::DbGuard,
) -> ApiResult<(
    Vec<infra_codegen_table::Model>,
    Vec<Vec<infra_codegen_column::Model>>,
)> {
    // Check if current table is a master table (template_type = 10 or 11 or 12)
    // 10: Master Normal, 11: Master ERP, 12: Master Inner
    let is_master = [
        CodegenTemplateTypeEnum::MasterNormal,
        CodegenTemplateTypeEnum::MasterErp,
        CodegenTemplateTypeEnum::MasterInner,
    ]
    .contains(&table.template_type);

    if !is_master {
        return Ok((Vec::new(), Vec::new()));
    }

    // Find tables that link to this master
    // Logic: find tables where master_table_id = current.id
    let sub_tables = InfraCodegenTable::find()
        .filter(infra_codegen_table::Column::MasterTableId.eq(&table.id))
        .all(db)
        .await?;

    let mut sub_columns_list = Vec::new();
    for sub in &sub_tables {
        let cols = InfraCodegenColumn::find()
            .filter(infra_codegen_column::Column::TableId.eq(&sub.id))
            .order_by_asc(infra_codegen_column::Column::OrdinalPosition)
            .all(db)
            .await?;
        sub_columns_list.push(cols);
    }

    Ok((sub_tables, sub_columns_list))
}

pub async fn download_codegen(table_id: &str) -> ApiResult<Vec<u8>> {
    let db = database::get_db_async().await;
    let table = get_codegen_table(table_id)
        .await?
        .ok_or_else(|| ApiError::biz("表不存在"))?;
    let columns = get_codegen_columns(table_id).await?;

    // Load Sub-tables
    let (sub_tables, sub_columns_list) = get_sub_tables(&table, &db).await?;

    let codes = CodegenEngine::execute(&table, &columns, &sub_tables, &sub_columns_list);

    let mut buf = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        for (path, content) in codes {
            zip.start_file(path, options)
                .map_err(|e| ApiError::biz(format!("Zip Error: {}", e)))?;
            zip.write_all(content.as_bytes())
                .map_err(|e| ApiError::biz(format!("Zip Error: {}", e)))?;
        }
        zip.finish()
            .map_err(|e| ApiError::biz(format!("Zip Error: {}", e)))?;
    }

    Ok(buf)
}
