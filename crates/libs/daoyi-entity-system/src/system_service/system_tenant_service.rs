use crate::system_entity::prelude::*;
use crate::system_entity::{system_tenant, system_tenant_package};
use crate::system_service::{
    system_menu_service, system_role_menu_service, system_role_service,
    system_tenant_package_service, system_user_role_service, system_users_service,
};
use daoyi_common_support::configs::AppConfig;
use daoyi_common_support::context::HttpRequestContext;
use daoyi_common_support::enumeration::redis_keys::RedisKey;
use daoyi_common_support::enumeration::{
    CommonStatusEnum, RoleCodeEnum, RoleTypeEnum, PACKAGE_ID_SYSTEM,
};
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::utils::collectors;
use daoyi_common_support::vo::system_vo::TenantPageReqVo;
use daoyi_common_support::vo::system_vo::{
    RoleSaveReqVo, TenantRespVO, TenantSaveReqVo, TenantUpdateReqVo,
};
use daoyi_common_support::{database, redis_utils};
use daoyi_macros::transactional;
use futures::future::try_join_all;
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{IntoActiveModel, QueryOrder, QueryTrait, Set};

pub async fn get_tenant_count_by_package_id(package_id: &str) -> ApiResult<u64> {
    let db = database::get_db_async().await;
    let models = system_tenant::Entity::find()
        .filter(system_tenant::Column::Deleted.eq(false))
        .filter(system_tenant::Column::PackageId.eq(package_id))
        .count(&db)
        .await?;
    Ok(models)
}
pub async fn get_tenant_list_by_package_id(
    package_id: &str,
) -> ApiResult<Vec<system_tenant::Model>> {
    let db = database::get_db_async().await;
    let models = system_tenant::Entity::find()
        .filter(system_tenant::Column::Deleted.eq(false))
        .filter(system_tenant::Column::PackageId.eq(package_id))
        .all(&db)
        .await?;
    Ok(models)
}

#[transactional]
pub async fn update_tenant(vo: TenantUpdateReqVo) -> ApiResult<()> {
    // 校验存在
    let model = validate_update_tenant(&vo.id).await?;
    // 校验租户名称是否重复
    valid_tenant_name_duplicate(&vo.name, Some(&vo.id)).await?;
    // 校验租户域名是否重复
    valid_tenant_website_duplicate(&vo.websites, Some(&vo.id)).await?;
    // 校验套餐被禁用
    let package = system_tenant_package_service::valid_tenant_package(&vo.package_id).await?;
    // 更新租户
    let db = database::get_db_async().await;
    let active_model: system_tenant::ActiveModel = vo.into();
    let res_model = active_model.update(&db).await?;
    // 如果套餐发生变化，则修改其角色的权限
    if model.package_id != res_model.package_id {
        update_tenant_role_menu(&model.id, &package.menu_ids).await?;
    }
    Ok(())
}

