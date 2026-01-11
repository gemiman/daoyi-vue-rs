use crate::system_entity::prelude::*;
use crate::system_entity::system_users;
use crate::system_service::{
    system_dept_service, system_post_service, system_tenant_service, system_user_post_service,
    system_user_role_service,
};
use daoyi_common_support::database;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::password::hash_password;
use daoyi_common_support::vo::system_vo::{
    UserPageReqVO, UserRespVO, UserSaveReqVO, UserUpdatePasswordReqVo, UserUpdateReqVO,
};
use daoyi_macros::transactional;
use sea_orm::entity::prelude::*;
use sea_orm::{IntoActiveModel, QueryOrder, QueryTrait, Set, Unchanged};
use std::collections::HashSet;

pub async fn update_user_password(vo: UserUpdatePasswordReqVo) -> ApiResult<()> {
    // 1. 校验用户存在
    let mut active_model = validate_user_exists(Some(&vo.id))
        .await?
        .unwrap()
        .into_active_model();
    // 2. 更新密码
    active_model.password = Set(hash_password(&vo.password).await?);
    active_model.update(&database::get_db_async().await).await?;
    Ok(())
}

#[transactional]
pub async fn update_user(vo: UserUpdateReqVO) -> ApiResult<()> {
    // 1. 校验正确性
    let old_user = validate_user_for_create_or_update(
        Some(&vo.id),
        &vo.username,
        vo.mobile.as_deref(),
        vo.email.as_deref(),
        vo.dept_id.as_deref(),
        vo.post_ids.as_ref(),
    )
    .await?
    .unwrap();
    // 2.1 更新用户
    let mut active_model: system_users::ActiveModel = vo.into();
    active_model.password = Unchanged(old_user.password);
    let db = database::get_db_async().await;
    let model = active_model.update(&db).await?;
    // 2.2 更新岗位
    let post_ids = &model.post_ids.unwrap_or_default();
    system_user_post_service::save_batch(&model.id, post_ids).await?;
    Ok(())
}

pub async fn create_user(req_vo: UserSaveReqVO) -> ApiResult<system_users::Model> {
    // 1.1 校验账户配合
    system_tenant_service::handle_tenant_info_async(async |tenant| {
        let db = database::get_db_async().await;
        let count = SystemUsers::find_perm_with_tenant()
            .await
            .count(&db)
            .await? as i32;
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
    let db = database::get_db_async().await;
    let mut active_model: system_users::ActiveModel = req_vo.into();
    active_model.status = Set(CommonStatusEnum::Enable);
    let model = active_model.insert(&db).await?;
    // 2.2 插入关联岗位
    if let Some(post_ids) = &model.post_ids
        && !post_ids.is_empty()
    {
        system_user_post_service::save_batch(&model.id, post_ids).await?;
    }
    Ok(model)
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
    if let Some(post_ids) = post_ids {
        system_post_service::validate_post_list(post_ids).await?;
    }
    Ok(option)
}

async fn validate_email_unique(id: Option<&str>, email: Option<&str>) -> ApiResult<()> {
    if email.is_none() {
        return Ok(());
    }
    let mobile = email.unwrap();
    let db = database::get_db_async().await;
    let option = SystemUsers::find_perm_with_tenant()
        .await
        .filter(system_users::Column::Email.eq(mobile))
        .one(&db)
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
    let db = database::get_db_async().await;
    let option = SystemUsers::find_perm_with_tenant()
        .await
        .filter(system_users::Column::Mobile.eq(mobile))
        .one(&db)
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
    let db = database::get_db_async().await;
    let option = SystemUsers::find_perm_with_tenant()
        .await
        .filter(system_users::Column::Username.eq(username))
        .one(&db)
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
    let db = database::get_db_async().await;
    let option = SystemUsers::find_perm_with_tenant()
        .await
        .filter(system_users::Column::Username.eq(username))
        .one(&db)
        .await?;
    Ok(option)
}

pub async fn get_by_id(id: &str) -> ApiResult<system_users::Model> {
    let db = database::get_db_async().await;
    SystemUsers::find_by_id_perm_with_tenant(&db, id)
        .await?
        .ok_or(ApiError::biz("用户不存在"))
}

pub async fn get_user_list_by_status(
    status: CommonStatusEnum,
) -> ApiResult<Vec<system_users::Model>> {
    let db = database::get_db_async().await;
    let list = SystemUsers::find_perm_with_tenant()
        .await
        .filter(system_users::Column::Status.eq(status))
        .all(&db)
        .await?;
    Ok(list)
}

async fn get_dept_condition(dept_id: Option<&str>) -> ApiResult<Option<Vec<String>>> {
    if dept_id.is_none() {
        return Ok(None);
    }
    let dept_id = String::from(dept_id.unwrap());
    let mut dept_ids = system_dept_service::get_child_dept_list(&dept_id)
        .await?
        .into_iter()
        .map(|dept| dept.id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if !dept_ids.contains(&dept_id) {
        dept_ids.push(dept_id); // 包括自身
    }
    Ok(Some(dept_ids))
}

pub async fn get_user_page(params: &UserPageReqVO) -> ApiResult<Page<UserRespVO>> {
    let db = database::get_db_async().await;
    // 获得用户分页列表
    // 如果有角色编号，查询角色对应的用户编号
    let user_ids = if let Some(role_id) = &params.role_id {
        let user_ids = system_user_role_service::get_user_role_id_list_by_role_id(&role_id).await?;
        Some(user_ids)
    } else {
        None
    };
    if let Some(user_ids) = &user_ids
        && user_ids.is_empty()
    {
        return Ok(Page::empty(&params.pagination));
    }
    let dept_ids = get_dept_condition(params.dept_id.as_deref()).await?;
    // 分页查询
    let paginator = SystemUsers::find_perm_with_tenant()
        .await
        .apply_if(params.username.as_ref(), |query, username| {
            query.filter(system_users::Column::Username.contains(username))
        })
        .apply_if(params.mobile.as_ref(), |query, mobile| {
            query.filter(system_users::Column::Mobile.contains(mobile))
        })
        .apply_if(params.status, |query, status| {
            query.filter(system_users::Column::Status.eq(status))
        })
        .apply_if(params.create_time.as_ref(), |query, create_time| {
            query.filter(system_users::Column::CreateTime.between(create_time[0], create_time[1]))
        })
        .apply_if(dept_ids, |query, dept_ids| {
            query.filter(system_users::Column::DeptId.is_in(dept_ids))
        })
        .apply_if(user_ids, |query, user_ids| {
            query.filter(system_users::Column::Id.is_in(user_ids))
        })
        .order_by_desc(system_users::Column::CreateTime)
        .order_by_desc(system_users::Column::Id)
        .paginate(&db, params.pagination.page_size);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(params.pagination.page_no - 1).await?;
    if list.is_empty() {
        return Ok(Page::empty(&params.pagination));
    }
    let dept_ids = list
        .iter()
        .filter(|x| x.dept_id.is_some())
        .map(|x| x.dept_id.as_deref().unwrap())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let dept_map = system_dept_service::get_dept_map(dept_ids).await?;
    let list = list
        .into_iter()
        .map(|u| {
            let dept_name = u
                .dept_id
                .as_ref()
                .and_then(|dept_id| dept_map.get(dept_id).map(|d| d.name.clone()));
            u.convert_vo(dept_name)
        })
        .collect();
    // 拼接数据
    let page = Page::from_pagination(&params.pagination, total, list);
    Ok(page)
}
