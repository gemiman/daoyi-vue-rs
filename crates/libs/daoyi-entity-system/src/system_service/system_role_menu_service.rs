use crate::system_entity::prelude::*;
use crate::system_entity::system_role_menu;
use crate::system_service::{system_menu_service, system_role_service};
use daoyi_common_support::database;
use daoyi_common_support::error::ApiResult;
use sea_orm::prelude::*;
use std::collections::HashSet;
use daoyi_macros::transactional;

#[transactional]
pub async fn assign_role_menu(role_id: &str, menu_ids: &Vec<String>) -> ApiResult<()> {
    let db = database::get_db_async().await;

    // 1. 获得角色拥有的菜单编号
    let db_menu_ids: HashSet<String> = SystemRoleMenu::find_perm_with_tenant()
        .await
        .filter(system_role_menu::Column::RoleId.eq(role_id))
        .all(&db)
        .await?
        .into_iter()
        .map(|x| x.menu_id)
        .collect();

    // 2. 将传入的菜单 ID 转为 HashSet
    let menu_id_set: HashSet<String> = menu_ids.iter().cloned().collect();

    // 3. 计算新增的菜单编号（在新集合中但不在旧集合中）
    let create_menu_ids: Vec<String> = menu_id_set.difference(&db_menu_ids).cloned().collect();

    // 4. 计算删除的菜单编号（在旧集合中但不在新集合中）
    let delete_menu_ids: Vec<String> = db_menu_ids.difference(&menu_id_set).cloned().collect();

    // 5. 执行新增操作
    if !create_menu_ids.is_empty() {
        let active_models: Vec<system_role_menu::ActiveModel> = create_menu_ids
            .iter()
            .map(|menu_id| system_role_menu::ActiveModel {
                role_id: sea_orm::Set(role_id.to_string()),
                menu_id: sea_orm::Set(menu_id.clone()),
                ..Default::default()
            })
            .collect();

        SystemRoleMenu::insert_many_auto(&db, active_models).await?;
    }

    // 6. 执行删除操作
    if !delete_menu_ids.is_empty() {
        SystemRoleMenu::delete_many()
            .filter(system_role_menu::Column::RoleId.eq(role_id))
            .filter(system_role_menu::Column::MenuId.is_in(delete_menu_ids))
            .exec(&db)
            .await?;
    }

    Ok(())
}
pub async fn get_role_menu_list_by_role_id(role_ids: &Vec<String>) -> ApiResult<Vec<String>> {
    if role_ids.is_empty() {
        return Ok(vec![]);
    }
    // 如果是管理员的情况下，获取全部菜单编号
    if system_role_service::has_any_super_admin(role_ids).await? {
        return Ok(system_menu_service::get_menu_list(None)
            .await?
            .into_iter()
            .map(|x| x.id)
            .collect());
    }
    let db = database::get_db_async().await;
    Ok(SystemRoleMenu::find_perm_with_tenant()
        .await
        .filter(system_role_menu::Column::RoleId.is_in(role_ids))
        .all(&db)
        .await?
        .into_iter()
        .map(|x| x.menu_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect())
}