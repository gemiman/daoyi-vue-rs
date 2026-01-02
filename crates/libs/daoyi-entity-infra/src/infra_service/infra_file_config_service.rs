use crate::infra_entity::infra_file_config;
use crate::infra_entity::prelude::InfraFileConfig;
use daoyi_common_support::database;
use daoyi_common_support::error::ApiResult;
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::vo::infra_vo::FileConfigPageReqVO;
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
