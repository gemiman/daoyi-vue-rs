use axum::http::header;
use merge::Merge;
use serde::Deserialize;
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
        self.ignored_urls.is_none()
            || path_any_matches(&self.ignored_urls.as_deref().unwrap(), url).unwrap_or(false)
    }
    pub fn is_ignored_tenant(&self, url: &str) -> bool {
        self.tenant_ignored_urls.is_none()
            || path_any_matches(&self.tenant_ignored_urls.as_deref().unwrap(), url).unwrap_or(false)
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
