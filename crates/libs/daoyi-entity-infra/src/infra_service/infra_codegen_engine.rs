use crate::infra_entity::{infra_codegen_column, infra_codegen_table};
use daoyi_common_support::id_util;
use heck::{ToLowerCamelCase, ToPascalCase, ToSnakeCase};
use std::collections::HashMap;
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

        // --- 1. Entity Template ---
        let entity_tpl = r#"
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use daoyi_macros::{daoyi_model, DaoyiActiveModelBehavior};
{% if has_decimal %}use rust_decimal::Decimal;{% endif %}
{% if has_json %}use serde_json::Value as Json;{% endif %}
{% if has_date %}use sea_orm::prelude::Date;{% endif %}
{% if has_time %}use sea_orm::prelude::Time;{% endif %}

#[daoyi_model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, DaoyiActiveModelBehavior)]
#[sea_orm(table_name = "{{ table.tableName }}")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key{% if not primary_key_is_auto %}, auto_increment = false{% endif %})]
    pub id: {{ primary_key_type }},
    {% for col in columns %}
    {% if col.columnName != "id" %}
    /// {{ col.columnComment }}
    pub {{ col.javaField }}: {{ col.javaType }},
    {% endif %}
    {% endfor %}
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
"#;
        tera.add_raw_template("entity.rs", entity_tpl).unwrap();

        // --- 2. Service Template ---
        let service_tpl = r#"
use crate::entity::{prelude::*, {{ table.className | snake_case }}};
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::{database, id_util};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};
{% if has_decimal %}use rust_decimal::Decimal;{% endif %}
{% if has_json %}use serde_json::Value as Json;{% endif %}

pub async fn create_{{ table.className | snake_case }}(req: {{ table.className }}CreateReq) -> ApiResult<String> {
    let db = database::get_db_async().await;
    let id = id_util::xid();
    let model = {{ table.className | snake_case }}::ActiveModel {
        id: Set(id.clone()),
        {% for col in columns %}
        {% if col.createOperation and col.columnName != "id" %}
        {{ col.javaField }}: Set(req.{{ col.javaField }}),
        {% endif %}
        {% endfor %}
        ..Default::default()
    };
    model.insert(&db).await?;
    Ok(id)
}

pub async fn update_{{ table.className | snake_case }}(req: {{ table.className }}UpdateReq) -> ApiResult<()> {
    let db = database::get_db_async().await;
    let mut model: {{ table.className | snake_case }}::ActiveModel = {{ table.className }}::find_by_id(&req.id)
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::biz("Record not found"))?
        .into();

    {% for col in columns %}
    {% if col.updateOperation and col.columnName != "id" %}
    model.{{ col.javaField }} = Set(req.{{ col.javaField }});
    {% endif %}
    {% endfor %}
    
    model.update(&db).await?;
    Ok(())
}

pub async fn delete_{{ table.className | snake_case }}(id: &str) -> ApiResult<()> {
    let db = database::get_db_async().await;
    {{ table.className }}::delete_by_id(id).exec(&db).await?;
    Ok(())
}

pub async fn get_{{ table.className | snake_case }}(id: &str) -> ApiResult<Option<{{ table.className | snake_case }}::Model>> {
    let db = database::get_db_async().await;
    let res = {{ table.className }}::find_by_id(id).one(&db).await?;
    Ok(res)
}

pub async fn get_{{ table.className | snake_case }}_page(req: {{ table.className }}PageReq) -> ApiResult<PageResult<{{ table.className | snake_case }}::Model>> {
    let db = database::get_db_async().await;
    let paginator = {{ table.className }}::find()
        {% for col in columns %}
        {% if col.listOperation %}
        .apply_if(req.{{ col.javaField }}, |query, val| {
            query.filter({{ table.className | snake_case }}::Column::{{ col.javaField | pascal_case }}.eq(val))
        })
        {% endif %}
        {% endfor %}
        .order_by_desc({{ table.className | snake_case }}::Column::CreateTime)
        .paginate(&db, req.pagination.page_size);

    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(req.pagination.page_no - 1).await?;
    Ok(PageResult::from_pagination(&req.pagination, total, list))
}
"#;
        tera.add_raw_template("service.rs", service_tpl).unwrap();

        // --- 3. Controller Template ---
        let controller_tpl = r#"
