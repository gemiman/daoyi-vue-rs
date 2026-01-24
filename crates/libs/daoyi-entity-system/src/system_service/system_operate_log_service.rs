use crate::system_entity::prelude::*;
use crate::system_entity::system_operate_log;
use daoyi_common_support::database;
use daoyi_common_support::error::ApiResult;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::system_vo::operate_log_vo::{
    OperateLogCreateReqDTO, OperateLogPageReqVO, OperateLogRespVO,
};
use sea_orm::entity::prelude::*;
use sea_orm::{QueryOrder, QueryTrait};

pub async fn get_operate_log_page(
    params: &OperateLogPageReqVO,
) -> ApiResult<PageResult<OperateLogRespVO>> {
    let db = database::get_db_async().await;
    let paginator = SystemOperateLog::find_perm_with_tenant()
        .await
        .apply_if(params.user_id.as_ref(), |query, val| {
            query.filter(system_operate_log::Column::UserId.eq(val))
        })
        .apply_if(params.biz_id.as_ref(), |query, val| {
            query.filter(system_operate_log::Column::BizId.eq(val))
        })
        .apply_if(params.r#type.as_ref(), |query, val| {
            query.filter(system_operate_log::Column::Type.contains(val))
        })
        .apply_if(params.sub_type.as_ref(), |query, val| {
            query.filter(system_operate_log::Column::SubType.contains(val))
        })
        .apply_if(params.action.as_ref(), |query, val| {
            query.filter(system_operate_log::Column::Action.contains(val))
        })
        .apply_if(params.create_time.as_ref(), |query, val| {
            query.filter(system_operate_log::Column::CreateTime.between(val[0], val[1]))
        })
        .order_by_desc(system_operate_log::Column::CreateTime)
        .paginate(&db, params.pagination.page_size);

    let total = paginator.num_items().await?;
    let list = paginator
        .fetch_page(params.pagination.page_no - 1)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    let page = PageResult::from_pagination(&params.pagination, total, list);
    Ok(page)
}

pub async fn get_operate_log(id: &str) -> ApiResult<Option<system_operate_log::Model>> {
    Ok(SystemOperateLog::find_by_id_perm_with_tenant(&database::get_db_async().await, id).await?)
}

pub async fn create_operate_log(vo: OperateLogCreateReqDTO) -> ApiResult<()> {
    let db = database::get_db_async().await;
    let active_model: system_operate_log::ActiveModel = vo.into();
    active_model.insert(&db).await?;
    Ok(())
}
