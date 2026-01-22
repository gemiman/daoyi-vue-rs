use crate::entity::{prelude::*, {{table.className | snake_case}}};
use crate::entity::{{table.className | snake_case}}::ActiveModel;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::{database, id_util};
use futures::future::try_join_all;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QueryTrait, Set};
// TODO: Import VOs
// use crate::vo::{{ table.businessName | snake_case }}_vo::{ {{ table.businessName | pascal_case }}PageReqVO, {{ table.businessName | pascal_case }}SaveReqVO, {{ table.businessName | pascal_case }}UpdateReqVo };

{% if has_decimal %}use rust_decimal::Decimal;
use serde_json::Value as Json;
{% endif %}
{% if has_json %}{% endif %}

pub async fn create_ { { table.businessName | snake_case } }(vo: {{ table.businessName | pascal_case }}SaveReqVO) -> ApiResult<{{ table.className | snake_case }}::Model> {
let db = database::get_db_async().await;
let active_model: ActiveModel = vo.into();
let model = active_model.insert( &db).await ?;
Ok(model)
}

pub async fn update_ { { table.businessName | snake_case } }(vo: {{ table.businessName | pascal_case }}UpdateReqVo) -> ApiResult<() > {
let db = database::get_db_async().await;
let active_model: ActiveModel = vo.into();
active_model.update( & db).await ?;
Ok(())
}

pub async fn delete_ { { table.businessName | snake_case } }(id: & str) -> ApiResult<() > {
let db = database::get_db_async().await;
{{ table.className }}::delete_logical_by_id( & db, id).await?;
Ok(())
}

pub async fn delete_ { { table.businessName | snake_case } }_list(ids: & Vec<String>) -> ApiResult<() > {
let db = database::get_db_async().await;
{{ table.className }}::delete_logical_by_ids( & db, ids).await?;
Ok(())
}

pub async fn get_ { { table.businessName | snake_case } }(id: & str) -> ApiResult<Option<{{ table.className | snake_case }}::Model>> {
let db = database::get_db_async().await;
Ok({{ table.className }}::find_by_id_perm_with_tenant( & db, id).await? )
}

pub async fn get_ { { table.businessName | snake_case } }_list(
ids: Option< & Vec<String> >,
status: Option<CommonStatusEnum>,
) -> ApiResult<Vec<{{ table.className | snake_case }}::Model>> {
let db = database::get_db_async().await;
let list = {{ table.className }}::find_perm_with_tenant()
.await
.apply_if(ids, | query, ids | {
query.filter({{ table.className | snake_case }}::Column::Id.is_in(ids))
})
.apply_if(status, | query, status| {
query.filter({{ table.className | snake_case }}::Column::Status.eq(status))
})
.all( & db)
.await ?;
Ok(list)
}

pub async fn get_ { { table.businessName | snake_case } }_page(params: &{{ table.businessName | pascal_case }}PageReqVO) -> ApiResult<PageResult<{{ table.className | snake_case }}::Model>> {
let db = database::get_db_async().await;
let paginator = {{ table.className }}::find_perm_with_tenant()
.await
{% for col in columns %}
{% if col.listOperation %}
.apply_if(params.{{ col.javaField }}.as_ref(), | query, val | {
query.filter({{ table.className | snake_case }}::Column::{{ col.javaField | pascal_case }}.eq(val))
})
{% endif %}
{% endfor %}
.order_by_desc({{ table.className | snake_case }}::Column::CreateTime)
.paginate( & db, params.pagination.page_size);

let total = paginator.num_items().await ?;
let list = paginator.fetch_page(params.pagination.page_no - 1).await ?;
Ok(PageResult::from_pagination( & params.pagination, total, list))
}