use axum::{
    extract::{Json, Query},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use daoyi_common_support::{
    app::AppState,
    request::valid::ValidJson,
    response::{ApiResponse, RestApiResult},
};
use crate::service::{{ table.className | snake_case }}_service;
// Import VO structs here
// Note: VO definition is usually in a separate file or common lib, 
// but for codegen simplicity we assume the user will handle VO imports or define them here.

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create))
        .route("/update", put(update))
        .route("/delete", delete(delete))
        .route("/get", get(get))
        .route("/page", get(get_page))
}

async fn create(ValidJson(req): ValidJson<{{ table.className }}CreateReq>) -> RestApiResult<String> {
    let res = {{ table.className | snake_case }}_service::create_{{ table.className | snake_case }}(req).await?;
    ApiResponse::success(res)
}

async fn update(ValidJson(req): ValidJson<{{ table.className }}UpdateReq>) -> RestApiResult<()> {
    {{ table.className | snake_case }}_service::update_{{ table.className | snake_case }}(req).await?;
    ApiResponse::success(())
}

async fn delete(Query(req): Query<IdReq>) -> RestApiResult<()> {
    {{ table.className | snake_case }}_service::delete_{{ table.className | snake_case }}(&req.id).await?;
    ApiResponse::success(())
}

async fn get(Query(req): Query<IdReq>) -> RestApiResult<Option<{{ table.className | snake_case }}::Model>> {
    let res = {{ table.className | snake_case }}_service::get_{{ table.className | snake_case }}(&req.id).await?;
    ApiResponse::success(res)
}

async fn get_page(Query(req): Query<{{ table.className }}PageReq>) -> RestApiResult<PageResult<{{ table.className | snake_case }}::Model>> {
    let res = {{ table.className | snake_case }}_service::get_{{ table.className | snake_case }}_page(req).await?;
    ApiResponse::success(res)
}
"#;
        tera.add_raw_template("controller.rs", controller_tpl)
            .unwrap();

        // --- 4. Vue Index Template ---
        let vue_index_tpl = r#"
<template>
  <ContentWrap>
    <!-- Search Form -->
    <el-form :model="queryParams" ref="queryFormRef" :inline="true" label-width="68px">
      {% for col in columns %}
      {% if col.listOperation %}
      <el-form-item label="{{ col.columnComment }}" prop="{{ col.javaField }}">
        <el-input v-model="queryParams.{{ col.javaField }}" placeholder="请输入{{ col.columnComment }}" clearable class="!w-240px" />
      </el-form-item>
      {% endif %}
      {% endfor %}
      <el-form-item>
        <el-button @click="handleQuery"><Icon icon="ep:search" class="mr-5px" /> 搜索</el-button>
        <el-button @click="resetQuery"><Icon icon="ep:refresh" class="mr-5px" /> 重置</el-button>
        <el-button type="primary" plain @click="openForm('create')">
          <Icon icon="ep:plus" class="mr-5px" /> 新增
        </el-button>
      </el-form-item>
    </el-form>
  </ContentWrap>

  <ContentWrap>
    <el-table v-loading="loading" :data="list">
      {% for col in columns %}
      {% if col.listOperationResult %}
      <el-table-column label="{{ col.columnComment }}" align="center" prop="{{ col.javaField }}" />
      {% endif %}
      {% endfor %}
      <el-table-column label="操作" align="center" fixed="right" width="180">
        <template #default="scope">
          <el-button link type="primary" @click="openForm('update', scope.row.id)">编辑</el-button>
          <el-button link type="danger" @click="handleDelete(scope.row.id)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
    <Pagination
      :total="total"
      v-model:page="queryParams.pageNo"
      v-model:limit="queryParams.pageSize"
      @pagination="getList"
    />
  </ContentWrap>

  <!-- Form Dialog -->
  <{{ table.className }}Form ref="formRef" @success="getList" />
