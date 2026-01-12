use crate::system_entity::prelude::*;
use crate::system_entity::system_dict_data;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::error::ApiResult;
use sea_orm::entity::prelude::*;
use sea_orm::{QueryOrder, QueryTrait};

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
