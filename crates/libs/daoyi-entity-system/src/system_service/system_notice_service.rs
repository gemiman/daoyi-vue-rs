use crate::system_entity::prelude::*;
use crate::system_entity::system_notice;
use daoyi_common_support::database;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::system_vo::{
    NoticePageReqVO, NoticeRespVO, NoticeSaveReqVO, NoticeUpdateReqVO,
};
use sea_orm::prelude::*;
use sea_orm::{QueryOrder, QueryTrait};

pub async fn create_notice(vo: NoticeSaveReqVO) -> ApiResult<system_notice::Model> {
    let db = database::get_db_async().await;
    let active_model: system_notice::ActiveModel = vo.into();
    Ok(active_model.insert(&db).await?)
}

pub async fn get_notice(id: &str) -> ApiResult<Option<system_notice::Model>> {
    let db = database::get_db_async().await;
    Ok(SystemNotice::find_by_id_perm_with_tenant(&db, id).await?)
}

async fn validate_notice_exists(id: &str) -> ApiResult<system_notice::Model> {
    get_notice(id)
        .await?
        .ok_or_else(|| ApiError::biz("当前通知公告不存在"))
}

pub async fn update_notice(vo: NoticeUpdateReqVO) -> ApiResult<system_notice::Model> {
    // 校验是否存在
    validate_notice_exists(&vo.id).await?;
    // 更新通知公告
    let db = database::get_db_async().await;
    let active_model: system_notice::ActiveModel = vo.into();
    Ok(active_model.update(&db).await?)
}

pub async fn delete_notice(id: &str) -> ApiResult<()> {
    // 校验是否存在
    validate_notice_exists(id).await?;
    // 删除通知公告
    let db = database::get_db_async().await;
    SystemNotice::delete_logical_by_id(&db, id).await?;
    Ok(())
}

pub async fn delete_notice_list(ids: &Vec<String>) -> ApiResult<()> {
    let db = database::get_db_async().await;
    SystemNotice::delete_logical_by_ids(&db, ids).await?;
    Ok(())
}

pub async fn get_notice_page(params: &NoticePageReqVO) -> ApiResult<PageResult<NoticeRespVO>> {
    let db = database::get_db_async().await;
    let paginator = SystemNotice::find_perm_with_tenant()
        .await
        .apply_if(params.title.as_ref(), |query, title| {
            query.filter(system_notice::Column::Title.contains(title))
        })
        .apply_if(params.status, |query, status| {
            query.filter(system_notice::Column::Status.eq(status))
        })
        .order_by_desc(system_notice::Column::CreateTime)
        .paginate(&db, params.pagination.page_size);
    let total = paginator.num_items().await?;
    let list = paginator
        .fetch_page(params.pagination.page_no - 1)
        .await?
        .into_iter()
        .map(|item| item.into())
        .collect();
    let page = PageResult::from_pagination(&params.pagination, total, list);
    Ok(page)
}