</template>
<script setup lang="ts">
import { dateFormatter } from '@/utils/formatTime'
import * as {{ table.className }}Api from '@/api/{{ table.moduleName }}/{{ table.businessName }}'
import {{ table.className }}Form from './{{ table.className }}Form.vue'

const message = useMessage()
const { t } = useI18n()

const loading = ref(true)
const list = ref([])
const total = ref(0)
const queryParams = reactive({
  pageNo: 1,
  pageSize: 10,
  {% for col in columns %}
  {% if col.listOperation %}
  {{ col.javaField }}: undefined,
  {% endif %}
  {% endfor %}
})
const queryFormRef = ref()

const getList = async () => {
  loading.value = true
  try {
    const data = await {{ table.className }}Api.get{{ table.className }}Page(queryParams)
    list.value = data.list
    total.value = data.total
  } finally {
    loading.value = false
  }
}

const handleQuery = () => {
  queryParams.pageNo = 1
  getList()
}

const resetQuery = () => {
  queryFormRef.value.resetFields()
  handleQuery()
}

const formRef = ref()
const openForm = (type: string, id?: number) => {
  formRef.value.open(type, id)
}

const handleDelete = async (id: number) => {
  try {
    await message.delConfirm()
    await {{ table.className }}Api.delete{{ table.className }}(id)
    message.success(t('common.delSuccess'))
    await getList()
  } catch {}
}

onMounted(() => {
  getList()
})
</script>
"#;
        tera.add_raw_template("index.vue", vue_index_tpl).unwrap();

        // --- 5. Vue Form Template ---
        let vue_form_tpl = r#"
<template>
  <Dialog :title="dialogTitle" v-model="dialogVisible">
    <el-form ref="formRef" :model="formData" :rules="formRules" label-width="100px" v-loading="formLoading">
      {% for col in columns %}
      {% if col.createOperation or col.updateOperation %}
      <el-form-item label="{{ col.columnComment }}" prop="{{ col.javaField }}">
        {% if col.htmlType == 'input' %}
        <el-input v-model="formData.{{ col.javaField }}" placeholder="请输入{{ col.columnComment }}" />
        {% elif col.htmlType == 'textarea' %}
        <el-input v-model="formData.{{ col.javaField }}" type="textarea" placeholder="请输入{{ col.columnComment }}" />
        {% elif col.htmlType == 'datetime' %}
        <el-date-picker v-model="formData.{{ col.javaField }}" type="datetime" value-format="x" placeholder="选择{{ col.columnComment }}" />
        {% else %}
        <el-input v-model="formData.{{ col.javaField }}" placeholder="请输入{{ col.columnComment }}" />
        {% endif %}
      </el-form-item>
      {% endif %}
      {% endfor %}
    </el-form>
    
    {% if sub_tables | length > 0 %}
    <!-- Sub Tables -->
    <el-tabs v-model="activeTab">
      {% for sub in sub_tables %}
      <el-tab-pane label="{{ sub.tableComment }}" name="{{ sub.className | camel_case }}">
        <!-- Placeholder for sub-table component -->
        <div>Sub-table: {{ sub.tableComment }}</div>
      </el-tab-pane>
      {% endfor %}
    </el-tabs>
    {% endif %}

    <template #footer>
      <el-button @click="submitForm" type="primary" :disabled="formLoading">确 定</el-button>
      <el-button @click="dialogVisible = false">取 消</el-button>
    </template>
  </Dialog>
</template>
<script setup lang="ts">
import * as {{ table.className }}Api from '@/api/{{ table.moduleName }}/{{ table.businessName }}'

const { t } = useI18n()
const message = useMessage()

const dialogVisible = ref(false)
const dialogTitle = ref('')
const formLoading = ref(false)
const formType = ref('')
const formData = ref({
  {% for col in columns %}
  {% if col.createOperation or col.updateOperation %}
  {{ col.javaField }}: undefined,
  {% endif %}
  {% endfor %}
})
const formRules = reactive({
  {% for col in columns %}
  {% if not col.nullable and (col.createOperation or col.updateOperation) %}
  {{ col.javaField }}: [{ required: true, message: '{{ col.columnComment }}不能为空', trigger: 'blur' }],
  {% endif %}
  {% endfor %}
})
const formRef = ref()
const activeTab = ref('')

