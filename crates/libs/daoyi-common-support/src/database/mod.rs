use crate::configs::AppConfig;
use sea_orm::{ConnectOptions, DatabaseConnection};

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    // let database_config = AppConfig::get().await.database();
    // let options = ConnectOptions::new(format!(
    //     "{}://{}:{}@{}:{}/{}",
    //     database_config.driver(),
    //     database_config.username(),
    //     database_config.password(),
    //     database_config.host(),
    //     database_config.port(),
    //     database_config.database()
    // ));
    todo!("init database")
}
