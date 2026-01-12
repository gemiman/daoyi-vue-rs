use crate::system_entity::prelude::*;
use crate::system_entity::system_user_post;
use daoyi_common_support::database;
use daoyi_common_support::error::ApiResult;
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use std::collections::HashSet;

pub async fn save_batch(user_id: &str, post_ids: &Vec<String>) -> ApiResult<()> {
    let db = database::get_db_async().await;

    // 1. 获得用户拥有的岗位编号
    let db_post_ids: HashSet<String> = SystemUserPost::find_perm_with_tenant()
        .await
        .filter(system_user_post::Column::UserId.eq(user_id))
        .all(&db)
        .await?
        .into_iter()
        .map(|x| x.post_id)
        .collect();

    // 2. 将传入的岗位 ID 转为 HashSet
    let post_id_set: HashSet<String> = post_ids.iter().cloned().collect();

    // 3. 计算新增的岗位编号（在新集合中但不在旧集合中）
    let create_post_ids: Vec<String> = post_id_set.difference(&db_post_ids).cloned().collect();

    // 4. 计算删除的岗位编号（在旧集合中但不在新集合中）
    let delete_post_ids: Vec<String> = db_post_ids.difference(&post_id_set).cloned().collect();

    // 5. 执行新增操作
    if !create_post_ids.is_empty() {
        let active_models: Vec<system_user_post::ActiveModel> = create_post_ids
            .iter()
            .map(|post_id| system_user_post::ActiveModel {
                user_id: Set(user_id.to_string()),
                post_id: Set(post_id.clone()),
                ..Default::default()
            })
            .collect();

        SystemUserPost::insert_many_auto(&db, active_models).await?;
    }

    // 6. 执行删除操作
    if !delete_post_ids.is_empty() {
        SystemUserPost::delete_many()
            .filter(system_user_post::Column::UserId.eq(user_id))
            .filter(system_user_post::Column::PostId.is_in(delete_post_ids))
            .exec(&db)
            .await?;
    }

    Ok(())
}
