use crate::system_entity::prelude::*;
use crate::system_entity::{system_notify_message, system_notify_template};
use daoyi_common_support::database;
use daoyi_common_support::enumeration::UserTypeEnum;
use daoyi_common_support::error::ApiResult;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::system_vo::{
    NotifyMessageMyPageReqVO, NotifyMessagePageReqVO, NotifyMessageRespVo,
};
use sea_orm::prelude::*;
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{QueryOrder, QuerySelect, QueryTrait, Set};
use std::collections::HashMap;

pub async fn get_notify_message(id: &str) -> ApiResult<Option<system_notify_message::Model>> {
    Ok(
        SystemNotifyMessage::find_by_id_perm_with_tenant(&database::get_db_async().await, id)
            .await?,
    )
}

pub async fn get_notify_message_page(
    params: &NotifyMessagePageReqVO,
) -> ApiResult<PageResult<NotifyMessageRespVo>> {
    let db = database::get_db_async().await;
    let paginator = SystemNotifyMessage::find_perm_with_tenant()
        .await
        .apply_if(params.user_id.as_ref(), |query, val| {
            query.filter(system_notify_message::Column::UserId.eq(val))
        })
        .apply_if(params.user_type, |query, val| {
            query.filter(system_notify_message::Column::UserType.eq(val))
        })
        .apply_if(params.template_code.as_ref(), |query, val| {
            query.filter(system_notify_message::Column::TemplateCode.contains(val))
        })
        .apply_if(params.template_type, |query, val| {
            query.filter(system_notify_message::Column::TemplateType.eq(val))
        })
        .apply_if(params.create_time.as_ref(), |query, val| {
            query.filter(system_notify_message::Column::CreateTime.between(val[0], val[1]))
        })
        .order_by_desc(system_notify_message::Column::CreateTime)
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

pub async fn get_my_notify_message_page(
    params: &NotifyMessageMyPageReqVO,
    user_id: &str,
    user_type: UserTypeEnum,
) -> ApiResult<PageResult<NotifyMessageRespVo>> {
    let db = database::get_db_async().await;
    let paginator = SystemNotifyMessage::find_perm_with_tenant()
        .await
        .apply_if(params.read_status, |query, val| {
            query.filter(system_notify_message::Column::ReadStatus.eq(val))
        })
        .filter(system_notify_message::Column::UserType.eq(user_id))
        .filter(system_notify_message::Column::UserType.eq(user_type))
        .apply_if(params.create_time.as_ref(), |query, val| {
            query.filter(system_notify_message::Column::CreateTime.between(val[0], val[1]))
        })
        .order_by_desc(system_notify_message::Column::CreateTime)
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

pub async fn update_notify_message_read(
    ids: &Vec<String>,
    user_id: &str,
    user_type: UserTypeEnum,
) -> ApiResult<u64> {
    let result = SystemNotifyMessage::update_many_auto()
        .await
        .filter(system_notify_message::Column::Id.is_in(ids))
        .filter(system_notify_message::Column::ReadStatus.eq(false))
        .filter(system_notify_message::Column::UserId.eq(user_id))
        .filter(system_notify_message::Column::UserType.eq(user_type))
        .col_expr(system_notify_message::Column::ReadStatus, Expr::value(true))
        .col_expr(
            system_notify_message::Column::ReadTime,
            Expr::value(Local::now().naive_local()),
        )
        .exec(&database::get_db_async().await)
        .await?;
    Ok(result.rows_affected)
}

pub async fn update_all_notify_message_read(
    user_id: &str,
    user_type: UserTypeEnum,
) -> ApiResult<u64> {
    let result = SystemNotifyMessage::update_many_auto()
        .await
        .filter(system_notify_message::Column::ReadStatus.eq(false))
        .filter(system_notify_message::Column::UserId.eq(user_id))
        .filter(system_notify_message::Column::UserType.eq(user_type))
        .col_expr(system_notify_message::Column::ReadStatus, Expr::value(true))
        .col_expr(
            system_notify_message::Column::ReadTime,
            Expr::value(Local::now().naive_local()),
        )
        .exec(&database::get_db_async().await)
        .await?;
    Ok(result.rows_affected)
}

pub async fn get_unread_notify_message_list(
    user_id: &str,
    user_type: UserTypeEnum,
    size: u64,
) -> ApiResult<Vec<system_notify_message::Model>> {
    Ok(SystemNotifyMessage::find_perm_with_tenant()
        .await
        .filter(system_notify_message::Column::ReadStatus.eq(false))
        .filter(system_notify_message::Column::UserId.eq(user_id))
        .filter(system_notify_message::Column::UserType.eq(user_type))
        .limit(size)
        .all(&database::get_db_async().await)
        .await?)
}

pub async fn get_unread_notify_message_count(
    user_id: &str,
    user_type: UserTypeEnum,
) -> ApiResult<u64> {
    Ok(SystemNotifyMessage::find_perm_with_tenant()
        .await
        .filter(system_notify_message::Column::ReadStatus.eq(false))
        .filter(system_notify_message::Column::UserId.eq(user_id))
        .filter(system_notify_message::Column::UserType.eq(user_type))
        .count(&database::get_db_async().await)
        .await?)
}

pub async fn create_notify_message(
    user_id: String,
    user_type: UserTypeEnum,
    template: system_notify_template::Model,
    template_content: String,
    template_params: HashMap<String, String>,
) -> ApiResult<system_notify_message::Model> {
    let active_model = system_notify_message::ActiveModel {
        user_id: Set(user_id),
        user_type: Set(user_type),
        template_id: Set(template.id),
        template_code: Set(template.code),
        template_type: Set(template.r#type),
        template_nickname: Set(template.nickname),
        template_content: Set(template_content),
        template_params: Set(serde_json::to_value(template_params)?),
        read_status: Set(false),
        ..Default::default()
    };
    let model = active_model.insert(&database::get_db_async().await).await?;
    Ok(model)
}
