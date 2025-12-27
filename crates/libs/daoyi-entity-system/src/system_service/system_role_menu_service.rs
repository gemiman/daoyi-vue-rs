use crate::system_entity::prelude::*;
use crate::system_entity::system_role_menu;
use crate::system_service::{system_menu_service, system_role_service};
use daoyi_common_support::database;
use daoyi_common_support::error::ApiResult;
use sea_orm::prelude::*;
use std::collections::HashSet;

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
    let db = database::get().await;
    Ok(SystemRoleMenu::find_perm()
        .await
        .filter(system_role_menu::Column::RoleId.is_in(role_ids))
        .all(db)
        .await?
        .into_iter()
        .map(|x| x.menu_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect())
}
