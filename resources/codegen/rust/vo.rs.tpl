use daoyi_common_support::models::pagination::PageParam;
use daoyi_common_support::enumeration::CommonStatusEnum;
use serde::{Deserialize, Serialize};
use validator::Validate;
{% if has_decimal %}use rust_decimal::Decimal;{% endif %}
{% if has_date %}use sea_orm::prelude::Date;{% endif %}
{% if has_time %}use sea_orm::prelude::Time;{% endif %}
{% if has_json %}use serde_json::Value as Json;{% endif %}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct {{ table.businessName | pascal_case }}SaveReqVO {
    {% for col in columns %}
    {% if col.createOperation and col.columnName != "id" %}
    #[validate(required)]
    pub {{ col.javaField }}: {{ col.javaType }},
    {% endif %}
    {% endfor %}
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct {{ table.businessName | pascal_case }}UpdateReqVo {
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
pub struct {{ table.businessName | pascal_case }}RespVo {
    pub id: String,
    {% for col in columns %}
    {% if col.columnName != "id" %}
    pub {{ col.javaField }}: {{ col.javaType }},
    {% endif %}
    {% endfor %}
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct {{ table.businessName | pascal_case }}SimpleRespVo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct {{ table.businessName | pascal_case }}PageReqVO {
    #[serde(flatten)]
    pub pagination: PageParam,
    {% for col in columns %}
    {% if col.listOperation %}
    pub {{ col.javaField }}: Option<{{ col.javaType }}>,
    {% endif %}
    {% endfor %}
}