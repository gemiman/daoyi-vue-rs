use crate::infra_entity::{infra_codegen_column, infra_codegen_table};
use daoyi_common_support::id_util;
use heck::{ToLowerCamelCase, ToPascalCase, ToSnakeCase};
use std::collections::HashMap;
use std::fs;
use tera::{Context, Tera, Value, to_value, try_get_value};

pub struct CodegenEngine;

impl CodegenEngine {
    pub fn execute(
        table: &infra_codegen_table::Model,
        columns: &[infra_codegen_column::Model],
        sub_tables: &[infra_codegen_table::Model],
        sub_columns_list: &[Vec<infra_codegen_column::Model>],
    ) -> HashMap<String, String> {
        let mut tera = Tera::default();

        tera.register_filter("snake_case", |value: &Value, _: &_| {
            let s = try_get_value!("snake_case", "value", String, value);
            Ok(to_value(s.to_snake_case()).unwrap())
        });

        tera.register_filter("camel_case", |value: &Value, _: &_| {
            let s = try_get_value!("camel_case", "value", String, value);
            Ok(to_value(s.to_lower_camel_case()).unwrap())
        });

        tera.register_filter("pascal_case", |value: &Value, _: &_| {
            let s = try_get_value!("pascal_case", "value", String, value);
            Ok(to_value(s.to_pascal_case()).unwrap())
        });

        // --- 0. VO Template ---

        let vo_tpl = fs::read_to_string("resources/codegen/rust/vo.rs.tpl")
            .expect("Failed to read vo.rs.tpl template");

        tera.add_raw_template("vo.rs", &vo_tpl).unwrap();

        // --- 1. Entity Template ---

        let entity_tpl = fs::read_to_string("resources/codegen/rust/entity.rs.tpl")
            .expect("Failed to read entity.rs.tpl template");

        tera.add_raw_template("entity.rs", &entity_tpl).unwrap();

        // --- 2. Service Template ---

        let service_tpl = fs::read_to_string("resources/codegen/rust/service.rs.tpl")
            .expect("Failed to read service.rs.tpl template");

        tera.add_raw_template("service.rs", &service_tpl).unwrap();

        // --- 3. Controller Template ---

        let controller_tpl = fs::read_to_string("resources/codegen/rust/controller.rs.tpl")
            .expect("Failed to read controller.rs.tpl template");

        tera.add_raw_template("controller.rs", &controller_tpl)
            .unwrap();

        // --- 4. Vue Index Template ---

        let vue_index_tpl = fs::read_to_string("resources/codegen/vue3/index.vue.tpl")
            .expect("Failed to read index.vue.tpl template");

        tera.add_raw_template("index.vue", &vue_index_tpl).unwrap();

        // --- 5. Vue Form Template ---

        let vue_form_tpl = fs::read_to_string("resources/codegen/vue3/form.vue.tpl")
            .expect("Failed to read form.vue.tpl template");

        tera.add_raw_template("form.vue", &vue_form_tpl).unwrap();

        // --- 6. SQL Template ---

        let sql_tpl = fs::read_to_string("resources/codegen/sql/sql.sql.tpl")
            .expect("Failed to read sql.sql.tpl template");

        tera.add_raw_template("sql.sql", &sql_tpl).unwrap();

        // --- 7. API Template ---

        let api_tpl = fs::read_to_string("resources/codegen/vue3/api.ts.tpl")
            .expect("Failed to read api.ts.tpl template");

        tera.add_raw_template("api.ts", &api_tpl).unwrap();

        // --- Context Preparation ---
        let mut context = Context::new();
        context.insert("table", table);
        context.insert("columns", columns);
        context.insert("sub_tables", sub_tables);
        context.insert("sub_columns_list", sub_columns_list);

        let primary_col = columns.iter().find(|c| c.primary_key);
        let primary_key_type = primary_col
            .map(|c| c.java_type.clone())
            .unwrap_or("String".to_string());
        let primary_key_is_auto = primary_key_type != "String";

        context.insert("primary_key_type", &primary_key_type);
        context.insert("primary_key_is_auto", &primary_key_is_auto);

        // Generate UUIDs for SQL
        context.insert("menu_id", &id_util::next_string());
        context.insert("button_query_id", &id_util::next_string());
        context.insert("button_create_id", &id_util::next_string());
        context.insert("button_update_id", &id_util::next_string());
        context.insert("button_delete_id", &id_util::next_string());
        context.insert("button_export_id", &id_util::next_string());

        // Check types for imports
        let has_decimal = columns.iter().any(|c| c.java_type == "Decimal");
        let has_json = columns.iter().any(|c| c.java_type == "Json");
        let has_date = columns.iter().any(|c| c.java_type == "Date");
        let has_time = columns.iter().any(|c| c.java_type == "Time");

        context.insert("has_decimal", &has_decimal);
        context.insert("has_json", &has_json);
        context.insert("has_date", &has_date);
        context.insert("has_time", &has_time);

        // --- Rendering ---
        let mut result = HashMap::new();

        // VO
        match tera.render("vo.rs", &context) {
            Ok(code) => {
                result.insert(
                    format!(
                        "backend/src/vo/{}_vo.rs",
                        table.business_name.to_snake_case()
                    ),
                    code,
                );
            }
            Err(e) => tracing::error!("Failed to render vo.rs: {:#?}", e),
        }

        // Entity
        match tera.render("entity.rs", &context) {
            Ok(code) => {
                result.insert(
                    format!("backend/src/entity/{}.rs", table.class_name.to_snake_case()),
                    code,
                );
            }
            Err(e) => tracing::error!("Failed to render entity.rs: {:#?}", e),
        }

        // Service
        match tera.render("service.rs", &context) {
            Ok(code) => {
                result.insert(
                    format!(
                        "backend/src/service/{}_service.rs",
                        table.class_name.to_snake_case()
                    ),
                    code,
                );
            }
            Err(e) => tracing::error!("Failed to render service.rs: {:#?}", e),
        }

        // Controller
        match tera.render("controller.rs", &context) {
            Ok(code) => {
                result.insert(
                    format!(
                        "backend/src/controller/{}_controller.rs",
                        table.class_name.to_snake_case()
                    ),
                    code,
                );
            }
            Err(e) => tracing::error!("Failed to render controller.rs: {:#?}", e),
        }

        // Vue Index
        match tera.render("index.vue", &context) {
            Ok(code) => {
                result.insert(
                    format!(
                        "frontend/src/views/{}/{}/index.vue",
                        table.module_name, table.business_name
                    ),
                    code,
                );
            }
            Err(e) => tracing::error!("Failed to render index.vue: {:#?}", e),
        }

        // Vue Form
        match tera.render("form.vue", &context) {
            Ok(code) => {
                result.insert(
                    format!(
                        "frontend/src/views/{}/{}/{}Form.vue",
                        table.module_name, table.business_name, table.class_name
                    ),
                    code,
                );
            }
            Err(e) => tracing::error!("Failed to render form.vue: {:#?}", e),
        }

        // SQL
        match tera.render("sql.sql", &context) {
            Ok(code) => {
                result.insert("sql/sql.sql".to_string(), code);
            }
            Err(e) => tracing::error!("Failed to render sql.sql: {:#?}", e),
        }

        // API
        match tera.render("api.ts", &context) {
            Ok(code) => {
                result.insert(
                    format!(
                        "frontend/src/api/{}/{}.ts",
                        table.module_name, table.business_name
                    ),
                    code,
                );
            }
            Err(e) => tracing::error!("Failed to render api.ts: {:#?}", e),
        }

        result
    }
}
