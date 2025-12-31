use crate::system_entity::prelude::*;
use crate::system_entity::system_post;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use sea_orm::entity::prelude::*;
use std::collections::HashMap;

pub async fn validate_post_list(ids: &Vec<String>) -> ApiResult<()> {
    if ids.is_empty() {
        return Ok(());
    }
    let map = get_post_map(ids).await?;
    for id in ids {
        if let Some(post) = map.get(id) {
            if CommonStatusEnum::Enable != post.status {
                return Err(ApiError::biz(format!(
                    "岗位({})不处于开启状态，不允许选择",
                    post.name
                )));
            }
        } else {
            return Err(ApiError::biz("当前岗位不存在"));
        }
    }
    Ok(())
}

pub async fn get_post_map(ids: &Vec<String>) -> ApiResult<HashMap<String, system_post::Model>> {
    let map = get_post_list(ids)
        .await?
        .into_iter()
        .map(|post| (post.id.clone(), post))
        .collect::<HashMap<_, _>>();
    Ok(map)
}
pub async fn get_post_list(ids: &Vec<String>) -> ApiResult<Vec<system_post::Model>> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    let db = database::get().await;
    let list = SystemPost::find_perm()
        .await
        .filter(system_post::Column::Id.is_in(ids))
        .all(db)
        .await?;
    Ok(list)
}
