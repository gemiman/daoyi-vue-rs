use crate::system_entity::prelude::*;
use crate::system_entity::{system_dict_data, system_dict_type};
use daoyi_common_support::database;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::system_vo::{
    DictDataPageReqVO, DictDataRespVO, DictDataSaveReqVO, DictDataUpdateReqVO,
};
use sea_orm::entity::prelude::*;
use sea_orm::{QueryOrder, QueryTrait, Set};

pub async fn get_dict_data_list(
    status: CommonStatusEnum,
    dict_type: Option<&str>,
) -> ApiResult<Vec<system_dict_data::Model>> {
    let db = database::get_db_async().await;
    let list = SystemDictData::find_perm()
        .await
        .filter(system_dict_data::Column::Status.eq(status))
        .apply_if(dict_type, |query, dict_type| {
            query.filter(system_dict_data::Column::DictType.eq(dict_type))
        })
        .order_by_asc(system_dict_data::Column::DictType)
        .order_by_asc(system_dict_data::Column::Sort)
        .all(&db)
        .await?;
    Ok(list)
}

pub async fn get_dict_data_count_by_dict_type<I, S>(dict_types: I) -> ApiResult<u64>
where
    I: IntoIterator<Item = S>,
    S: Into<Value>,
{
    let db = database::get_db_async().await;
    let count = SystemDictData::find_perm()
        .await
        .filter(system_dict_data::Column::DictType.is_in(dict_types))
        .count(&db)
        .await?;
    Ok(count)
}

async fn validate_dict_type_exists(dict_type: &str) -> ApiResult<()> {
    let model = SystemDictType::find_perm()
        .await
        .filter(system_dict_type::Column::Type.eq(dict_type))
        .one(&database::get_db_async().await)
        .await?
        .ok_or_else(|| ApiError::biz("当前字典类型不存在"))?;
    if model.status != CommonStatusEnum::Enable {
        return Err(ApiError::biz("字典类型不处于开启状态，不允许选择"));
    }
    Ok(())
}
pub async fn get_dict_data(
    dict_type: &str,
    value: &str,
) -> ApiResult<Option<system_dict_data::Model>> {
    let db = database::get_db_async().await;
    let option = SystemDictData::find_perm()
        .await
        .filter(system_dict_data::Column::DictType.eq(dict_type))
        .filter(system_dict_data::Column::Value.eq(value))
        .one(&db)
        .await?;
    Ok(option)
}
async fn validate_dict_data_value_unique(
    id: Option<&str>,
    dict_type: &str,
    dict_value: &str,
) -> ApiResult<()> {
    let dict_data = get_dict_data(dict_type, dict_value).await?;
    if dict_data.is_none() {
        return Ok(());
    }
    if id.is_none() {
        return Err(ApiError::biz("已经存在该值的字典数据"));
    }
    if dict_data.unwrap().id != id.unwrap() {
        return Err(ApiError::biz("已经存在该值的字典数据"));
    }
    Ok(())
}
pub async fn create_dict_data(vo: DictDataSaveReqVO) -> ApiResult<system_dict_data::Model> {
    // 校验字典类型有效
    validate_dict_type_exists(&vo.dict_type).await?;
    // 校验字典数据的值的唯一性
    validate_dict_data_value_unique(None, &vo.dict_type, &vo.value).await?;
    // 插入字典类型
    let active_model: system_dict_data::ActiveModel = vo.into();
    let model = active_model.insert(&database::get_db_async().await).await?;
    Ok(model)
}

pub async fn get_dict_data_by_id(id: &str) -> ApiResult<Option<system_dict_data::Model>> {
    let db = database::get_db_async().await;
    Ok(SystemDictData::find_by_id_perm(&db, id).await?)
}

async fn validate_dict_data_exists(id: &str) -> ApiResult<system_dict_data::Model> {
    get_dict_data_by_id(id)
        .await?
        .ok_or_else(|| ApiError::biz("当前字典数据不存在"))
}

pub async fn update_dict_data(vo: DictDataUpdateReqVO) -> ApiResult<()> {
    // 校验自己存在
    let model = validate_dict_data_exists(&vo.id).await?;
    // 校验字典类型有效
    validate_dict_type_exists(&model.dict_type).await?;
    // 校验字典数据的值的唯一性
    validate_dict_data_value_unique(Some(&vo.id), &model.dict_type, &vo.value).await?;
    // 更新字典类型
    let mut active_model: system_dict_data::ActiveModel = vo.into();
    active_model.dict_type = Set(model.dict_type);
    active_model.update(&database::get_db_async().await).await?;
    Ok(())
}

pub async fn delete_dict_data(id: &str) -> ApiResult<()> {
    // 校验是否存在
    validate_dict_data_exists(&id).await?;
    // 删除字典数据
    SystemDictData::delete_logical_by_id(&database::get_db_async().await, id).await?;
    Ok(())
}

pub async fn delete_dict_data_list(ids: &Vec<String>) -> ApiResult<()> {
    SystemDictData::delete_logical_by_ids(&database::get_db_async().await, ids).await?;
    Ok(())
}

pub async fn get_dict_data_page(
    params: &DictDataPageReqVO,
) -> ApiResult<PageResult<DictDataRespVO>> {
    let db = database::get_db_async().await;
    let paginator = SystemDictData::find_perm()
        .await
        .apply_if(params.label.as_ref(), |query, val| {
            query.filter(system_dict_data::Column::Label.contains(val))
        })
        .apply_if(params.dict_type.as_ref(), |query, val| {
            query.filter(system_dict_data::Column::DictType.eq(val))
        })
        .apply_if(params.status, |query, val| {
            query.filter(system_dict_data::Column::Status.eq(val))
        })
        .order_by_asc(system_dict_data::Column::Sort)
        .order_by_desc(system_dict_data::Column::CreateTime)
        .paginate(&db, params.pagination.page_size);

    let total = paginator.num_items().await?;
    let list = paginator
        .fetch_page(params.pagination.page_no - 1)
        .await?
        .into_iter()
        .map(|m| m.into())
        .collect();
    let page = PageResult::from_pagination(&params.pagination, total, list);
    Ok(page)
}