#[transactional]
pub async fn update_tenant_role_menu(tenant_id: &str, menu_ids: &Vec<String>) -> ApiResult<()> {
    HttpRequestContext::execute_with_other_context_async(
        HttpRequestContext::builder()
            .tenant_id(tenant_id)
            .ignore_tenant(false)
            .build(),
        async || -> ApiResult<()> {
            let tenant_id = HttpRequestContext::get_tenant_id_as_string().await?;
            // 获得所有角色
            let roles = system_role_service::get_role_list().await?;
            for role in &roles {
                if role.tenant_id != tenant_id {
                    return Err(ApiError::biz(format!(
                        "角色({}/{}) 租户({})不匹配",
                        role.id, role.tenant_id, tenant_id
                    )));
                }
            } // 兜底校验
            // 重新分配每个角色的权限
            for role in roles {
                let role_id = role.id.as_str();
                // 如果是租户管理员，重新分配其权限为租户套餐的权限
                if role.code == RoleCodeEnum::TenantAdmin.code() {
                    system_role_menu_service::assign_role_menu(role_id, menu_ids).await?;
                    tracing::info!(
                        "[update_tenant_role_menu][租户管理员({}/{}) 的权限修改为({:?})]",
                        role_id,
                        role.tenant_id,
                        menu_ids
                    );
                } else {
                    // 如果是其他角色，则去掉超过套餐的权限
                    let role_menu_ids =
                        system_role_menu_service::get_role_menu_list_by_role_id(&vec![
                            String::from(role_id),
                        ])
                        .await?;
                    let role_menu_ids =
                        collectors::intersection_distinct(&role_menu_ids, &menu_ids);
                    system_role_menu_service::assign_role_menu(role_id, &role_menu_ids).await?;
                    tracing::info!(
                        "[update_tenant_role_menu][角色({}/{}) 的权限修改为({:?})]",
                        role_id,
                        role.tenant_id,
                        role_menu_ids
                    );
                }
            }
            Ok(())
        },
    )
    .await?;
    Ok(())
}

async fn validate_update_tenant(id: &str) -> ApiResult<system_tenant::Model> {
    let model = get_tenant_by_id(id).await?;
    if is_system_tenant(&model).await? {
        return Err(ApiError::biz("系统租户不能进行修改、删除等操作！"));
    }
    Ok(model)
}

async fn is_system_tenant(tenant: &system_tenant::Model) -> ApiResult<bool> {
    Ok(tenant.package_id == PACKAGE_ID_SYSTEM)
}

#[transactional]
pub async fn create_tenant(vo: TenantSaveReqVo) -> ApiResult<system_tenant::Model> {
    // 校验租户名称是否重复
    valid_tenant_name_duplicate(&vo.name, None).await?;
    // 校验租户域名是否重复
    valid_tenant_website_duplicate(&vo.websites, None).await?;
    // 校验套餐被禁用
    let package = system_tenant_package_service::valid_tenant_package(&vo.package_id).await?;
    // 创建租户
    let db = database::get_db_async().await;
    let active_model: system_tenant::ActiveModel = vo.clone().into();
    let model = active_model.insert(&db).await?;
    // 创建租户的管理员
    HttpRequestContext::execute_with_other_context_async(
        HttpRequestContext::builder()
            .tenant_id(&model.id)
            .ignore_tenant(false)
            .build(),
        async || -> ApiResult<()> {
            // 创建角色
            let role_id = create_role(&package).await?;
            // 创建用户，并分配角色
            let user_id = create_user(&role_id, &vo).await?;
            // 修改租户的管理员
            let mut active_model = model.clone().into_active_model();
            active_model.contact_user_id = Set(Some(user_id));
            active_model.update(&db).await?;
            Ok(())
        },
    )
    .await?;
    Ok(model)
}

#[transactional]
async fn create_user(role_id: &str, req_vo: &TenantSaveReqVo) -> ApiResult<String> {
    // 创建用户
    let user_id = system_users_service::create_user(req_vo.into()).await?;
    // 分配角色
    system_user_role_service::assign_user_role(&user_id, &vec![String::from(role_id)]).await?;
    Ok(user_id)
}

pub async fn handle_tenant_info_async<F, Fut>(handler: F) -> ApiResult<()>
where
    F: FnOnce(system_tenant::Model) -> Fut,
    Fut: Future<Output = ApiResult<()>>,
{
    // 如果禁用租户功能，则不执行逻辑
    if is_tenant_disable().await {
        return Ok(());
    }
    // 获得租户 ID
    let tenant_id = HttpRequestContext::get_tenant_id_as_string().await?;
    // 获得租户
    let tenant = get_tenant_by_id(&tenant_id).await?;
    // 执行处理器
    handler(tenant).await?;

    Ok(())
}

