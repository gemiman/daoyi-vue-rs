use axum::http::header;
use merge::Merge;
use serde::Deserialize;
use std::time::Duration;
use wax::{Glob, Pattern};

#[derive(Debug, Deserialize, Default, Merge)]
pub struct AuthConfig {
    #[merge(strategy = merge::option::overwrite_none)]
    ignored_urls: Option<Vec<String>>,
    #[merge(strategy = merge::option::overwrite_none)]
    tenant_ignored_urls: Option<Vec<String>>,
    #[merge(strategy = merge::option::overwrite_none)]
    header_key_token: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    header_key_tenant: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    token_expiration: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    token_check_url: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    tenant_check_url: Option<String>,
}
impl AuthConfig {
    pub fn header_key_token(&self) -> &str {
        self.header_key_token
            .as_deref()
            .unwrap_or(header::AUTHORIZATION.as_str())
    }
    pub fn header_key_tenant(&self) -> &str {
        self.header_key_tenant.as_deref().unwrap_or("tenant-id")
    }

    pub fn is_ignored_auth(&self, url: &str) -> bool {
        self.ignored_urls.is_some()
            && path_any_matches(&self.ignored_urls.as_deref().unwrap(), url).unwrap_or(false)
    }
    pub fn is_ignored_tenant(&self, url: &str) -> bool {
        self.tenant_ignored_urls.is_some()
            && path_any_matches(&self.tenant_ignored_urls.as_deref().unwrap(), url).unwrap_or(false)
    }
    pub fn token_expiration(&self) -> Duration {
        if let Some(token_expiration) = &self.token_expiration {
            return humantime::parse_duration(token_expiration)
                .unwrap_or(Duration::from_secs(3600 * 12));
        }
        Duration::from_secs(3600 * 12)
    }
    pub fn token_check_url(&self) -> &str {
        self.token_check_url.as_deref().unwrap_or("http://127.0.0.1:48001/admin-api/system/auth/login")
    }
    pub fn tenant_check_url(&self) -> &str {
        self.tenant_check_url.as_deref().unwrap_or("")
    }
}

fn path_matches(pattern: &str, target: &str) -> anyhow::Result<bool> {
    // 将通配符模式编译为 Glob 表达式
    let glob = Glob::new(pattern)?;
    // 判断目标路径是否匹配该模式
    Ok(glob.is_match(target))
}

fn path_any_matches<A: AsRef<str>>(patterns: &[A], target: &str) -> anyhow::Result<bool> {
    for pattern in patterns {
        if path_matches(pattern.as_ref(), target)? {
            return Ok(true);
        }
    }
    Ok(false)
}
