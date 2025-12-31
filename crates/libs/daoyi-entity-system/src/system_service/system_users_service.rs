use crate::system_entity::prelude::*;
use crate::system_entity::system_users;
use crate::system_service::{system_dept_service, system_tenant_service, system_user_post_service};
use daoyi_common_support::database;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::vo::system_vo::UserSaveReqVo;
use sea_orm::Set;
use sea_orm::entity::prelude::*;

pub async fn create_user(req_vo: UserSaveReqVo) -> ApiResult<String> {
    // 1.1 校验账户配合
    system_tenant_service::handle_tenant_info_async(async |tenant| {
        let db = database::get().await;
        let count = SystemUsers::find_perm().await.count(db).await? as i32;
        if count > tenant.account_count {
            return Err(ApiError::biz(format!(
                "创建用户失败，原因：超过租户最大租户配额({})！",
                tenant.account_count
            )));
        }
        Ok(())
    })
    .await?;
    // 1.2 校验正确性
    validate_user_for_create_or_update(
        None,
        &req_vo.username,
        req_vo.mobile.as_deref(),
        req_vo.email.as_deref(),
        req_vo.dept_id.as_deref(),
        req_vo.post_ids.as_ref(),
    )
    .await?;
    // 2.1 插入用户
    let db = database::get().await;
    let mut active_model: system_users::ActiveModel = req_vo.into();
    active_model.status = Set(CommonStatusEnum::Enable);
    let model = active_model.insert(db).await?;
    // 2.2 插入关联岗位
    if let Some(post_ids) = model.post_ids
        && !post_ids.is_empty()
    {
        system_user_post_service::save_batch(&model.id, &post_ids).await?;
    }
    Ok(model.id)
}
async fn validate_user_for_create_or_update(
    id: Option<&str>,
    username: &str,
    mobile: Option<&str>,
    email: Option<&str>,
    dept_id: Option<&str>,
    post_ids: Option<&Vec<String>>,
) -> ApiResult<Option<system_users::Model>> {
    // 校验用户存在
    let option = validate_user_exists(id).await?;
    // 校验用户名唯一
    validate_username_unique(id, username).await?;
    // 校验手机号唯一
    validate_mobile_unique(id, mobile).await?;
    // 校验邮箱唯一
    validate_email_unique(id, email).await?;
    // 校验部门处于开启状态
    if let Some(dept_id) = dept_id {
        system_dept_service::validate_dept_list(&vec![String::from(dept_id)]).await?;
    }
    // 校验岗位处于开启状态
    Ok(option)
}

async fn validate_email_unique(id: Option<&str>, email: Option<&str>) -> ApiResult<()> {
    if email.is_none() {
        return Ok(());
    }
    let mobile = email.unwrap();
    let db = database::get().await;
    let option = SystemUsers::find_perm()
        .await
        .filter(system_users::Column::Email.eq(mobile))
        .one(db)
        .await?;
    if option.is_none() {
        return Ok(());
    }
    if id.is_none() {
        return Err(ApiError::biz("邮箱已经存在"));
    }
    if option.unwrap().id != id.unwrap() {
        return Err(ApiError::biz("邮箱已经存在"));
    }
    Ok(())
}

async fn validate_mobile_unique(id: Option<&str>, mobile: Option<&str>) -> ApiResult<()> {
    if mobile.is_none() {
        return Ok(());
    }
    let mobile = mobile.unwrap();
    let db = database::get().await;
    let option = SystemUsers::find_perm()
        .await
        .filter(system_users::Column::Mobile.eq(mobile))
        .one(db)
        .await?;
    if option.is_none() {
        return Ok(());
    }
    if id.is_none() {
        return Err(ApiError::biz("手机号已经存在"));
    }
    if option.unwrap().id != id.unwrap() {
        return Err(ApiError::biz("手机号已经存在"));
    }
    Ok(())
}

async fn validate_username_unique(id: Option<&str>, username: &str) -> ApiResult<()> {
    let db = database::get().await;
    let option = SystemUsers::find_perm()
        .await
        .filter(system_users::Column::Username.eq(username))
        .one(db)
        .await?;
    if option.is_none() {
        return Ok(());
    }
    if id.is_none() {
        return Err(ApiError::biz("用户账号已经存在"));
    }
    if option.unwrap().id != id.unwrap() {
        return Err(ApiError::biz("用户账号已经存在"));
    }
    Ok(())
}

async fn validate_user_exists(id: Option<&str>) -> ApiResult<Option<system_users::Model>> {
    if id.is_none() {
        return Ok(None);
    }
    let model = get_by_id(id.unwrap()).await?;
    Ok(Some(model))
}
pub async fn get_by_username(username: &str) -> ApiResult<Option<system_users::Model>> {
    let db = database::get().await;
    let option = SystemUsers::find_perm()
        .await
        .filter(system_users::Column::Username.eq(username))
        .one(db)
        .await?;
    Ok(option)
}

pub async fn get_by_id(id: &str) -> ApiResult<system_users::Model> {
    let db = database::get().await;
    SystemUsers::find_perm()
        .await
        .filter(system_users::Column::Id.eq(id))
        .one(db)
        .await?
        .ok_or(ApiError::biz("用户不存在"))
}
