use crate::system_entity::prelude::*;
use crate::system_entity::system_post;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::vo::system_vo::{PostPageReqVO, PostSaveReqVO, PostUpdateReqVo};
use futures::future::try_join_all;
use sea_orm::entity::prelude::*;
use sea_orm::{QueryOrder, QueryTrait};
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
    let map = get_post_list(Some(ids), None)
        .await?
        .into_iter()
        .map(|post| (post.id.clone(), post))
        .collect::<HashMap<_, _>>();
    Ok(map)
}
pub async fn get_post_list(
    ids: Option<&Vec<String>>,
    status: Option<CommonStatusEnum>,
) -> ApiResult<Vec<system_post::Model>> {
    if let Some(ids) = ids
        && ids.is_empty()
    {
        return Ok(vec![]);
    }
    let db = database::get_db_async().await;
    let list = SystemPost::find_perm_with_tenant()
        .await
        .apply_if(ids, |query, ids| {
            query.filter(system_post::Column::Id.is_in(ids))
        })
        .apply_if(status, |query, status| {
            query.filter(system_post::Column::Status.eq(status))
        })
        .all(&db)
        .await?;
    Ok(list)
}

pub async fn get_post_page(params: &PostPageReqVO) -> ApiResult<Page<system_post::Model>> {
    let db = database::get_db_async().await;
    let paginator = SystemPost::find_perm_with_tenant()
        .await
        .apply_if(params.code.as_ref(), |query, code| {
            query.filter(system_post::Column::Code.contains(code))
        })
        .apply_if(params.name.as_ref(), |query, name| {
            query.filter(system_post::Column::Name.contains(name))
        })
        .apply_if(params.status, |query, status| {
            query.filter(system_post::Column::Status.eq(status))
        })
        .order_by_desc(system_post::Column::CreateTime)
        .paginate(&db, params.pagination.page_size);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(params.pagination.page_no - 1).await?;
    let page = Page::from_pagination(&params.pagination, total, list);
    Ok(page)
}

pub async fn get_post(id: &str) -> ApiResult<Option<system_post::Model>> {
    Ok(SystemPost::find_by_id_perm_with_tenant(&database::get_db_async().await, id).await?)
}

pub async fn delete_post(id: &str) -> ApiResult<()> {
    // 校验是否存在
    validate_post_exists(Some(id)).await?;
    // 删除岗位
    let db = database::get_db_async().await;
    SystemPost::delete_logical_by_id(&db, id).await?;
    Ok(())
}

pub async fn delete_post_list(ids: &Vec<String>) -> ApiResult<()> {
    // 校验存在
    try_join_all(ids.iter().map(|id| validate_post_exists(Some(id)))).await?;
    let db = database::get_db_async().await;
    SystemPost::delete_logical_by_ids(&db, ids).await?;
    Ok(())
}

async fn validate_post_exists(id: Option<&str>) -> ApiResult<()> {
    if let Some(id) = id
        && None == get_post(id).await?
    {
        return Err(ApiError::biz("当前岗位不存在"));
    }
    Ok(())
}

async fn validate_post_name_unique(id: Option<&str>, name: &str) -> ApiResult<()> {
    let db = database::get_db_async().await;
    let option = SystemPost::find_perm_with_tenant()
        .await
        .filter(system_post::Column::Name.eq(name))
        .one(&db)
        .await?;
    if let Some(post) = option {
        if id.is_none() || id != Some(&post.id) {
            return Err(ApiError::biz("已经存在该名字的岗位"));
        }
    }
    Ok(())
}

async fn validate_post_code_unique(id: Option<&str>, code: &str) -> ApiResult<()> {
    let db = database::get_db_async().await;
    let option = SystemPost::find_perm_with_tenant()
        .await
        .filter(system_post::Column::Code.eq(code))
        .one(&db)
        .await?;
    if let Some(post) = option {
        if id.is_none() || id != Some(&post.id) {
            return Err(ApiError::biz("已经存在该标识的岗位"));
        }
    }
    Ok(())
}

async fn validate_post_for_create_or_update(
    id: Option<&str>,
    name: &str,
    code: &str,
) -> ApiResult<()> {
    // 校验自己存在
    validate_post_exists(id).await?;
    // 校验岗位名的唯一性
    validate_post_name_unique(id, name).await?;
    // 校验岗位编码的唯一性
    validate_post_code_unique(id, code).await?;
    Ok(())
}

pub async fn create_post(vo: PostSaveReqVO) -> ApiResult<system_post::Model> {
    // 校验正确性
    validate_post_for_create_or_update(None, &vo.name, &vo.code).await?;
    // 插入岗位
    let db = database::get_db_async().await;
    let active_model: system_post::ActiveModel = vo.into();
    let model = active_model.insert(&db).await?;
    Ok(model)
}

pub async fn update_post(vo: PostUpdateReqVo) -> ApiResult<()> {
    // 校验正确性
    validate_post_for_create_or_update(Some(&vo.id), &vo.name, &vo.code).await?;
    let db = database::get_db_async().await;
    let active_model: system_post::ActiveModel = vo.into();
    active_model.update(&db).await?;
    Ok(())
}
