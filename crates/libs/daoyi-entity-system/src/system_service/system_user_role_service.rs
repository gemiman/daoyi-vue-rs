use crate::system_entity::prelude::*;
use crate::system_entity::system_user_role;
use daoyi_common_support::database;
use daoyi_common_support::error::ApiResult;
use sea_orm::Set;
use sea_orm::prelude::*;
use std::collections::HashSet;
use daoyi_macros::transactional;

#[transactional]
pub async fn assign_user_role(user_id: &str, role_ids: &Vec<String>) -> ApiResult<()> {
    let db = database::get().await;

    // 1. 获得用户拥有的角色编号
    let db_role_ids: HashSet<String> = SystemUserRole::find_perm()
        .await
        .filter(system_user_role::Column::UserId.eq(user_id))
        .all(&db)
        .await?
        .into_iter()
        .map(|x| x.role_id)
        .collect();

    // 2. 将传入的角色 ID 转为 HashSet
    let role_id_set: HashSet<String> = role_ids.iter().cloned().collect();

    // 3. 计算新增的角色编号（在新集合中但不在旧集合中）
    let create_role_ids: Vec<String> = role_id_set.difference(&db_role_ids).cloned().collect();

    // 4. 计算删除的角色编号（在旧集合中但不在新集合中）
    let delete_role_ids: Vec<String> = db_role_ids.difference(&role_id_set).cloned().collect();

    // 5. 执行新增操作
    if !create_role_ids.is_empty() {
        let active_models: Vec<system_user_role::ActiveModel> = create_role_ids
            .iter()
            .map(|role_id| system_user_role::ActiveModel {
                user_id: Set(user_id.to_string()),
                role_id: Set(role_id.clone()),
                ..Default::default()
            })
            .collect();

        SystemUserRole::insert_many_auto(&db, active_models).await?;
    }

    // 6. 执行删除操作
    if !delete_role_ids.is_empty() {
        SystemUserRole::delete_many()
            .filter(system_user_role::Column::UserId.eq(user_id))
            .filter(system_user_role::Column::RoleId.is_in(delete_role_ids))
            .exec(&db)
            .await?;
    }

    Ok(())
}
pub async fn get_user_role_id_list_by_user_id(user_id: &str) -> ApiResult<Vec<String>> {
    let db = database::get().await;
    let list = SystemUserRole::find_perm()
        .await
        .filter(system_user_role::Column::UserId.eq(user_id))
        .all(db)
        .await?
        .into_iter()
        .map(|item| item.role_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    Ok(list)
}
