use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Block, Data, DeriveInput, ExprPath, Fields, ImplItem, Item, ItemStruct, parse_macro_input,
    parse_quote,
    visit_mut::{self, VisitMut},
};

struct DatabaseGetReplacer;

impl VisitMut for DatabaseGetReplacer {
    fn visit_expr_path_mut(&mut self, i: &mut ExprPath) {
        visit_mut::visit_expr_path_mut(self, i);
        if is_database_get(i) {
            *i = parse_quote! { daoyi_common_support::database::get_db_async };
        }
    }
}

fn is_database_get(path: &ExprPath) -> bool {
    let segments = &path.path.segments;
    if segments.is_empty() {
        return false;
    }
    let last = segments.last().unwrap();
    if last.ident != "get" {
        return false;
    }

    // Check for `database::get`
    if segments.len() >= 2 {
        let second_last = &segments[segments.len() - 2];
        if second_last.ident == "database" {
            return true;
        }
    }

    false
}

fn process_block(block: &mut Block) {
    // 1. Visit body and replace calls
    let mut replacer = DatabaseGetReplacer;
    replacer.visit_block_mut(block);

    // 2. Wrap body
    let original_stmts = &block.stmts;
    let new_block: Block = parse_quote! {
        {
            daoyi_common_support::database::call_in_transaction(async move {
                #(#original_stmts)*
            }).await
        }
    };

    *block = new_block;
}

