use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_macros::{daoyi_model, DaoyiActiveModelBehavior};
use sea_orm::entity::prelude::*;
use sea_orm::{Set, Unchanged};
use serde::{Deserialize, Serialize};
// TODO: Import your VOs here. Assuming they are in a module named after business_name or similar.
// use crate::vo::{{ table.businessName | snake_case }}_vo::{ {{ table.businessName | pascal_case }}RespVo, {{ table.businessName | pascal_case }}SaveReqVO, {{ table.businessName | pascal_case }}SimpleRespVo, {{ table.businessName | pascal_case }}UpdateReqVo };

{% if has_decimal %}use rust_decimal::Decimal;
use sea_orm::prelude::Date;
use sea_orm::prelude::Time;
use serde_json::Value as Json;
{% endif %}
{% if has_json %}{% endif %}
{% if has_date %}{% endif %}
{% if has_time %}{% endif %}

#[daoyi_model]
#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize,
    DaoyiActiveModelBehavior
)]
#[sea_orm(table_name = "{{ table.tableName }}")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key{% if not primary_key_is_auto %}, auto_increment = false{% endif %})]
    pub id: {
    { primary_key_type }
},
{% for col in columns %}
{% if col.columnName != "id" %}
/// {{ col.columnComment }}
pub {{ col.javaField }}: {{ col.javaType }},
{% endif %}
{% endfor %}
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl From<Model> for { { table.businessName | pascal_case }}SimpleRespVo {
fn from(value: Model) -> Self {
    Self {
        id: value.id,
        name: value.name, // Ensure 'name' field exists or adjust
    }
}
}

impl From<Model> for { { table.businessName | pascal_case }}RespVo {
fn from(value: Model) -> Self {
    Self {
        id: value.id,
        {% for col in columns %}
        {% if col.columnName != "id" %}
        {{ col.javaField }}: value.{{ col.javaField }},
    {
        % endif %
    }
    { % endfor % }
}
    }
}

impl From<{ { table.businessName | pascal_case } }SaveReqVO> for ActiveModel {
    fn from(value: { { table.businessName | pascal_case } }SaveReqVO) -> Self {
    Self {
    {% for col in columns %}
    {% if col.createOperation and col.columnName != "id" %}
    {{ col.javaField }}: Set(value.{{ col.javaField }}),
    {% endif %}
    {% endfor %}
    ..Default::default()
    }
    }
}

impl From<{ { table.businessName | pascal_case } }UpdateReqVo> for ActiveModel {
    fn from(value: { { table.businessName | pascal_case } }UpdateReqVo) -> Self {
    Self {
    id: Unchanged(value.id),
    {% for col in columns %}
    {% if col.updateOperation and col.columnName != "id" %}
    {{ col.javaField }}: Set(value.{{ col.javaField }}),
    {% endif %}
    {% endfor %}
    ..Default::default()
    }
    }
}
