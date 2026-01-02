use crate::infra_entity::infra_file_config;
use crate::infra_entity::prelude::InfraFileConfig;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::FileStorageEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::serde::validate_and_parse;
use daoyi_common_support::vo::infra_vo::{
    DbFileClientConfig, FileConfigPageReqVO, FileConfigSaveReqVo, FileConfigUpdateReqVo,
    FtpFileClientConfig, LocalFileClientConfig, S3FileClientConfig, SftpFileClientConfig,
};
use sea_orm::prelude::Json;
use sea_orm::*;

pub async fn get_file_config_page(
    params: &FileConfigPageReqVO,
) -> ApiResult<Page<infra_file_config::Model>> {
    let db = database::get_db_async().await;
    let paginator = InfraFileConfig::find()
        .filter(infra_file_config::Column::Deleted.eq(false))
        .apply_if(params.name.as_ref(), |query, name| {
            query.filter(infra_file_config::Column::Name.contains(name))
        })
        .apply_if(params.storage, |query, storage| {
            query.filter(infra_file_config::Column::Storage.eq(storage))
        })
        .apply_if(params.create_time.as_ref(), |query, create_time| {
            query.filter(
                infra_file_config::Column::CreateTime.between(create_time[0], create_time[1]),
            )
        })
        .order_by_desc(infra_file_config::Column::CreateTime)
        .paginate(&db, params.pagination.page_size);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(params.pagination.page_no - 1).await?;
    let page = Page::from_pagination(&params.pagination, total, list);
    Ok(page)
}

pub async fn create_file_config(vo: FileConfigSaveReqVo) -> ApiResult<infra_file_config::Model> {
    validate_file_config_storage(&vo.storage, &vo.config).await?;
    let db = database::get_db_async().await;
    let active_model: infra_file_config::ActiveModel = vo.into();
    let model = active_model.insert(&db).await?;
    Ok(model)
}

pub async fn update_file_config(vo: FileConfigUpdateReqVo) -> ApiResult<()> {
    validate_file_config_storage(&vo.storage, &vo.config).await?;
    // 校验存在
    let mut active_model = get_file_config(&vo.id).await?.into_active_model();
    active_model.config = Set(vo.config);
    active_model.name = Set(vo.name);
    active_model.remark = Set(vo.remark);
    active_model.storage = Set(vo.storage);
    let db = database::get_db_async().await;
    active_model.update(&db).await?;
    Ok(())
}

async fn validate_file_config_storage(storage: &FileStorageEnum, config: &Json) -> ApiResult<()> {
    match storage {
        FileStorageEnum::DB => {
            validate_and_parse::<DbFileClientConfig>(config)?;
        }
        FileStorageEnum::LOCAL => {
            validate_and_parse::<LocalFileClientConfig>(config)?;
        }
        FileStorageEnum::FTP => {
            validate_and_parse::<FtpFileClientConfig>(config)?;
        }
        FileStorageEnum::SFTP => {
            validate_and_parse::<SftpFileClientConfig>(config)?;
        }
        FileStorageEnum::S3 => {
            let config = validate_and_parse::<S3FileClientConfig>(config)?;
            if config.endpoint.contains("qiniucs.com")
                && config.domain.as_deref().unwrap_or("").is_empty()
            {
                return Err(ApiError::valid("domain 不能为空"));
            }
        }
    }
    Ok(())
}

pub async fn get_file_config(id: &str) -> ApiResult<infra_file_config::Model> {
    let db = database::get_db_async().await;
    let model = InfraFileConfig::find_perm()
        .await
        .filter(infra_file_config::Column::Id.eq(id))
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::biz("文件配置不存在"))?;
    Ok(model)
}
