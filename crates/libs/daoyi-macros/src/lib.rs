use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

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
                .map_err(|e| DbErr::Custom(e.to_string()))?);
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        #[sea_orm::prelude::async_trait::async_trait]
        impl ActiveModelBehavior for ActiveModel {
            async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
            where
                C: ConnectionTrait,
            {
                use sea_orm::Set;
                use daoyi_common_support::id;
                use daoyi_common_support::password::hash_password;
                use daoyi_common_support::context::HttpRequestContext;
                use sea_orm::sqlx::types::chrono::Local;

                if insert {
                    self.id = Set(id::next_string());
                    #password_logic
                    self.create_time = Set(Local::now().naive_local());
                    self.update_time = Set(Local::now().naive_local());
                    if let Ok(login_id) = HttpRequestContext::get_login_id_as_string().await {
                        self.creator = Set(Some(login_id.clone()));
                        self.updater = Set(Some(login_id));
                    }
                } else {
                    self.update_time = Set(Local::now().naive_local());
                    if let Ok(login_id) = HttpRequestContext::get_login_id_as_string().await {
                        self.updater = Set(Some(login_id));
                    }
                }
                Ok(self)
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
            self.#field = sea_orm::Set(daoyi_common_support::id::next_string());
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