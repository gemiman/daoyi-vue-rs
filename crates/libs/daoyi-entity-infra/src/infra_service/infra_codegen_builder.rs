use crate::infra_entity::{infra_codegen_column, infra_codegen_table};
use daoyi_common_support::enumeration::{
    CodegenFrontTypeEnum, CodegenSceneEnum, CodegenTemplateTypeEnum,
};
use daoyi_common_support::id_util;
use heck::{ToLowerCamelCase, ToPascalCase};
use sea_orm::Set;

pub struct CodegenBuilder;

impl CodegenBuilder {
    pub fn build_table(
        data_source_config_id: &str,
        table_name: &str,
        table_comment: &str,
    ) -> infra_codegen_table::ActiveModel {
        // Assume format like "module_business_name" or "module_business"
        // 1. Module Name: prefix before first '_'
        let parts: Vec<&str> = table_name.split('_').collect();
        let module_name = parts.first().unwrap_or(&"module").to_string();

        // 2. Business Name: rest of the parts
        let business_name = if parts.len() > 1 {
            parts[1..].join("_")
        } else {
            table_name.to_string()
        };
        let business_name_camel = business_name.to_lower_camel_case();

        // 3. Class Name: Business Name in PascalCase
        let class_name = business_name.to_pascal_case();

        infra_codegen_table::ActiveModel {
            id: Set(id_util::xid()),
            data_source_config_id: Set(data_source_config_id.to_string()),
            table_name: Set(table_name.to_string()),
            table_comment: Set(table_comment.to_string()),
            class_name: Set(class_name),
            module_name: Set(module_name),
            business_name: Set(business_name_camel),
            author: Set("admin".to_string()),
            template_type: Set(CodegenTemplateTypeEnum::ONE), // CRUD
            front_type: Set(CodegenFrontTypeEnum::Vue3ElementPlus), // Vue3 Element Plus
            scene: Set(CodegenSceneEnum::ADMIN),              // Admin
            ..Default::default()
        }
    }

    pub fn build_column(
        table_id: &str,
        column_name: &str,
        data_type: &str,
        column_comment: &str,
        is_nullable: bool,
        is_primary_key: bool,
        ordinal_position: i32,
    ) -> infra_codegen_column::ActiveModel {
        let java_type = match data_type.to_lowercase().as_str() {
            "varchar" | "text" | "char" | "mediumtext" | "longtext" => "String",
            "bigint" => "i64",
            "int" | "integer" => "i32",
            "tinyint" => "i16", // SeaORM TinyInt is often i8, but i16 is safer for logic
            "smallint" => "i16",
            "double" => "f64",
            "float" => "f32",
            "decimal" | "numeric" => "Decimal",
            "datetime" | "timestamp" => "DateTime",
            "date" => "Date",
            "time" => "Time",
            "boolean" | "bit" => "bool",
            "json" | "jsonb" => "Json",
            "blob" | "longblob" => "Vec<u8>",
            _ => "String",
        };

        let java_field = column_name.to_lower_camel_case();

        // Defaults
        let mut create_operation = true;
        let mut update_operation = true;
        let mut list_operation = true;
        let mut list_operation_result = true;
        let mut list_operation_condition = "=".to_string();
        let mut html_type = "input".to_string();

        // BaseDO fields handling (id, create_time, update_time, creator, updater, deleted)
        match column_name {
            "id" => {
                create_operation = false;
                update_operation = false; // Usually not updated
                list_operation = false; // Usually not filtered by exact ID list in standard page
            }
            "create_time" => {
                create_operation = false;
                update_operation = false;
                list_operation_condition = "BETWEEN".to_string();
                html_type = "datetime".to_string();
            }
            "update_time" | "creator" | "updater" | "deleted" => {
                create_operation = false;
                update_operation = false;
                list_operation = false;
                list_operation_result = false; // Usually hidden
            }
            _ => {}
        }

        // HTML Type guessing
        if column_name.ends_with("image") || column_name.ends_with("avatar") {
            html_type = "imageUpload".to_string();
        } else if column_name.ends_with("file") {
            html_type = "fileUpload".to_string();
        } else if column_name.ends_with("content") || column_name.ends_with("description") {
            html_type = "editor".to_string();
        } else if data_type == "datetime" || data_type == "timestamp" {
            html_type = "datetime".to_string();
        } else if data_type == "tinyint" || data_type == "boolean" {
            html_type = "radio".to_string();
        }

        // Condition guessing
        if column_name.ends_with("name") || column_name.ends_with("title") {
            list_operation_condition = "LIKE".to_string();
        }

        infra_codegen_column::ActiveModel {
            id: Set(id_util::xid()),
            table_id: Set(table_id.to_string()),
            column_name: Set(column_name.to_string()),
            data_type: Set(data_type.to_string()),
            column_comment: Set(column_comment.to_string()),
            nullable: Set(is_nullable),
            primary_key: Set(is_primary_key),
            ordinal_position: Set(ordinal_position),
            java_type: Set(java_type.to_string()),
            java_field: Set(java_field),
            dict_type: Set(None),
            example: Set(None),
            create_operation: Set(create_operation),
            update_operation: Set(update_operation),
            list_operation: Set(list_operation),
            list_operation_condition: Set(list_operation_condition),
            list_operation_result: Set(list_operation_result),
            html_type: Set(html_type),
            ..Default::default()
        }
    }
}
