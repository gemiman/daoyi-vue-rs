use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::models::pagination::PageParam;
use rust_decimal::Decimal;
use sea_orm::prelude::Date;
use sea_orm::prelude::Time;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use validator::Validate;
{% if has_decimal %}{% endif %}
{% if has_date %}{% endif %}
{% if has_time %}{% endif %}
{% if has_json %}{% endif %}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct {
{
    table.businessName | pascal_case,
}}SaveReqVO {
{% for col in columns %}
{% if col.createOperation and col.columnName != "id" %}
# [validate(required)]
pub {{ col.javaField }}: {{ col.javaType }},
{% endif %}
{% endfor %}
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct {
{
    table.businessName | pascal_case,
}}UpdateReqVo {
#[validate(required)]
pub id: String,
{% for col in columns %}
{% if col.updateOperation and col.columnName != "id" %}
pub {{ col.javaField }}: {{ col.javaType }},
{% endif %}
{% endfor %}
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct {
{
    table.businessName | pascal_case,
}}RespVo {
pub id: String,
{% for col in columns %}
{% if col.columnName != "id" %}
pub {{ col.javaField }}: {{ col.javaType }},
{% endif %}
{% endfor %}
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct {
{
    table.businessName | pascal_case,
}}SimpleRespVo {
pub id: String,
pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct {
{
    table.businessName | pascal_case,
}}PageReqVO {
#[serde(flatten)]
pub pagination: PageParam,
{% for col in columns %}
{% if col.listOperation %}
pub {{ col.javaField }}: Option<{{ col.javaType }}>,
{% endif %}
{% endfor %}
}
