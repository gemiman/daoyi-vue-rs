use crate::system_entity::{system_sms_log, prelude::*};
use daoyi_common_support::error::ApiResult;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::system_vo::SmsLogPageReqVO;
use daoyi_common_support::database;
use sea_orm::{EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, ColumnTrait, QueryTrait};
use sea_orm::prelude::DateTime;

pub async fn get_sms_log_page(req: SmsLogPageReqVO) -> ApiResult<PageResult<system_sms_log::Model>> {
    let db = database::get_db_async().await;

    let paginator = SystemSmsLog::find()
        .apply_if(req.channel_id, |query, val: String| {
            query.filter(system_sms_log::Column::ChannelId.eq(val))
        })
        .apply_if(req.template_id, |query, val: String| {
            query.filter(system_sms_log::Column::TemplateId.eq(val))
        })
        .apply_if(req.mobile, |query, val: String| {
            query.filter(system_sms_log::Column::Mobile.contains(val))
        })
        .apply_if(req.send_status, |query, val: i16| {
            query.filter(system_sms_log::Column::SendStatus.eq(val))
        })
        .apply_if(req.receive_status, |query, val: bool| {
            query.filter(system_sms_log::Column::ReceiveStatus.eq(val))
        })
        .apply_if(req.send_time, |query, val: Vec<DateTime>| {
            if val.len() == 2 {
                query.filter(system_sms_log::Column::SendTime.between(val[0], val[1]))
            } else {
                query
            }
        })
        .order_by_desc(system_sms_log::Column::Id)
        .paginate(&db, req.pagination.page_size);

    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(req.pagination.page_no - 1).await?;

    Ok(PageResult::from_pagination(&req.pagination, total, list))
}
