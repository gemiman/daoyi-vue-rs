use crate::system_entity::prelude::*;
use crate::system_entity::system_sms_channel;
use daoyi_common_support::database;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::system_vo::{
    SmsChannelPageReqVO, SmsChannelRespVO, SmsChannelSaveReqVO, SmsChannelSimpleRespVO,
    SmsChannelUpdateReqVO,
};
use sea_orm::QueryTrait;
use sea_orm::prelude::*;

pub async fn create_sms_channel(vo: SmsChannelSaveReqVO) -> ApiResult<String> {
    let db = database::get_db_async().await;
    let active_model: system_sms_channel::ActiveModel = vo.into();
    let result = active_model.insert(&db).await?;
    Ok(result.id)
}

pub async fn update_sms_channel(vo: SmsChannelUpdateReqVO) -> ApiResult<()> {
    // 校验是否存在
    validate_sms_channel_exists(&vo.id).await?;
    // 更新
    let db = database::get_db_async().await;
    let active_model: system_sms_channel::ActiveModel = vo.into();
    active_model.update(&db).await?;
    Ok(())
}

pub async fn delete_sms_channel(id: &str) -> ApiResult<()> {
    // 校验是否存在
    validate_sms_channel_exists(id).await?;
    // 删除
    let db = database::get_db_async().await;
    SystemSmsChannel::delete_by_id(id).exec(&db).await?;
    Ok(())
}

pub async fn delete_sms_channel_list(ids: &Vec<String>) -> ApiResult<()> {
    let db = database::get_db_async().await;
    SystemSmsChannel::delete_many()
        .filter(system_sms_channel::Column::Id.is_in(ids))
        .exec(&db)
        .await?;
    Ok(())
}

pub async fn get_sms_channel(id: &str) -> ApiResult<Option<system_sms_channel::Model>> {
    let db = database::get_db_async().await;
    Ok(SystemSmsChannel::find_by_id(id).one(&db).await?)
}

async fn validate_sms_channel_exists(id: &str) -> ApiResult<system_sms_channel::Model> {
    get_sms_channel(id)
        .await?
        .ok_or_else(|| ApiError::biz("当前短信渠道不存在"))
}

pub async fn get_sms_channel_page(
    params: &SmsChannelPageReqVO,
) -> ApiResult<PageResult<SmsChannelRespVO>> {
    let db = database::get_db_async().await;
    let paginator = SystemSmsChannel::find()
        .apply_if(params.signature.as_ref(), |query, signature| {
            query.filter(system_sms_channel::Column::Signature.contains(signature))
        })
        .apply_if(params.status, |query, status| {
            query.filter(system_sms_channel::Column::Status.eq(status))
        })
        // .order_by_desc(system_sms_channel::Column::CreateTime) // Assuming CreateTime exists or ID is sortable
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

pub async fn get_sms_channel_list_simple() -> ApiResult<Vec<SmsChannelSimpleRespVO>> {
    let db = database::get_db_async().await;
    let list = SystemSmsChannel::find().all(&db).await?;
    Ok(list.into_iter().map(Into::into).collect())
}
