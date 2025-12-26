use crate::configs::AppConfig;
use anyhow::{Context, anyhow};
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement,
};
use std::cmp::max;
use std::time::Duration;
use tokio::sync::OnceCell;

static DB_CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn init() -> anyhow::Result<()> {
    if DB_CONN.get().is_some() {
        return Ok(());
    }
    let cpus = num_cpus::get() as u32;
    let database_config = AppConfig::get().await.database();
    let mut options = ConnectOptions::new(format!(
        "{}://{}:{}@{}:{}/{}",
        database_config.driver(),
        database_config.user(),
        database_config.password(),
        database_config.host(),
        database_config.port(),
        database_config.database()
    ));
    options
        .min_connections(max(cpus * 4, 10))
        .max_connections(max(cpus * 8, 20))
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(300))
        .sqlx_logging(false)
        .set_schema_search_path(database_config.schema());
    let db = Database::connect(options).await?;
    db.ping().await?;
    tracing::info!("Database connection successful");
    log_database_version(&db).await?;
    DB_CONN
        .set(db)
        .with_context(|| anyhow!("Failed to set database config"))?;
    Ok(())
}
pub async fn get() -> &'static DatabaseConnection {
    DB_CONN
        .get()
        .unwrap_or_else(|| panic!("Failed to load database config"))
}

/// 关闭数据库连接池
/// 注意: SeaORM 的 DatabaseConnection 会在 Drop 时自动关闭连接
/// 这个函数主要用于显式日志记录，实际的连接关闭会在程序退出时自动完成
pub async fn shutdown() -> anyhow::Result<()> {
    tracing::info!("Database connection pool will be closed on application exit");
    // SeaORM 的 DatabaseConnection 实现了 Drop trait
    // 当程序退出时会自动关闭所有连接
    // 这里我们只记录日志，不需要手动关闭
    Ok(())
}
async fn log_database_version(db: &DatabaseConnection) -> anyhow::Result<()> {
    let version_result = db
        .query_one(Statement::from_string(
            DbBackend::Postgres,
            "SELECT VERSION()",
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("Database version unknown"))?;
    tracing::info!(
        "Database version: {}",
        version_result.try_get_by_index::<String>(0)?
    );
    Ok(())
}