const open = async (type: string, id?: number) => {
  dialogVisible.value = true
  dialogTitle.value = t('action.' + type)
  formType.value = type
  resetForm()
  if (id) {
    formLoading.value = true
    try {
      formData.value = await {{ table.className }}Api.get{{ table.className }}(id)
    } finally {
      formLoading.value = false
    }
  }
}
defineExpose({ open })

const emit = defineEmits(['success'])
const submitForm = async () => {
  await formRef.value.validate()
  formLoading.value = true
  try {
    const data = formData.value
    if (formType.value === 'create') {
      await {{ table.className }}Api.create{{ table.className }}(data)
      message.success(t('common.createSuccess'))
    } else {
      await {{ table.className }}Api.update{{ table.className }}(data)
      message.success(t('common.updateSuccess'))
    }
    dialogVisible.value = false
    emit('success')
  } finally {
    formLoading.value = false
  }
}

const resetForm = () => {
  formData.value = {
    {% for col in columns %}
    {% if col.createOperation or col.updateOperation %}
    {{ col.javaField }}: undefined,
    {% endif %}
    {% endfor %}
  }
  formRef.value?.resetFields()
}
</script>
"#;
        tera.add_raw_template("form.vue", vue_form_tpl).unwrap();

        // --- 6. SQL Template ---
        let sql_tpl = r#"
-- 菜单 SQL
INSERT INTO system.system_menu (
    id, name, permission, type, sort, parent_id,
    path, icon, component, component_name, status, visible, keep_alive, always_show,
    creator, create_time, updater, update_time, deleted, tenant_id
)
VALUES (
    '{{ menu_id }}', '{{ table.classComment }}', '', '2', {{ table.genMenuSort | default(value=1) }}, '{{ table.parentMenuId }}',
    '{{ table.businessName }}', 'ep:menu', '{{ table.moduleName }}/{{ table.businessName }}/index', '{{ table.className }}', '0', true, true, true,
    'admin', CURRENT_TIMESTAMP, 'admin', CURRENT_TIMESTAMP, false, '1'
);

-- 按钮 SQL
-- 1. 查询
INSERT INTO system.system_menu (
    id, name, permission, type, sort, parent_id,
    path, icon, component, component_name, status, visible, keep_alive, always_show,
    creator, create_time, updater, update_time, deleted, tenant_id
)
VALUES (
    '{{ button_query_id }}', '{{ table.classComment }}查询', '{{ table.moduleName }}:{{ table.businessName }}:query', '3', 1, '{{ menu_id }}',
    '', '', '', '', '0', true, true, true,
    'admin', CURRENT_TIMESTAMP, 'admin', CURRENT_TIMESTAMP, false, '1'
);

-- 2. 新增
INSERT INTO system.system_menu (
    id, name, permission, type, sort, parent_id,
    path, icon, component, component_name, status, visible, keep_alive, always_show,
    creator, create_time, updater, update_time, deleted, tenant_id
)
VALUES (
    '{{ button_create_id }}', '{{ table.classComment }}新增', '{{ table.moduleName }}:{{ table.businessName }}:create', '3', 2, '{{ menu_id }}',
    '', '', '', '', '0', true, true, true,
    'admin', CURRENT_TIMESTAMP, 'admin', CURRENT_TIMESTAMP, false, '1'
);

-- 3. 修改
INSERT INTO system.system_menu (
    id, name, permission, type, sort, parent_id,
    path, icon, component, component_name, status, visible, keep_alive, always_show,
    creator, create_time, updater, update_time, deleted, tenant_id
)
VALUES (
    '{{ button_update_id }}', '{{ table.classComment }}修改', '{{ table.moduleName }}:{{ table.businessName }}:update', '3', 3, '{{ menu_id }}',
    '', '', '', '', '0', true, true, true,
    'admin', CURRENT_TIMESTAMP, 'admin', CURRENT_TIMESTAMP, false, '1'
);

