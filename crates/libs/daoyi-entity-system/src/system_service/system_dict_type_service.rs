use crate::system_entity::prelude::*;
use crate::system_entity::system_dict_type;
use daoyi_common_support::database;
use daoyi_common_support::error::ApiResult;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::system_vo::{DictTypePageReqVO, DictTypeRespVO};
use sea_orm::entity::prelude::*;
use sea_orm::{QueryOrder, QueryTrait};

pub async fn get_dict_type_page(
    params: &DictTypePageReqVO,
) -> ApiResult<PageResult<DictTypeRespVO>> {
    let db = database::get_db_async().await;
    let paginator = SystemDictType::find_perm()
        .await
        .apply_if(params.name.as_ref(), |query, val| {
            query.filter(system_dict_type::Column::Name.contains(val))
        })
        .apply_if(params.r#type.as_ref(), |query, val| {
            query.filter(system_dict_type::Column::Type.eq(val))
        })
        .apply_if(params.status, |query, val| {
            query.filter(system_dict_type::Column::Status.eq(val))
        })
        .apply_if(params.create_time.as_ref(), |query, val| {
            query.filter(system_dict_type::Column::CreateTime.between(val[0], val[1]))
        })
        .order_by_desc(system_dict_type::Column::CreateTime)
        .paginate(&db, params.pagination.page_size);

    let total = paginator.num_items().await?;
    let list = paginator
        .fetch_page(params.pagination.page_no - 1)
        .await?
        .into_iter()
        .map(|m| m.into())
        .collect();
    let page = PageResult::from_pagination(&params.pagination, total, list);
    Ok(page)
}
