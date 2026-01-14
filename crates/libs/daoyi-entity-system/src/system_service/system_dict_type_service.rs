use crate::system_entity::prelude::*;
use crate::system_entity::system_dict_type;
use crate::system_service::system_dict_data_service;
use daoyi_common_support::database;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::system_vo::{
    DictTypePageReqVO, DictTypeRespVO, DictTypeSaveReqVO, DictTypeUpdateReqVO,
};
use daoyi_macros::transactional;
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{QueryOrder, QueryTrait, Unchanged};

pub async fn get_dict_type_page(
    params: &DictTypePageReqVO,
) -> ApiResult<PageResult<DictTypeRespVO>> {
    let db = database::get_db_async().await;
    let paginator = SystemDictType::find_perm()
        .await
        .apply_if(params.name.as_ref(), |query, val| {
            query.filter(system_dict_type::Column::Name.contains(val))
        })
        .apply_if(params.r#type.as_ref(), |query, val| {
            query.filter(system_dict_type::Column::Type.eq(val))
        })
        .apply_if(params.status, |query, val| {
            query.filter(system_dict_type::Column::Status.eq(val))
        })
        .apply_if(params.create_time.as_ref(), |query, val| {
            query.filter(system_dict_type::Column::CreateTime.between(val[0], val[1]))
        })
        .order_by_desc(system_dict_type::Column::CreateTime)
        .paginate(&db, params.pagination.page_size);

    let total = paginator.num_items().await?;
    let list = paginator
        .fetch_page(params.pagination.page_no - 1)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    let page = PageResult::from_pagination(&params.pagination, total, list);
    Ok(page)
}

pub async fn get_dict_type(id: &str) -> ApiResult<Option<system_dict_type::Model>> {
    let db = database::get_db_async().await;
    let model = SystemDictType::find_by_id_perm(&db, id).await?;
    Ok(model)
}

async fn validate_dict_type_name_unique(id: Option<&str>, name: &str) -> ApiResult<()> {
    let db = database::get_db_async().await;
    let option = SystemDictType::find_perm()
        .await
        .filter(system_dict_type::Column::Name.eq(name))
        .one(&db)
        .await?;
    if option.is_none() {
        return Ok(());
    }
    if id.is_none() {
        return Err(ApiError::biz("已经存在该名字的字典类型"));
    }
    if option.unwrap().id != id.unwrap() {
        return Err(ApiError::biz("已经存在该名字的字典类型"));
    }
    Ok(())
}

async fn validate_dict_type_unique(id: Option<&str>, dict_type: &str) -> ApiResult<()> {
    let db = database::get_db_async().await;
    let option = SystemDictType::find_perm()
        .await
        .filter(system_dict_type::Column::Type.eq(dict_type))
        .one(&db)
        .await?;
    if option.is_none() {
        return Ok(());
    }
    if id.is_none() {
        return Err(ApiError::biz("已经存在该类型的字典类型"));
    }
    if option.unwrap().id != id.unwrap() {
        return Err(ApiError::biz("已经存在该类型的字典类型"));
    }
    Ok(())
}

pub async fn create_dict_type(vo: DictTypeSaveReqVO) -> ApiResult<system_dict_type::Model> {
    // 校验字典类型的名字的唯一性
    validate_dict_type_name_unique(None, &vo.name).await?;
    // 校验字典类型的类型的唯一性
    validate_dict_type_unique(None, &vo.r#type).await?;
    // 插入字典类型
    let active_model: system_dict_type::ActiveModel = vo.into();
    let model = active_model.insert(&database::get_db_async().await).await?;
    Ok(model)
}

async fn validate_dict_type_exists(id: &str) -> ApiResult<system_dict_type::Model> {
    get_dict_type(id)
        .await?
        .ok_or_else(|| ApiError::biz("当前字典类型不存在"))
}
pub async fn update_dict_type(vo: DictTypeUpdateReqVO) -> ApiResult<()> {
    // 校验自己存在
    let model = validate_dict_type_exists(&vo.id).await?;
    // 校验字典类型的名字的唯一性
    validate_dict_type_name_unique(Some(&vo.id), &vo.name).await?;
    // 校验字典类型的类型的唯一性
    validate_dict_type_unique(Some(&vo.id), &vo.r#type).await?;
    // 更新字典类型
    let mut active_model: system_dict_type::ActiveModel = vo.into();
    active_model.r#type = Unchanged(model.r#type); // 不允许修改字典类型
    active_model.update(&database::get_db_async().await).await?;
    Ok(())
}

#[transactional]
pub async fn delete_dict_type_list(ids: &Vec<String>) -> ApiResult<()> {
    let db = database::get_db_async().await;
    let dict_types = SystemDictType::find_by_ids_perm(&db, ids).await?;
    if dict_types.len() != ids.len() {
        return Err(ApiError::biz("部分字典类型不存在"));
    }
    let dict_types: Vec<String> = dict_types.into_iter().map(|m| m.r#type).collect();
    // 校验是否有字典数据
    if system_dict_data_service::get_dict_data_count_by_dict_type(dict_types).await? > 0 {
        return Err(ApiError::biz("无法删除，该字典类型还有字典数据"));
    }
    // 删除字典类型
    SystemDictType::update_many_auto()
        .await
        .filter(system_dict_type::Column::Type.is_in(ids))
        .col_expr(
            system_dict_type::Column::DeletedTime,
            Expr::value(Local::now().naive_local()),
        )
        .exec(&db)
        .await?;
    SystemDictType::delete_logical_by_ids(&db, ids).await?;
    Ok(())
}

#[transactional]
pub async fn delete_dict_type(id: &str) -> ApiResult<()> {
    // 校验是否存在
    let dict_type = validate_dict_type_exists(id).await?.r#type;
    // 校验是否有字典数据
    if system_dict_data_service::get_dict_data_count_by_dict_type(vec![dict_type]).await? > 0 {
        return Err(ApiError::biz("无法删除，该字典类型还有字典数据"));
    }
    // 删除字典类型
    SystemDictType::update_many_auto()
        .await
        .filter(system_dict_type::Column::Type.eq(id))
        .col_expr(
            system_dict_type::Column::DeletedTime,
            Expr::value(Local::now().naive_local()),
        )
        .exec(&database::get_db_async().await)
        .await?;
    SystemDictType::delete_logical_by_id(&database::get_db_async().await, id).await?;
    Ok(())
}

pub async fn get_dict_type_list() -> ApiResult<Vec<system_dict_type::Model>> {
    let db = database::get_db_async().await;
    let list = SystemDictType::find_perm().await.all(&db).await?;
    Ok(list)
}