pub async fn handle_tenant_menu_async<F, Fut>(handler: F) -> ApiResult<()>
where
    F: FnOnce(Vec<String>) -> Fut,
    Fut: Future<Output = ApiResult<()>>,
{
    // 如果禁用租户功能，则不执行逻辑
    if is_tenant_disable().await {
        return Ok(());
    }
    // 获得租户 ID
    let tenant_id = HttpRequestContext::get_tenant_id_as_string().await?;
    // 获得租户
    let tenant = get_tenant_by_id(&tenant_id).await?;
    let menu_ids = if is_system_tenant(&tenant).await? {
        // 系统租户，菜单是全量的
        system_menu_service::get_all_menu_list()
            .await?
            .into_iter()
            .map(|m| m.id)
            .collect()
    } else {
        system_tenant_package_service::valid_tenant_package(&tenant.package_id)
            .await?
            .menu_ids
    };
    // 执行处理器
    handler(menu_ids).await?;
    Ok(())
}

async fn is_tenant_disable() -> bool {
    !AppConfig::get().await.auth().tenant_enable()
}

#[transactional]
async fn create_role(tenant_package: &system_tenant_package::Model) -> ApiResult<String> {
    // 创建角色
    let req_vo = RoleSaveReqVo {
        code: String::from(RoleCodeEnum::TenantAdmin.code()),
        id: None,
        name: String::from(RoleCodeEnum::TenantAdmin.name()),
        remark: Some(String::from("系统自动生成")),
        sort: 0,
        status: CommonStatusEnum::Enable,
    };
    let role_id = system_role_service::create_role(req_vo, Some(RoleTypeEnum::SYSTEM)).await?;
    // 分配权限
    system_role_menu_service::assign_role_menu(&role_id, &tenant_package.menu_ids).await?;
    Ok(role_id)
}

async fn valid_tenant_website_duplicate(
    websites: &Option<Vec<String>>,
    id: Option<&str>,
) -> ApiResult<()> {
    if websites.is_none() || websites.as_ref().unwrap().is_empty() {
        return Ok(());
    }
    let websites = websites.as_ref().unwrap();
    for website in websites {
        if let Ok(tenant) = get_tenant_by_website(website).await {
            if id.is_none() {
                return Err(ApiError::biz(format!("域名为【{}】的租户已存在", website)));
            }
            if id.is_some() && id != Some(tenant.id.as_str()) {
                return Err(ApiError::biz(format!("域名为【{}】的租户已存在", website)));
            }
        }
    }
    Ok(())
}

async fn valid_tenant_name_duplicate(name: &str, id: Option<&str>) -> ApiResult<()> {
    if let Ok(tenant) = get_tenant_by_name(name).await {
        if id.is_none() {
            return Err(ApiError::biz(format!("名字为【{}】的租户已存在", name)));
        }
        if id.is_some() && id != Some(tenant.id.as_str()) {
            return Err(ApiError::biz(format!("名字为【{}】的租户已存在", name)));
        }
    }
    Ok(())
}

pub async fn get_tenant_list_by_status(
    status: Option<CommonStatusEnum>,
) -> ApiResult<Vec<system_tenant::Model>> {
    let db = database::get_db_async().await;
    let list = SystemTenant::find()
        .filter(system_tenant::Column::Deleted.eq(false))
        .apply_if(status, |query, status| {
            query.filter(system_tenant::Column::Status.eq(status))
        })
        .all(&db)
        .await?;
    Ok(list)
}
#[transactional]
pub async fn get_tenant_by_id(tenant_id: &str) -> ApiResult<system_tenant::Model> {
    let db = database::get_db_async().await;
    let model = SystemTenant::find_by_id(tenant_id)
        .filter(system_tenant::Column::Deleted.eq(false))
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::biz("租户不存在"))?;
    Ok(model)
}

