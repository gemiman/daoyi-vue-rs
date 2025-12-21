use merge::Merge;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Merge)]
pub struct DatabaseConfig {
    #[merge(strategy = merge::option::overwrite_none)]
    driver: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    host: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    port: Option<u16>,
    #[merge(strategy = merge::option::overwrite_none)]
    user: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    password: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    database: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    schema: Option<String>,
}

impl DatabaseConfig {
    pub fn driver(&self) -> &str {
        self.driver.as_deref().unwrap_or("postgres")
    }
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("127.0.0.1")
    }
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(5432)
    }
    pub fn user(&self) -> &str {
        self.user.as_deref().unwrap_or("postgres")
    }
    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("postgres")
    }
    pub fn database(&self) -> &str {
        self.database.as_deref().unwrap_or("postgres")
    }
    pub fn schema(&self) -> &str {
        self.schema.as_deref().unwrap_or("public")
    }
}
