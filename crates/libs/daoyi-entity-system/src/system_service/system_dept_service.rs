use crate::system_entity::prelude::*;
use crate::system_entity::system_dept;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::vo::system_vo::DeptListReqVO;
use sea_orm::entity::prelude::*;
use sea_orm::{QueryOrder, QueryTrait};
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

pub async fn get_dept_map<I, S>(ids: I) -> ApiResult<HashMap<String, system_dept::Model>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let map = get_dept_list(ids)
        .await?
        .into_iter()
        .map(|dept| (dept.id.clone(), dept))
        .collect::<HashMap<_, _>>();
    Ok(map)
}

pub async fn get_dept_list<I, S>(ids: I) -> ApiResult<Vec<system_dept::Model>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let ids_vec: Vec<String> = ids.into_iter().map(|s| s.as_ref().to_string()).collect();
    if ids_vec.is_empty() {
        return Ok(vec![]);
    }
    let db = database::get_db_async().await;
    let list = SystemDept::find_perm_with_tenant()
        .await
        .filter(system_dept::Column::Id.is_in(&ids_vec))
        .all(&db)
        .await?;
    Ok(list)
}

pub async fn get_dept_list_by_req(req: &DeptListReqVO) -> ApiResult<Vec<system_dept::Model>> {
    let db = database::get_db_async().await;
    let list = SystemDept::find_perm_with_tenant()
        .await
        .apply_if(req.name.as_deref(), |query, name| {
            query.filter(system_dept::Column::Name.contains(name))
        })
        .apply_if(req.status, |query, status| {
            query.filter(system_dept::Column::Status.eq(status))
        })
        .order_by_asc(system_dept::Column::Sort)
        .all(&db)
        .await?;
    Ok(list)
}
