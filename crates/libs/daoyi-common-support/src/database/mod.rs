use crate::configs::AppConfig;
use anyhow::{Context, anyhow};
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement,
    DatabaseTransaction, TransactionTrait, DbErr, ExecResult, QueryResult, TransactionError
};
use std::cmp::max;
use std::time::Duration;
use tokio::sync::OnceCell;
use std::sync::Arc;
use std::pin::Pin;
use std::future::Future;

static DB_CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub type TxRef = Arc<DatabaseTransaction>;
tokio::task_local! {
    static TX_CONTEXT: TxRef;
}

#[derive(Clone)]
pub enum DbGuard {
    Global(&'static DatabaseConnection),
    Tx(TxRef),
}

#[sea_orm::prelude::async_trait::async_trait]
impl ConnectionTrait for DbGuard {
    fn get_database_backend(&self) -> DbBackend {
        match self {
            DbGuard::Global(db) => db.get_database_backend(),
            DbGuard::Tx(tx) => tx.get_database_backend(),
        }
    }

    async fn execute(&self, stmt: Statement) -> Result<ExecResult, DbErr> {
        match self {
            DbGuard::Global(db) => db.execute(stmt).await,
            DbGuard::Tx(tx) => tx.execute(stmt).await,
        }
    }

    async fn execute_unprepared(&self, sql: &str) -> Result<ExecResult, DbErr> {
        match self {
            DbGuard::Global(db) => db.execute_unprepared(sql).await,
            DbGuard::Tx(tx) => tx.execute_unprepared(sql).await,
        }
    }

    async fn query_one(&self, stmt: Statement) -> Result<Option<QueryResult>, DbErr> {
        match self {
            DbGuard::Global(db) => db.query_one(stmt).await,
            DbGuard::Tx(tx) => tx.query_one(stmt).await,
        }
    }

    async fn query_all(&self, stmt: Statement) -> Result<Vec<QueryResult>, DbErr> {
        match self {
            DbGuard::Global(db) => db.query_all(stmt).await,
            DbGuard::Tx(tx) => tx.query_all(stmt).await,
        }
    }
}

impl DbGuard {
    pub async fn begin(&self) -> Result<DatabaseTransaction, DbErr> {
         match self {
            DbGuard::Global(db) => db.begin().await,
            DbGuard::Tx(tx) => tx.begin().await,
        }
    }

    pub async fn transaction<F, T, E>(&self, callback: F) -> Result<T, E>
    where
        F: for<'c> FnOnce(&'c DatabaseTransaction) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'c>> + Send,
        T: Send,
        E: From<DbErr> + std::fmt::Display + std::fmt::Debug + Send,
    {
         let res = match self {
            DbGuard::Global(db) => db.transaction(callback).await,
            DbGuard::Tx(tx) => tx.transaction(callback).await,
        };
        match res {
            Ok(v) => Ok(v),
            Err(TransactionError::Connection(e)) => Err(E::from(e)),
            Err(TransactionError::Transaction(e)) => Err(e),
        }
    }
}

pub fn get_db() -> DbGuard {
    if let Ok(tx) = TX_CONTEXT.try_with(|v| v.clone()) {
        DbGuard::Tx(tx)
    } else {
        let db = DB_CONN.get().expect("Database not initialized");
        DbGuard::Global(db)
    }
}

pub async fn get_db_async() -> DbGuard {
    get_db()
}

pub async fn call_in_transaction<T, E, F>(func: F) -> Result<T, E>
where
    F: Future<Output = Result<T, E>>,
    E: From<DbErr>,
{
    if TX_CONTEXT.try_with(|_| ()).is_ok() {
        return func.await;
    }

    let db = get().await;
    let txn = db.begin().await?;
    let txn_arc = Arc::new(txn);

    let result = TX_CONTEXT.scope(txn_arc.clone(), func).await;

    match result {
        Ok(v) => {
            match Arc::try_unwrap(txn_arc) {
                Ok(txn) => {
                    txn.commit().await.map_err(Into::into)?;
                    Ok(v)
                }
                Err(_) => {
                     Err(DbErr::Custom("Transaction reference leak: cannot commit".into()).into())
                }
            }
        },
        Err(e) => {
            match Arc::try_unwrap(txn_arc) {
                Ok(txn) => {
                     txn.rollback().await.map_err(Into::into)?;
                     Err(e)
                },
                Err(_) => {
                    Err(e)
                }
            }
        }
    }
}

pub async fn init_db() -> anyhow::Result<()> {
    let database_config = AppConfig::get().database();
    let mut options = ConnectOptions::new(format!(
        "postgres://{}:{}@{}:{}/{}",
        database_config.username,
        database_config.password,
        database_config.host,
        database_config.port,
        database_config.database
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
async fn get() -> &'static DatabaseConnection {
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