-- 4. 删除
INSERT INTO system.system_menu (
    id, name, permission, type, sort, parent_id,
    path, icon, component, component_name, status, visible, keep_alive, always_show,
    creator, create_time, updater, update_time, deleted, tenant_id
)
VALUES (
    '{{ button_delete_id }}', '{{ table.classComment }}删除', '{{ table.moduleName }}:{{ table.businessName }}:delete', '3', 4, '{{ menu_id }}',
    '', '', '', '', '0', true, true, true,
    'admin', CURRENT_TIMESTAMP, 'admin', CURRENT_TIMESTAMP, false, '1'
);

-- 5. 导出
INSERT INTO system.system_menu (
    id, name, permission, type, sort, parent_id,
    path, icon, component, component_name, status, visible, keep_alive, always_show,
    creator, create_time, updater, update_time, deleted, tenant_id
)
VALUES (
    '{{ button_export_id }}', '{{ table.classComment }}导出', '{{ table.moduleName }}:{{ table.businessName }}:export', '3', 5, '{{ menu_id }}',
    '', '', '', '', '0', true, true, true,
    'admin', CURRENT_TIMESTAMP, 'admin', CURRENT_TIMESTAMP, false, '1'
);
"#;
        tera.add_raw_template("sql.sql", sql_tpl).unwrap();

        // --- 7. API Template (New) ---
        let api_tpl = r#"
import request from '@/config/axios'

export interface {{ table.className }}VO {
  id: number
  {% for col in columns %}
  {% if col.columnName != "id" %}
  {{ col.javaField }}: {{ col.javaType | replace(from="i32", to="number") | replace(from="i64", to="number") | replace(from="f64", to="number") | replace(from="String", to="string") | replace(from="bool", to="boolean") | replace(from="DateTime", to="number") | replace(from="Date", to="number") | replace(from="Decimal", to="number") }}
  {% endif %}
  {% endfor %}
}

export interface {{ table.className }}PageReqVO extends PageParam {
  {% for col in columns %}
  {% if col.listOperation %}
  {{ col.javaField }}?: {{ col.javaType | replace(from="i32", to="number") | replace(from="i64", to="number") | replace(from="f64", to="number") | replace(from="String", to="string") | replace(from="bool", to="boolean") | replace(from="DateTime", to="number") | replace(from="Date", to="number") | replace(from="Decimal", to="number") }}
  {% endif %}
  {% endfor %}
}

// 查询列表
export const get{{ table.className }}Page = (params: {{ table.className }}PageReqVO) => {
  return request.get({ url: '/{{ table.moduleName }}/{{ table.businessName }}/page', params })
}

// 查询详情
export const get{{ table.className }} = (id: number) => {
  return request.get({ url: '/{{ table.moduleName }}/{{ table.businessName }}/get?id=' + id })
}

// 新增
export const create{{ table.className }} = (data: {{ table.className }}VO) => {
  return request.post({ url: '/{{ table.moduleName }}/{{ table.businessName }}/create', data })
}

// 修改
export const update{{ table.className }} = (data: {{ table.className }}VO) => {
  return request.put({ url: '/{{ table.moduleName }}/{{ table.businessName }}/update', data })
}

// 删除
export const delete{{ table.className }} = (id: number) => {
  return request.delete({ url: '/{{ table.moduleName }}/{{ table.businessName }}/delete?id=' + id })
}

// 导出
export const export{{ table.className }} = (params: {{ table.className }}PageReqVO) => {
  return request.download({ url: '/{{ table.moduleName }}/{{ table.businessName }}/export-excel', params })
}
"#;
        tera.add_raw_template("api.ts", api_tpl).unwrap();

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
        context.insert("menu_id", &id_util::xid());
        context.insert("button_query_id", &id_util::xid());
        context.insert("button_create_id", &id_util::xid());
        context.insert("button_update_id", &id_util::xid());
        context.insert("button_delete_id", &id_util::xid());
        context.insert("button_export_id", &id_util::xid());

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
