use crate::system_entity::prelude::*;
use crate::system_entity::system_login_log;
use daoyi_common_support::database;
use daoyi_common_support::error::ApiResult;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::system_vo::login_log_vo::{
    LoginLogCreateReqDTO, LoginLogPageReqVO, LoginLogRespVO,
};
use sea_orm::entity::prelude::*;
use sea_orm::{QueryOrder, QueryTrait};

pub async fn get_login_log_page(
    params: &LoginLogPageReqVO,
) -> ApiResult<PageResult<LoginLogRespVO>> {
    let db = database::get_db_async().await;
    let paginator = SystemLoginLog::find_perm_with_tenant()
        .await
        .apply_if(params.user_ip.as_ref(), |query, val| {
            query.filter(system_login_log::Column::UserIp.contains(val))
        })
        .apply_if(params.username.as_ref(), |query, val| {
            query.filter(system_login_log::Column::Username.contains(val))
        })
        .apply_if(params.create_time.as_ref(), |query, val| {
            query.filter(system_login_log::Column::CreateTime.between(val[0], val[1]))
        })
        .order_by_desc(system_login_log::Column::CreateTime)
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

pub async fn create_login_log(vo: LoginLogCreateReqDTO) -> ApiResult<()> {
    let db = database::get_db_async().await;
    let active_model: system_login_log::ActiveModel = vo.into();
    active_model.insert(&db).await?;
    Ok(())
}
