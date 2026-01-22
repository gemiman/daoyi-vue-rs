use daoyi_common_support::enumeration::{
    CodegenFrontTypeEnum, CodegenSceneEnum, CodegenTemplateTypeEnum,
};
use daoyi_common_support::vo::infra_vo::{CodegenTableRespVO, CodegenTableSaveReqVO};
use daoyi_macros::{DaoyiActiveModelBehavior, daoyi_model};
use sea_orm::entity::prelude::*;
use sea_orm::{Set, Unchanged};
use serde::{Deserialize, Serialize};

#[daoyi_model]
#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, DaoyiActiveModelBehavior,
)]
#[sea_orm(schema_name = "infra", table_name = "infra_codegen_table")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub data_source_config_id: String,
    pub scene: CodegenSceneEnum,
    pub table_name: String,
    pub table_comment: String,
    pub remark: Option<String>,
    pub module_name: String,
    pub business_name: String,
    pub class_name: String,
    pub class_comment: String,
    pub author: String,
    pub template_type: CodegenTemplateTypeEnum,
    pub front_type: CodegenFrontTypeEnum,
    pub parent_menu_id: Option<String>,
    pub master_table_id: Option<String>,
    pub sub_join_column_id: Option<String>,
    pub sub_join_many: Option<bool>,
    pub tree_parent_column_id: Option<String>,
    pub tree_name_column_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl From<Model> for CodegenTableRespVO {
    fn from(value: Model) -> Self {
        Self {
            author: value.author,
            business_name: value.business_name,
            id: value.id,
            master_table_id: value.master_table_id,
            module_name: value.module_name,
            parent_menu_id: value.parent_menu_id,
            remark: value.remark,
            scene: value.scene,
            sub_join_column_id: value.sub_join_column_id,
            data_source_config_id: value.data_source_config_id,
            table_name: value.table_name,
            template_type: value.template_type,
            tree_name_column_id: value.tree_name_column_id,
            table_comment: value.table_comment,
            class_name: value.class_name,
            create_time: value.create_time,
            update_time: value.update_time,
            class_comment: value.class_comment,
            front_type: value.front_type,
            sub_join_many: value.sub_join_many,
            tree_parent_column_id: value.tree_parent_column_id,
        }
    }
}

impl From<CodegenTableSaveReqVO> for ActiveModel {
    fn from(value: CodegenTableSaveReqVO) -> Self {
        Self {
            id: Unchanged(value.id),
            scene: Set(value.scene),
            table_name: Set(value.table_name),
            table_comment: Set(value.table_comment),
            remark: Set(value.remark),
            module_name: Set(value.module_name),
            business_name: Set(value.business_name),
            class_name: Set(value.class_name),
            class_comment: Set(value.class_comment),
            author: Set(value.author),
            template_type: Set(value.template_type),
            front_type: Set(value.front_type),
            parent_menu_id: Set(value.parent_menu_id),
            master_table_id: Set(value.master_table_id),
            sub_join_column_id: Set(value.sub_join_column_id),
            sub_join_many: Set(value.sub_join_many),
            tree_parent_column_id: Set(value.tree_parent_column_id),
            tree_name_column_id: Set(value.tree_name_column_id),
            ..Default::default()
        }
    }
}
