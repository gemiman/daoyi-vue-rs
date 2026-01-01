use crate::system_entity::prelude::*;
use crate::system_entity::{system_tenant, system_tenant_package};
use crate::system_service::{
    system_role_menu_service, system_role_service, system_tenant_package_service,
    system_user_role_service, system_users_service,
};
use daoyi_common_support::configs::AppConfig;
use daoyi_common_support::context::HttpRequestContext;
use daoyi_common_support::enumeration::redis_keys::RedisKey;
use daoyi_common_support::enumeration::{CommonStatusEnum, RoleCodeEnum, RoleTypeEnum};
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::models::system::TenantPageReqVo;
use daoyi_common_support::vo::system_vo::{RoleSaveReqVo, TenantRespVO, TenantSaveReqVo};
use daoyi_common_support::{database, redis_utils};
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{IntoActiveModel, QueryOrder, QueryTrait, Set};
use daoyi_macros::transactional;

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