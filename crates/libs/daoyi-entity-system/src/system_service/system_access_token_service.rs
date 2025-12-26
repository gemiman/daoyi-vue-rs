use crate::system_entity::prelude::*;
use crate::system_entity::system_access_token;
use daoyi_common_support::configs::AppConfig;
use daoyi_common_support::context::HttpRequestContext;
use daoyi_common_support::database;
use daoyi_common_support::vo::system_vo::AuthLoginRespVO;
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::chrono::Local;

pub async fn get_access_token(token: &str) -> anyhow::Result<system_access_token::Model> {
    let db = database::get().await;
    let option = SystemAccessToken::find()
        .filter(system_access_token::Column::AccessToken.eq(token))
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Token不存在"))?;
    Ok(option)
}

pub async fn create_token_after_login_success(
    tenant_id: &str,
    login_id: &str,
) -> anyhow::Result<AuthLoginRespVO> {
    let access_token = loop {
        let token = xid::new().to_string();
        if let Err(_) = get_access_token(&token).await {
            break token;
        }
    };
    let mut context = HttpRequestContext::new();
    context.token = Some(access_token.clone());
    context.login_id = Some(String::from(login_id));
    context.tenant_id = Some(String::from(tenant_id));
    HttpRequestContext::set_current(context);
    let token_expiration = AppConfig::get().await.auth().token_expiration();
    let db = database::get().await;
    let mut active_model = system_access_token::ActiveModel::new();
    active_model.user_id = Set(String::from(login_id));
    active_model.access_token = Set(access_token);
    active_model.expires_time = Set(Local::now().naive_local() + token_expiration);
    let model = active_model.insert(db).await?;
    Ok(AuthLoginRespVO {
        tenant_id: model.tenant_id,
        user_id: model.user_id,
        access_token: model.access_token,
        expires_time: model.expires_time,
    })
}
