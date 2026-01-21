use crate::infra_entity::{infra_data_source_config, prelude::*};
use daoyi_common_support::configs::AppConfig;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::ID_ROOT;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::vo::infra_vo::{DataSourceConfigSaveReqVO, DataSourceConfigUpdateReqVO};
use sea_orm::{
    ActiveModelTrait, ConnectOptions, ConnectionTrait, Database, DatabaseConnection, QueryOrder,
    Statement,
};
use std::time::Duration;

pub async fn create_data_source_config(req: DataSourceConfigSaveReqVO) -> ApiResult<String> {
    check_connection(&req.url, req.username.as_deref(), req.password.as_deref()).await?;
    let db = database::get_db_async().await;
    let model: infra_data_source_config::ActiveModel = req.into();
    let res = model.insert(&db).await?;
    Ok(res.id)
}

pub async fn update_data_source_config(req: DataSourceConfigUpdateReqVO) -> ApiResult<()> {
    validate_data_source_config_exists(&req.id).await?;
    check_connection(&req.url, req.username.as_deref(), req.password.as_deref()).await?;
    if ID_ROOT == req.id {
        return Ok(());
    }
    let db = database::get_db_async().await;
    let active_model: infra_data_source_config::ActiveModel = req.into();
    active_model.update(&db).await?;
    Ok(())
}

pub async fn validate_data_source_config_exists(
    id: &str,
) -> ApiResult<infra_data_source_config::Model> {
    get_data_source_config(id)
        .await?
        .ok_or_else(|| ApiError::biz("数据源配置不存在"))
}

pub async fn get_database_conn_by_id(id: &str) -> ApiResult<DatabaseConnection> {
    let config = validate_data_source_config_exists(id).await?;
    let mut clean_url = config.url;
    if clean_url.starts_with("jdbc:") {
        clean_url = clean_url.replacen("jdbc:", "", 1);
    }
    let username = config.username.as_deref().unwrap_or("");
    let password = config.password_plaintext.as_deref().unwrap_or("");
    if !username.is_empty() && !clean_url.contains(&format!("{}:{}@", username, password)) {
        if let Some(scheme_end) = clean_url.find("://") {
            let (scheme, rest) = clean_url.split_at(scheme_end + 3);
            clean_url = format!("{}{}:{}@{}", scheme, username, password, rest);
        }
    }
    let mut opts = ConnectOptions::new(clean_url);
    opts.connect_timeout(Duration::from_secs(5))
        .sqlx_logging(false)
        .set_schema_search_path(config.schema_name);
    let db = Database::connect(opts).await?;
    Ok(db)
}

async fn check_connection(
    url: &str,
    username: Option<&str>,
    password: Option<&str>,
) -> ApiResult<()> {
    let mut clean_url = url.trim().to_string();
    if clean_url.starts_with("jdbc:") {
        clean_url = clean_url.replacen("jdbc:", "", 1);
    }

    let username = username.unwrap_or("");
    let password = password.unwrap_or("");

    if !username.is_empty() && !clean_url.contains(&format!("{}:{}@", username, password)) {
        if let Some(scheme_end) = clean_url.find("://") {
            let (scheme, rest) = clean_url.split_at(scheme_end + 3);
            clean_url = format!("{}{}:{}@{}", scheme, username, password, rest);
        }
    }

    let mut opts = ConnectOptions::new(clean_url);
    opts.connect_timeout(Duration::from_secs(5))
        .sqlx_logging(false);

    let db = Database::connect(opts)
        .await
        .map_err(|e| ApiError::biz(format!("数据源连接失败: {}", e)))?;
    let _ = db
        .execute(Statement::from_string(
            db.get_database_backend(),
            "SELECT 1".to_owned(),
        ))
        .await
        .map_err(|e| ApiError::biz(format!("数据源连接测试失败: {}", e)))?;

    db.close().await?;

    Ok(())
}

pub async fn delete_data_source_config(id: &str) -> ApiResult<()> {
    if ID_ROOT == id {
        return Ok(());
    }
    validate_data_source_config_exists(id).await?;
    let db = database::get_db_async().await;
    InfraDataSourceConfig::delete_logical_by_id(&db, id).await?;
    Ok(())
}

pub async fn get_data_source_config(
    id: &str,
) -> ApiResult<Option<infra_data_source_config::Model>> {
    if id == ID_ROOT {
        return Ok(Some(AppConfig::get().database().into()));
    }
    let db = database::get_db_async().await;
    let config = InfraDataSourceConfig::find_by_id_perm_with_tenant(&db, id).await?;
    Ok(config)
}

pub async fn get_data_source_config_list() -> ApiResult<Vec<infra_data_source_config::Model>> {
    let db = database::get_db_async().await;
    let mut list = InfraDataSourceConfig::find_perm_with_tenant()
        .await
        .order_by_desc(infra_data_source_config::Column::CreateTime)
        .all(&db)
        .await?;
    list.insert(0, AppConfig::get().database().into());
    Ok(list)
}
