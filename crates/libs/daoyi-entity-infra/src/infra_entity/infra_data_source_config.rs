use daoyi_common_support::configs::DatabaseConfig;
use daoyi_common_support::enumeration::ID_ROOT;
use daoyi_common_support::vo::infra_vo::{
    DataSourceConfigRespVO, DataSourceConfigSaveReqVO, DataSourceConfigUpdateReqVO,
};
use daoyi_macros::{DaoyiActiveModelBehavior, daoyi_model};
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{Set, Unchanged};
use serde::{Deserialize, Serialize};

#[daoyi_model]
#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize,
    DaoyiActiveModelBehavior,
)]
#[sea_orm(schema_name = "infra", table_name = "infra_data_source_config")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub url: String,
    pub schema_name: String,
    pub username: Option<String>,
    pub password_plaintext: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl From<&DatabaseConfig> for Model {
    fn from(value: &DatabaseConfig) -> Self {
        Self {
            id: String::from(ID_ROOT),
            name: String::from("Master"),
            url: value.url(),
            schema_name: String::from(value.schema()),
            username: Some(String::from(value.user())),
            password_plaintext: Some(String::from(value.password())),
            create_time: Local::now().naive_local(),
            creator: None,
            update_time: Local::now().naive_local(),
            updater: None,
            deleted: false,
            tenant_id: String::from(ID_ROOT),
        }
    }
}

impl From<DataSourceConfigSaveReqVO> for ActiveModel {
    fn from(value: DataSourceConfigSaveReqVO) -> Self {
        Self {
            name: Set(value.name),
            url: Set(value.url),
            schema_name: Set(value.schema_name),
            username: Set(value.username),
            password_plaintext: Set(value.password),
            ..Default::default()
        }
    }
}

impl From<DataSourceConfigUpdateReqVO> for ActiveModel {
    fn from(value: DataSourceConfigUpdateReqVO) -> Self {
        Self {
            id: Unchanged(value.id),
            name: Set(value.name),
            url: Set(value.url),
            schema_name: Set(value.schema_name),
            username: Set(value.username),
            password_plaintext: Set(value.password),
            ..Default::default()
        }
    }
}

impl From<Model> for DataSourceConfigRespVO {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            url: value.url,
            username: value.username,
            password: value.password_plaintext,
            schema_name: value.schema_name,
            create_time: value.create_time,
        }
    }
}
