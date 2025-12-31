use crate::system_entity::prelude::*;
use crate::system_entity::system_dept;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use sea_orm::entity::prelude::*;
use std::collections::HashMap;

pub async fn validate_dept_list(ids: &Vec<String>) -> ApiResult<()> {
    if ids.is_empty() {
        return Ok(());
    }
    let map = get_dept_map(ids).await?;
    for id in ids {
        if let Some(dept) = map.get(id) {
            if CommonStatusEnum::Enable != dept.status {
                return Err(ApiError::biz(format!(
                    "部门({})不处于开启状态，不允许选择",
                    dept.name
                )));
            }
        } else {
            return Err(ApiError::biz("当前部门不存在"));
        }
    }
    Ok(())
}

pub async fn get_dept_map(ids: &Vec<String>) -> ApiResult<HashMap<String, system_dept::Model>> {
    let map = get_dept_list(ids)
        .await?
        .into_iter()
        .map(|dept| (dept.id.clone(), dept))
        .collect::<HashMap<_, _>>();
    Ok(map)
}
pub async fn get_dept_list(ids: &Vec<String>) -> ApiResult<Vec<system_dept::Model>> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    let db = database::get().await;
    let list = SystemDept::find_perm()
        .await
        .filter(system_dept::Column::Id.is_in(ids))
        .all(db)
        .await?;
    Ok(list)
}