#[proc_macro_attribute]
pub fn transactional(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    match item {
        Item::Fn(mut item_fn) => {
            process_block(&mut item_fn.block);
            TokenStream::from(quote! { #item_fn })
        }
        Item::Impl(mut item_impl) => {
            for item in &mut item_impl.items {
                if let ImplItem::Fn(method) = item {
                    if method.sig.asyncness.is_some() {
                        process_block(&mut method.block);
                    }
                }
            }
            TokenStream::from(quote! { #item_impl })
        }
        _ => TokenStream::from(quote! { #item }),
    }
}

#[proc_macro_attribute]
pub fn daoyi_model(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);

    if let Fields::Named(ref mut fields) = item_struct.fields {
        let new_fields: Vec<syn::Field> = vec![
            parse_quote! {
                pub creator: Option<String>
            },
            parse_quote! {
                #[serde(with = "daoyi_common_support::serde::datetime_format")]
                pub create_time: DateTime
            },
            parse_quote! {
                pub updater: Option<String>
            },
            parse_quote! {
                #[serde(with = "daoyi_common_support::serde::datetime_format")]
                pub update_time: DateTime
            },
            parse_quote! {
                pub deleted: bool
            },
            parse_quote! {
                pub tenant_id: String
            },
        ];

        for field in new_fields {
            fields.named.push(field);
        }
    }

    TokenStream::from(quote! {
        #item_struct

        impl Entity {
            pub async fn find_perm() -> sea_orm::Select<Entity> {
                use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
                let mut query = <Self as EntityTrait>::find()
                    .filter(Column::Deleted.eq(false));
                query
            }
            pub async fn find_perm_with_tenant() -> sea_orm::Select<Entity> {
                use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
                let mut query = Self::find_perm().await;

                if let Some(tenant_id) = daoyi_common_support::context::HttpRequestContext::get_tenant_id_arc() {
                     if !daoyi_common_support::context::HttpRequestContext::get_ignore_tenant() {
                        query = query.filter(Column::TenantId.eq(tenant_id.as_str()));
                    }
                }
                query
            }

            pub async fn find_by_id_perm<C, T>(db: &C, id: T) -> Result<Option<Model>, sea_orm::DbErr>
            where
                C: sea_orm::ConnectionTrait,
                T: Into<sea_orm::Value>,
            {
                use sea_orm::{QueryFilter, ColumnTrait};
                Self::find_perm().await
                    .filter(Column::Id.eq(id))
                    .one(db)
                    .await
            }

            pub async fn find_by_id_perm_with_tenant<C, T>(db: &C, id: T) -> Result<Option<Model>, sea_orm::DbErr>
            where
                C: sea_orm::ConnectionTrait,
                T: Into<sea_orm::Value>,
            {
                use sea_orm::{QueryFilter, ColumnTrait};
                Self::find_perm_with_tenant().await
                    .filter(Column::Id.eq(id))
                    .one(db)
                    .await
            }


            pub async fn find_by_ids_perm<C, I, V>(db: &C, ids: I) -> Result<Vec<Model>, sea_orm::DbErr>
            where
                C: sea_orm::ConnectionTrait,
                I: IntoIterator<Item = V>,
                V: Into<sea_orm::Value>,
            {
                use sea_orm::{QueryFilter, ColumnTrait};
                Self::find_perm().await
                    .filter(Column::Id.is_in(ids))
                    .all(db)
                    .await
            }

            pub async fn find_by_ids_perm_with_tenant<C, I, V>(db: &C, ids: I) -> Result<Vec<Model>, sea_orm::DbErr>
            where
                C: sea_orm::ConnectionTrait,
                I: IntoIterator<Item = V>,
                V: Into<sea_orm::Value>,
            {
                use sea_orm::{QueryFilter, ColumnTrait};
                Self::find_perm_with_tenant().await
                    .filter(Column::Id.is_in(ids))
                    .all(db)
                    .await
            }
        }
    })
}

/// 自动实现 ActiveModelBehavior 的 before_save 方法（通用版本）
///
/// 该宏会自动处理以下字段：
/// - 如果存在 `id` 字段：自动生成ID
/// - 如果存在 `password` 字段：自动哈希密码
/// - 如果存在 `create_time` 字段：设置创建时间
/// - 如果存在 `update_time` 字段：设置更新时间
/// - 如果存在 `creator` 字段：设置创建人
/// - 如果存在 `updater` 字段：设置更新人
#[proc_macro_derive(DaoyiActiveModelBehavior)]
pub fn derive_active_model_behavior(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Check if the struct has a field named "password"
    let has_password = if let Data::Struct(data) = &input.data {
        match &data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .any(|f| f.ident.as_ref().map(|i| i == "password").unwrap_or(false)),
            _ => false,
        }
    } else {
        false
    };

    let password_logic = if has_password {
        quote! {
            self.password = Set(hash_password(self.password.as_ref())
                .await
                .map_err(|e| sea_orm::DbErr::Custom(e.to_string()))?);
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        impl ActiveModel {
             pub async fn set_default_values(&mut self, insert: bool) -> Result<(), sea_orm::DbErr> {
                use sea_orm::Set;
                use daoyi_common_support::id_util;
                use daoyi_common_support::password::hash_password;
                use daoyi_common_support::context::HttpRequestContext;
                use sea_orm::sqlx::types::chrono::Local;

                if insert {
                    self.id = Set(id_util::next_string());
                    #password_logic
                    self.create_time = Set(Local::now().naive_local());
                    self.update_time = Set(Local::now().naive_local());
                                        if let Ok(login_id) = HttpRequestContext::get_login_id_as_string() {
                        self.creator = Set(Some(login_id.clone()));
                        self.updater = Set(Some(login_id));
                    }
                    if let Ok(tenant_id) = HttpRequestContext::get_tenant_id_as_string() {
                        self.tenant_id = Set(tenant_id);
                    }
                } else {
                    self.update_time = Set(Local::now().naive_local());
                                        if let Ok(login_id) = HttpRequestContext::get_login_id_as_string() {
                        self.updater = Set(Some(login_id));
                    }
                }
                Ok(())
             }
        }

        #[sea_orm::prelude::async_trait::async_trait]
        impl ActiveModelBehavior for ActiveModel {
            async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
            where
                C: ConnectionTrait,
            {
                self.set_default_values(insert).await?;
                Ok(self)
            }
        }

        impl Entity {
            pub async fn insert_many_auto<C, I>(db: &C, models: I) -> Result<sea_orm::InsertResult<ActiveModel>, sea_orm::DbErr>
            where
                C: sea_orm::ConnectionTrait,
                I: IntoIterator<Item = ActiveModel>,
            {
                let mut models_vec: Vec<ActiveModel> = models.into_iter().collect();
                for model in &mut models_vec {
                    model.set_default_values(true).await?;
                }
                <Self as sea_orm::EntityTrait>::insert_many(models_vec).exec(db).await
            }

            pub async fn update_many_auto() -> sea_orm::UpdateMany<Entity> {
                use sea_orm::{EntityTrait, sea_query::Expr};
                use daoyi_common_support::context::HttpRequestContext;
                use sea_orm::sqlx::types::chrono::Local;

                let mut query = <Self as sea_orm::EntityTrait>::update_many()
                    .col_expr(Column::UpdateTime, Expr::value(Local::now().naive_local()));

                                    if let Ok(login_id) = HttpRequestContext::get_login_id_as_string() {
                    query = query.col_expr(Column::Updater, Expr::value(login_id));
                }

                query
            }

            pub async fn delete_logical_by_id<C, T>(db: &C, id: T) -> Result<sea_orm::UpdateResult, sea_orm::DbErr>
            where
                C: sea_orm::ConnectionTrait,
                T: Into<sea_orm::Value>,
            {
                use sea_orm::{EntityTrait, sea_query::Expr, ColumnTrait, QueryFilter};
                Self::update_many_auto().await
                    .col_expr(Column::Deleted, Expr::value(true))
                    .filter(Column::Id.eq(id))
                    .exec(db)
                    .await
            }

            pub async fn batch_delete_logical_by_id<C, I, V>(db: &C, ids: I) -> Result<sea_orm::UpdateResult, sea_orm::DbErr>
            where
                C: sea_orm::ConnectionTrait,
                I: IntoIterator<Item = V>,
                V: Into<sea_orm::Value>,
            {
                use sea_orm::{EntityTrait, sea_query::Expr, ColumnTrait, QueryFilter};
                Self::update_many_auto().await
                    .col_expr(Column::Deleted, Expr::value(true))
                    .filter(Column::Id.is_in(ids))
                    .exec(db)
                    .await
            }
        }
    };

    TokenStream::from(expanded)
}

/// 支持自定义属性的 ActiveModelBehavior 实现
///
/// 支持的字段属性：
/// - `#[auto_id]`: 在插入时自动生成ID
/// - `#[hash_password]`: 在插入时自动哈希密码
///
/// # 示例
///
/// ```rust
/// #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
/// #[sea_orm(schema_name = "demo", table_name = "sys_user")]
/// #[derive(BeforeInsert)]
/// pub struct Model {
///     #[sea_orm(primary_key, auto_increment = false)]
///     #[auto_id]
///     pub id: String,
///     
///     #[hash_password]
///     pub password: String,
/// }
/// ```
#[proc_macro_derive(BeforeInsert, attributes(auto_id, hash_password))]
pub fn derive_before_insert(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // 解析结构体字段
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("BeforeInsert only supports structs with named fields"),
        },
        _ => panic!("BeforeInsert only supports structs"),
    };

    // 收集需要自动生成ID的字段
    let mut auto_id_fields = Vec::new();
    // 收集需要哈希密码的字段
    let mut hash_password_fields = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();

        // 检查是否有 auto_id 或 hash_password 属性
        for attr in &field.attrs {
            if attr.path().is_ident("auto_id") {
                auto_id_fields.push(field_name);
            }
            if attr.path().is_ident("hash_password") {
                hash_password_fields.push(field_name);
            }
        }
    }

    // 生成 before_save 方法体
    let mut insert_statements = Vec::new();

    // 生成 ID 自动生成代码
    for field in &auto_id_fields {
        insert_statements.push(quote! {
            self.#field = sea_orm::Set(daoyi_common_support::id_util::next_string());
        });
    }

    // 生成密码哈希代码
    for field in &hash_password_fields {
        insert_statements.push(quote! {
            self.#field = sea_orm::Set(
                daoyi_common_support::password::hash_password(self.#field.as_ref())
                    .await
                    .map_err(|e| sea_orm::DbErr::Custom(e.to_string()))?
            );
        });
    }

    // 如果没有任何自动处理的字段，则生成空实现
    let before_save_impl = if insert_statements.is_empty() {
        quote! {
            #[sea_orm::prelude::async_trait::async_trait]
            impl sea_orm::ActiveModelBehavior for ActiveModel {}
        }
    } else {
        quote! {
            #[sea_orm::prelude::async_trait::async_trait]
            impl sea_orm::ActiveModelBehavior for ActiveModel {
                async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, sea_orm::DbErr>
                where
                    C: sea_orm::ConnectionTrait,
                {
                    if insert {
                        #(#insert_statements)*
                    }
                    Ok(self)
                }
            }
        }
    };

    TokenStream::from(before_save_impl)
}

/// 自动实现 IntoActiveValue trait
///
/// 该宏为枚举自动实现 `IntoActiveValue<T>`，将枚举值包装为 `ActiveValue::Set(self)`。
///
/// # 示例
///
/// ```rust
/// #[derive(DaoyiIntoActiveValue)]
/// pub enum Gender {
///     Male,
///     Female,
/// }
/// ```
#[proc_macro_derive(DaoyiIntoActiveValue)]
pub fn derive_daoyi_into_active_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl sea_orm::IntoActiveValue<#name> for #name {
            fn into_active_value(self) -> sea_orm::ActiveValue<#name> {
                sea_orm::ActiveValue::Set(self)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(DaoyiStringOrNumberSerde)]
pub fn derive_string_or_number_serde(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("DaoyiStringOrNumberSerde only supports enums"),
    };

    let mut serialize_arms = Vec::new();
    let mut deserialize_string_arms = Vec::new();
    let mut deserialize_number_arms = Vec::new();
    let mut valid_string_values = Vec::new();

    for variant in variants {
        let variant_name = &variant.ident;

        // 从 #[sea_orm(string_value = "...")] 中提取值
        let mut string_value = None;
        for attr in &variant.attrs {
            if attr.path().is_ident("sea_orm") {
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("string_value") {
                        let value: syn::LitStr = meta.value()?.parse()?;
                        string_value = Some(value.value());
                    }
                    Ok(())
                });
            }
        }

        let value = string_value.expect(&format!(
            "Variant {} must have #[sea_orm(string_value = \"...\")] attribute",
            variant_name
        ));

        valid_string_values.push(value.clone());

        serialize_arms.push(quote! {
            #name::#variant_name => serializer.serialize_str(#value),
        });

        deserialize_string_arms.push(quote! {
            #value => Ok(#name::#variant_name),
        });

        // 尝试解析为数字用于数字匹配
        if let Ok(num) = value.parse::<i32>() {
            deserialize_number_arms.push(quote! {
                #num => Ok(#name::#variant_name),
            });
        }
    }

    let valid_values_str = valid_string_values.join(", ");
    let valid_values_array: Vec<_> = valid_string_values.iter().map(|s| s.as_str()).collect();

    let expanded = quote! {
        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    #(#serialize_arms)*
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(serde::Deserialize)]
                #[serde(untagged)]
                enum StringOrNumber {
                    String(String),
                    Number(i32),
                }

                match StringOrNumber::deserialize(deserializer)? {
                    StringOrNumber::String(s) => match s.as_str() {
                        #(#deserialize_string_arms)*
                        _ => Err(serde::de::Error::unknown_variant(
                            &s,
                            &[#(#valid_values_array),*],
                        )),
                    },
                    StringOrNumber::Number(n) => match n {
                        #(#deserialize_number_arms)*
                        _ => Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Signed(n as i64),
                            &#valid_values_str,
                        )),
                    },
                }
            }
        }
    };

    TokenStream::from(expanded)
}
