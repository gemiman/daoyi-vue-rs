use crate::system_entity::prelude::*;
use crate::system_entity::system_dept;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::{CommonStatusEnum, ID_ROOT};
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::vo::system_vo::{DeptListReqVO, DeptSaveReqVO, DeptUpdateReqVO};
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

async fn validate_parent_dept(id: Option<&str>, parent_id: Option<&str>) -> ApiResult<()> {
    if parent_id.is_none() || parent_id == Some(ID_ROOT) {
        return Ok(());
    }
    // 1. 不能设置自己为父部门
    if id == parent_id {
        return Err(ApiError::biz("不能设置自己为父部门"));
    }
    // 2. 父部门不存在
    let mut parent_dept = validate_dept_exists(parent_id.unwrap())
        .await
        .map_err(|_| ApiError::biz("父级部门不存在"))?;
    // 3. 递归校验父部门，如果父部门是自己的子部门，则报错，避免形成环路
    if id.is_none() {
        // id 为空，说明新增，不需要考虑环路
        return Ok(());
    }
    let id = id.unwrap();
    loop {
        // 3.1 校验环路
        let parent_id = parent_dept.parent_id.as_str();
        if id == parent_id {
            break Err(ApiError::biz("不能设置自己的子部门为父部门"));
        }
        // 3.2 继续递归下一级父部门
        if parent_id == ID_ROOT {
            break Ok(());
        }
        parent_dept = validate_dept_exists(parent_id)
            .await
            .map_err(|_| ApiError::biz("父级部门不存在"))?;
    }
}

async fn validate_dept_name_unique(
    id: Option<&str>,
    parent_id: Option<&str>,
    name: &str,
) -> ApiResult<()> {
    let db = database::get_db_async().await;
    let dept = SystemDept::find_perm_with_tenant()
        .await
        .filter(system_dept::Column::Name.eq(name))
        .filter(system_dept::Column::Id.eq(parent_id.unwrap_or(ID_ROOT)))
        .one(&db)
        .await?;
    if dept.is_none() {
        return Ok(());
    }
    if let Some(dept) = dept {
        if id.is_none() || id.unwrap() != dept.id {
            return Err(ApiError::biz("已经存在该名字的部门"));
        }
    }
    Ok(())
}

pub async fn get_dept(id: &str) -> ApiResult<Option<system_dept::Model>> {
    Ok(SystemDept::find_by_id_perm_with_tenant(&database::get_db_async().await, id).await?)
}

async fn validate_dept_exists(id: &str) -> ApiResult<system_dept::Model> {
    let opt = get_dept(id).await?;
    if let Some(dept) = opt {
        Ok(dept)
    } else {
        Err(ApiError::biz("当前部门不存在"))
    }
}

pub async fn create_dept(vo: DeptSaveReqVO) -> ApiResult<system_dept::Model> {
    // 校验父部门的有效性
    validate_parent_dept(None, vo.parent_id.as_deref()).await?;
    // 校验部门名的唯一性
    validate_dept_name_unique(None, vo.parent_id.as_deref(), vo.name.as_str()).await?;
    // 插入部门
    let active_model: system_dept::ActiveModel = vo.into();
    let model = active_model.insert(&database::get_db_async().await).await?;
    Ok(model)
}

pub async fn update_dept(vo: DeptUpdateReqVO) -> ApiResult<()> {
    // 校验自己存在
    validate_dept_exists(vo.id.as_str()).await?;
    // 校验父部门的有效性
    validate_parent_dept(Some(vo.id.as_str()), vo.parent_id.as_deref()).await?;
    // 校验部门名的唯一性
    validate_dept_name_unique(
        Some(vo.id.as_str()),
        vo.parent_id.as_deref(),
        vo.name.as_str(),
    )
    .await?;
    // 更新部门
    let active_model: system_dept::ActiveModel = vo.into();
    active_model.update(&database::get_db_async().await).await?;
    Ok(())
}
