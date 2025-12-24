use merge::Merge;
use nacos_sdk::api::props::ClientProps;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default, Merge)]
pub struct NacosConfig {
    #[merge(strategy = merge::option::overwrite_none)]
    enable: Option<bool>,
    #[merge(strategy = merge::option::overwrite_none)]
    server_addr: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    namespace: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    app_name: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    group: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    auth_username: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    auth_password: Option<String>,
}

impl NacosConfig {
    pub fn enable(&self) -> bool {
        self.enable.unwrap_or(false)
    }
    pub fn server_addr(&self) -> &str {
        &self.server_addr.as_deref().unwrap_or("127.0.0.1:8848")
    }
    pub fn namespace(&self) -> &str {
        &self.namespace.as_deref().unwrap_or("public")
    }
    pub fn app_name(&self) -> &str {
        &self.app_name.as_deref().unwrap_or("app")
    }
    pub fn group(&self) -> &str {
        &self.group.as_deref().unwrap_or("DEFAULT_GROUP")
    }
    pub fn auth_username(&self) -> &str {
        &self.auth_username.as_deref().unwrap_or("nacos")
    }
    pub fn auth_password(&self) -> &str {
        &self.auth_password.as_deref().unwrap_or("nacos")
    }
}

impl Into<ClientProps> for &NacosConfig {
    fn into(self) -> ClientProps {
        ClientProps::new()
            .server_addr(self.server_addr())
            .namespace(self.namespace())
            .app_name(self.app_name())
            .auth_username(self.auth_username())
            .auth_password(self.auth_password())
    }
}
