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
use daoyi_macros::transactional;
use sea_orm::prelude::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QueryTrait, Set, Statement,
};
use std::collections::{HashMap, HashSet};
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

pub async fn get_codegen_table_list(
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

#[transactional]
pub async fn create_codegen_list(req: CodegenCreateListReqVO) -> ApiResult<Vec<String>> {
    let target_db =
        infra_data_source_config_service::get_db_conn(&req.data_source_config_id).await?;
    let main_db = database::get_db_async().await;
    let tasks = req.table_names.into_iter().map(|table_name| {
        let target_db = target_db.clone();
        let main_db = main_db.clone();
        let config_id = req.data_source_config_id.clone();
        async move {
            // 1. Get Table Info
            // 2. Build Table Model
            let table_model = build_table_from_db(&target_db, &config_id, &table_name).await?;
            // Insert Table
            let table_id = table_model.insert(&main_db).await?.id;
            // 3. Get and Build Columns Info
            let columns = build_column_from_db(&target_db, &table_id, &table_name).await?;
            // 4. Insert Columns
            for column_model in columns {
                column_model.insert(&main_db).await?;
            }
            Ok::<String, ApiError>(table_id)
        }
    });
    let ids = futures::future::try_join_all(tasks).await?;
    target_db.close().await?;
    Ok(ids)
}

pub async fn build_table_from_db(
    conn: &DatabaseConnection,
    data_source_config_id: &str,
    table_name: &str,
) -> ApiResult<infra_codegen_table::ActiveModel> {
    // 1. Get Table Info
    let sql_table = match conn.get_database_backend() {
        DbBackend::Postgres => Ok(format!(
            r#"
                SELECT t.table_name           as table_name,
                   obj_description(c.oid) as table_comment
                    FROM information_schema.tables t
                             JOIN pg_class c ON c.relname = t.table_name
                    WHERE t.table_type = 'BASE TABLE'
                      and t.table_schema = current_schema() and t.table_name = '{table_name}'
                "#,
        )),
        _ => Err(ApiError::biz("不支持的数据库类型")),
    }?;

    let stmt_table = Statement::from_string(conn.get_database_backend(), sql_table);
    let table_res = conn
        .query_one(stmt_table)
        .await
        .map_err(|e| ApiError::biz(format!("查询表 {} 失败: {}", table_name, e)))?
        .ok_or_else(|| ApiError::biz(format!("表 {} 不存在", table_name)))?;

    let comment: String = table_res.try_get("", "table_comment").unwrap_or_default();

    // 2. Build Table Model
    Ok(CodegenBuilder::build_table(
        data_source_config_id,
        table_name,
        &comment,
    ))
}

pub async fn build_column_from_db(
    conn: &DatabaseConnection,
    table_id: &str,
    table_name: &str,
) -> ApiResult<Vec<infra_codegen_column::ActiveModel>> {
    // 3. Get Columns Info
    let sql_columns = match conn.get_database_backend() {
        DbBackend::Postgres => Ok(format!(
            r#"
                SELECT a.attname                                         AS column_name,
                       sc.udt_name                                       AS data_type,
                       col_description(a.attrelid, a.attnum)             AS column_comment,
                       CASE WHEN a.attnotnull THEN 'NO' ELSE 'YES' END   AS is_nullable,
                       CASE WHEN pk.contype = 'p' THEN 'PRI' ELSE '' END AS column_key,
                       a.attnum                                          AS ordinal_position
                FROM pg_attribute a
                         JOIN pg_class c ON a.attrelid = c.oid
                         JOIN pg_namespace n ON c.relnamespace = n.oid
                         LEFT JOIN pg_constraint pk ON c.oid = pk.conrelid AND a.attnum = ANY (pk.conkey) AND pk.contype = 'p'
                         join information_schema.columns sc
                              on sc.table_schema = n.nspname and sc.table_name = c.relname and sc.column_name = a.attname
                WHERE n.nspname = current_schema()
                  AND c.relname = '{table_name}'
                  AND a.attnum > 0
                  AND NOT a.attisdropped
                ORDER BY a.attnum
                "#,
        )),
        _ => Err(ApiError::biz("不支持的数据库类型")),
    }?;

    let stmt_columns = Statement::from_string(conn.get_database_backend(), sql_columns);
    let columns_res = conn
        .query_all(stmt_columns)
        .await
        .map_err(|e| ApiError::biz(format!("查询表 {} 字段失败: {}", table_name, e)))?
        .into_iter()
        .map(|col_row| {
            let col_name: String = col_row.try_get("", "column_name").unwrap_or_default();
            let data_type: String = col_row.try_get("", "data_type").unwrap_or_default();
            let col_comment: String = col_row.try_get("", "column_comment").unwrap_or_default();
            let is_nullable_str: String = col_row.try_get("", "is_nullable").unwrap_or_default();
            let column_key: String = col_row.try_get("", "column_key").unwrap_or_default();
            let ordinal_position: i32 = col_row.try_get("", "ordinal_position").unwrap_or(0);

            let is_nullable = is_nullable_str.to_uppercase() == "YES";
            let is_primary = column_key == "PRI";

            CodegenBuilder::build_column(
                &table_id,
                &col_name,
                &data_type,
                &col_comment,
                is_nullable,
                is_primary,
                ordinal_position,
            )
        })
        .collect();
    Ok(columns_res)
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
    let table = InfraCodegenTable::find_by_id_perm_with_tenant(&db, table_id)
        .await?
        .ok_or_else(|| ApiError::biz("表不存在"))?;
    delete_codegen(table_id).await?;
    create_codegen_list(CodegenCreateListReqVO {
        data_source_config_id: table.data_source_config_id,
        table_names: vec![table.table_name],
    })
    .await?;
    Ok(())
}

pub async fn delete_codegen(table_id: &str) -> ApiResult<()> {
    let db = database::get_db_async().await;
    // Delete columns first
    InfraCodegenColumn::update_many_auto()
        .await
        .filter(infra_codegen_column::Column::TableId.eq(table_id))
        .col_expr(infra_codegen_column::Column::Deleted, Expr::value(true))
        .exec(&db)
        .await?;
    // Delete table
    InfraCodegenTable::delete_logical_by_id(&db, table_id).await?;
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

pub async fn generation_codes(table_id: &str) -> ApiResult<HashMap<String, String>> {
    let db = database::get_db_async().await;
    let table = get_codegen_table(table_id)
        .await?
        .ok_or_else(|| ApiError::biz("表不存在"))?;
    let columns = get_codegen_columns(table_id).await?;

    // Load Sub-tables
    let (sub_tables, sub_columns_list) = get_sub_tables(&table, &db).await?;

    Ok(CodegenEngine::execute(
        &table,
        &columns,
        &sub_tables,
        &sub_columns_list,
    ))
}

pub async fn download_codegen(table_id: &str) -> ApiResult<Vec<u8>> {
    let codes = generation_codes(table_id).await?;

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
