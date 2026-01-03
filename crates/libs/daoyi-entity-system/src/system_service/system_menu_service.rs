use crate::system_entity::prelude::*;
use crate::system_entity::system_menu;
use crate::system_service::system_tenant_service;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::{CommonStatusEnum, MenuTypeEnum, MENU_ID_ROOT};
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::vo::system_vo::{
    MenuListReqVO, MenuSaveVO, MenuUpdateVO, MenuVO,
};
use daoyi_macros::transactional;
use sea_orm::prelude::*;
use sea_orm::QueryTrait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub async fn get_all_menu_list() -> ApiResult<Vec<system_menu::Model>> {
    let db = database::get_db_async().await;
    Ok(SystemMenu::find()
        .filter(system_menu::Column::Deleted.eq(false))
        .all(&db)
        .await?)
}

pub async fn get_menu_list(ids: Option<&Vec<String>>) -> ApiResult<Vec<system_menu::Model>> {
    if ids.is_some() && ids.unwrap().is_empty() {
        return Ok(vec![]);
    }
    let db = database::get_db_async().await;
    Ok(SystemMenu::find()
        .apply_if(ids, |query, ids| {
            query.filter(system_menu::Column::Id.is_in(ids))
        })
        .all(&db)
        .await?)
}

pub async fn get_menu_list_by_req(req: &MenuListReqVO) -> ApiResult<Vec<system_menu::Model>> {
    let db = database::get_db_async().await;
    Ok(SystemMenu::find()
        .apply_if(req.name.as_ref(), |query, name| {
            query.filter(system_menu::Column::Name.contains(name))
        })
        .apply_if(req.status.as_ref(), |query, status| {
            query.filter(system_menu::Column::Status.eq(*status))
        })
        .all(&db)
        .await?)
}

pub async fn create_menu(req: MenuSaveVO) -> ApiResult<String> {
    // 校验父菜单存在
    if req.parent_id.as_str() != MENU_ID_ROOT {
        validate_menu(&req.parent_id).await?;
    }
    let db = database::get_db_async().await;
    let active_model: system_menu::ActiveModel = req.into();
    let model = active_model.insert(&db).await?;
    Ok(model.id)
}

pub async fn update_menu(req: MenuUpdateVO) -> ApiResult<()> {
    if req.parent_id == req.id {
        return Err(ApiError::biz("Parent menu cannot be self"));
    }
    if req.parent_id.as_str() != MENU_ID_ROOT {
        validate_menu(&req.parent_id).await?;
    }

    let db = database::get_db_async().await;
    let active_model: system_menu::ActiveModel = req.into();
    active_model.update(&db).await?;
    Ok(())
}

pub async fn delete_menu(id: &str) -> ApiResult<()> {
    validate_menu(id).await?;
    let db = database::get_db_async().await;
    // 校验是否还有子菜单
    if SystemMenu::find()
        .filter(system_menu::Column::ParentId.eq(id))
        .count(&db)
        .await?
        > 0
    {
        return Err(ApiError::biz("存在子菜单，无法删除"));
    }
    SystemMenu::delete_by_id(id).exec(&db).await?;
    Ok(())
}

#[transactional]
pub async fn delete_menu_list(ids: &Vec<String>) -> ApiResult<()> {
    for id in ids {
        // Simple implementation loop, could be optimized but delete_menu has logic
        delete_menu(id).await?;
    }
    Ok(())
}

pub async fn get_menu(id: &str) -> ApiResult<Option<system_menu::Model>> {
    let db = database::get_db_async().await;
    let option = SystemMenu::find_by_id(id).one(&db).await?;
    Ok(option)
}

async fn validate_menu(id: &str) -> ApiResult<system_menu::Model> {
    if let Some(menu) = get_menu(id).await? {
        return Ok(menu);
    }
    Err(ApiError::biz("菜单不存在"))
}

pub async fn build_menu_tree(menus: Vec<system_menu::Model>) -> ApiResult<Vec<MenuVO>> {
    let mut menus = menus;
    menus.retain(|m| m.r#type != MenuTypeEnum::BUTTON);

    menus.sort_by(|a, b| a.sort.cmp(&b.sort));

    let mut map: HashMap<String, Vec<system_menu::Model>> = HashMap::new();
    for menu in menus {
        map.entry(menu.parent_id.clone())
            .or_insert(vec![])
            .push(menu);
    }

    Ok(build_children(String::from("0"), &map))
}

fn build_children(
    parent_id: String,
    map: &HashMap<String, Vec<system_menu::Model>>,
) -> Vec<MenuVO> {
    if let Some(children) = map.get(&parent_id) {
        children
            .iter()
            .map(|m| {
                let m = m.to_owned();
                MenuVO {
                    id: m.id.clone(),
                    parent_id: m.parent_id,
                    name: m.name,
                    path: m.path,
                    component: m.component,
                    component_name: m.component_name,
                    icon: m.icon,
                    visible: m.visible,
                    keep_alive: m.keep_alive,
                    always_show: m.always_show,
                    children: build_children(m.id, map),
                }
            })
            .collect()
    } else {
        vec![]
    }
}

pub async fn get_menu_list_by_tenant(
    status: Option<CommonStatusEnum>,
) -> ApiResult<Vec<system_menu::Model>> {
    // 查询所有菜单，并过滤掉关闭的节点
    let menus = get_menu_list_by_req(&MenuListReqVO { name: None, status }).await?;
    // 开启多租户的情况下，需要过滤掉未开通的菜单
    let menus_arc = Arc::new(Mutex::new(menus));
    let menus_clone = Arc::clone(&menus_arc);

    system_tenant_service::handle_tenant_menu_async(move |menu_ids| async move {
        let mut menus = menus_clone.lock().unwrap();
        menus.retain(|m| menu_ids.contains(&m.id));
        Ok(())
    })
    .await?;

    let result = Arc::try_unwrap(menus_arc)
        .map_err(|_| ApiError::biz("解包 Arc 失败"))?
        .into_inner()
        .map_err(|_| ApiError::biz("获取互斥锁内部数据失败"))?;

    Ok(result)
}