#[transactional]
pub async fn get_tenant_by_name(name: &str) -> ApiResult<system_tenant::Model> {
    let db = database::get_db_async().await;
    let option = SystemTenant::find()
        .filter(system_tenant::Column::Deleted.eq(false))
        .filter(system_tenant::Column::Name.eq(name))
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::biz("租户不存在"))?;
    Ok(option)
}

#[transactional]
pub async fn get_tenant_by_website(website: &str) -> ApiResult<system_tenant::Model> {
    let db = database::get_db_async().await;
    let tenant = SystemTenant::find()
        .filter(system_tenant::Column::Deleted.eq(false))
        .filter(system_tenant::Column::Status.eq(CommonStatusEnum::Enable))
        .filter(Expr::cust_with_values("$1 = ANY(websites)", [website]))
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::biz("租户不存在"))?;
    Ok(tenant)
}

pub async fn check_tenant_id(tenant_id: &str) -> ApiResult<TenantRespVO> {
    let redis_key = RedisKey::CheckTenantId.key(tenant_id);
    // 1. Try to get from Redis
    if let Some(vo) = redis_utils::cache_get_json::<TenantRespVO>(&redis_key).await? {
        return Ok(vo);
    }
    let model = get_tenant_by_id(tenant_id).await?;
    if model.status == CommonStatusEnum::Disable {
        return Err(ApiError::unauthenticated("租户被禁用"));
    }
    let vo: TenantRespVO = model.into();
    let now = Local::now().naive_local();
    let duration = vo.expire_time - now;
    let ttl = duration.num_seconds();
    if ttl > 0 {
        redis_utils::cache_set_json_ex(&redis_key, &vo, ttl as u64).await?;
    } else {
        return Err(ApiError::unauthenticated("租户过期"));
    }
    Ok(vo)
}

pub async fn get_tenant_page(params: &TenantPageReqVo) -> ApiResult<Page<system_tenant::Model>> {
    let db = database::get_db_async().await;
    let paginator = SystemTenant::find()
        .filter(system_tenant::Column::Deleted.eq(false))
        .apply_if(params.contact_mobile.as_ref(), |query, contact_mobile| {
            query.filter(system_tenant::Column::ContactMobile.contains(contact_mobile))
        })
        .apply_if(params.contact_name.as_ref(), |query, contact_name| {
            query.filter(system_tenant::Column::ContactName.contains(contact_name))
        })
        .apply_if(params.create_time.as_ref(), |query, create_time| {
            query.filter(system_tenant::Column::CreateTime.between(create_time[0], create_time[1]))
        })
        .apply_if(params.name.as_ref(), |query, name| {
            query.filter(system_tenant::Column::Name.contains(name))
        })
        .apply_if(params.status, |query, status| {
            query.filter(system_tenant::Column::Status.eq(status))
        })
        .order_by_desc(system_tenant::Column::CreateTime)
        .paginate(&db, params.pagination.page_size);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(params.pagination.page_no - 1).await?;
    let page = Page::from_pagination(&params.pagination, total, list);
    Ok(page)
}

pub async fn delete_tenant_list(ids: &Vec<String>) -> ApiResult<()> {
    if ids.is_empty() {
        return Ok(());
    }
    // 校验存在
    try_join_all(ids.iter().map(|id| validate_update_tenant(id))).await?;
    // 删除
    let db = database::get_db_async().await;
    SystemTenant::update_many_auto().await
        .col_expr(system_tenant::Column::Deleted, Expr::value(true))
        .filter(system_tenant::Column::Id.is_in(ids))
        .exec(&db)
        .await?;
    Ok(())
}

pub async fn delete_tenant(id: &str) -> ApiResult<()> {
    // 校验存在
    validate_update_tenant(id).await?;
    // 删除
    let db = database::get_db_async().await;
    SystemTenant::update_many_auto().await
        .col_expr(system_tenant::Column::Deleted, Expr::value(true))
        .filter(system_tenant::Column::Id.eq(id))
        .exec(&db)
        .await?;
    Ok(())
}
