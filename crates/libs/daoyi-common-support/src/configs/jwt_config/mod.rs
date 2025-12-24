use merge::Merge;
use serde::Deserialize;
use std::time::Duration;

const DEFAULT_SECRET: &str = r#"2234!QW@#ESDX234GVYBHKJU@234#$WEBHJ@#WSEDRCFrdcftghuyj"#;
#[derive(Debug, Deserialize, Default, Merge)]
pub struct JwtConfig {
    #[merge(strategy = merge::option::overwrite_none)]
    secret: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    expiration: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    audience: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    issuer: Option<String>,
}
impl JwtConfig {
    pub fn secret(&self) -> &str {
        self.secret.as_deref().unwrap_or(DEFAULT_SECRET)
    }
    pub fn expiration(&self) -> Duration {
        if let Some(expiration) = &self.expiration {
            return humantime::parse_duration(expiration).unwrap_or(Duration::from_secs(120));
        }
        Duration::from_secs(60 * 60)
    }
    pub fn audience(&self) -> &str {
        self.audience.as_deref().unwrap_or("audience")
    }
    pub fn issuer(&self) -> &str {
        self.issuer.as_deref().unwrap_or("issuer")
    }
}
