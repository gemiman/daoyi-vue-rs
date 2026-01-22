use daoyi_common_support::vo::infra_vo::{CodegenColumnRespVO, CodegenColumnSaveReqVO};
use daoyi_macros::{DaoyiActiveModelBehavior, daoyi_model};
use sea_orm::entity::prelude::*;
use sea_orm::{Set, Unchanged};
use serde::{Deserialize, Serialize};

#[daoyi_model]
#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, DaoyiActiveModelBehavior,
)]
#[sea_orm(schema_name = "infra", table_name = "infra_codegen_column")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub table_id: String,
    pub column_name: String,
    pub data_type: String,
    pub column_comment: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub ordinal_position: i32,
    pub java_type: String, // Maps to Rust type
    pub java_field: String,
    pub dict_type: Option<String>,
    pub example: Option<String>,
    pub create_operation: bool,
    pub update_operation: bool,
    pub list_operation: bool,
    pub list_operation_condition: String,
    pub list_operation_result: bool,
    pub html_type: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl From<Model> for CodegenColumnRespVO {
    fn from(value: Model) -> Self {
        Self {
            column_comment: value.column_comment,
            column_name: value.column_name,
            create_operation: value.create_operation,
            create_time: value.create_time,
            data_type: value.data_type,
            dict_type: value.dict_type,
            example: value.example,
            html_type: value.html_type,
            id: value.id,
            java_field: value.java_field,
            java_type: value.java_type,
            list_operation: value.list_operation,
            list_operation_condition: value.list_operation_condition,
            list_operation_result: value.list_operation_result,
            nullable: value.nullable,
            ordinal_position: value.ordinal_position,
            primary_key: value.primary_key,
            table_id: value.table_id,
            update_operation: value.update_operation,
        }
    }
}

impl From<CodegenColumnSaveReqVO> for ActiveModel {
    fn from(value: CodegenColumnSaveReqVO) -> Self {
        Self {
            id: Unchanged(value.id),
            table_id: Set(value.table_id),
            column_name: Set(value.column_name),
            data_type: Set(value.data_type),
            column_comment: Set(value.column_comment),
            nullable: Set(value.nullable),
            primary_key: Set(value.primary_key),
            ordinal_position: Set(value.ordinal_position),
            java_type: Set(value.java_type),
            java_field: Set(value.java_field),
            dict_type: Set(value.dict_type),
            example: Set(value.example),
            create_operation: Set(value.create_operation),
            update_operation: Set(value.update_operation),
            list_operation: Set(value.list_operation),
            list_operation_condition: Set(value.list_operation_condition),
            html_type: Set(value.html_type),
            ..Default::default()
        }
    }
}
