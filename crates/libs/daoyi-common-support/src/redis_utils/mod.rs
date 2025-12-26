use crate::configs::AppConfig;
use crate::error::ApiResult;
use crate::id;
use deadpool_redis::redis::{AsyncCommands, FromRedisValue, ToRedisArgs};
use deadpool_redis::{Config, Connection, Pool, Runtime};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::OnceCell;

static REDIS: OnceCell<Pool> = OnceCell::const_new();

const CONNECTION_TEST_KEY: &str = "connection_test_key";

async fn init() -> anyhow::Result<Pool> {
    let redis_config = AppConfig::get().await.redis();
    let host = redis_config.host();
    let port = redis_config.port();
    let db = redis_config.database();
    let passwd = redis_config.password();

    let url = if passwd.is_empty() {
        format!("redis://{host}:{port}/{db}")
    } else {
        format!("redis://:{passwd}@{host}:{port}/{db}")
    };

    let cfg = Config::from_url(url);
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;

    // 测试连接
    let mut conn = pool.get().await?;
    let _: () = conn.set(CONNECTION_TEST_KEY, id::next_string()).await?;
    let val: String = conn.get(CONNECTION_TEST_KEY).await?;

    tracing::info!("Redis connected successfully, {CONNECTION_TEST_KEY} = {val}");
    Ok(pool)
}

/// 初始化Redis客户端
pub async fn init_redis() -> anyhow::Result<()> {
    REDIS.get_or_try_init(|| init()).await?;
    Ok(())
}

/// 获取Redis连接池实例
fn get_pool() -> anyhow::Result<&'static Pool> {
    REDIS
        .get()
        .ok_or_else(|| anyhow::anyhow!("Redis pool not initialized"))
}

/// 测试Redis连接
pub async fn test_redis() -> anyhow::Result<String> {
    let v: String = get(CONNECTION_TEST_KEY).await?;
    tracing::info!("Redis test success...{CONNECTION_TEST_KEY}={v}");
    Ok(v)
}

async fn key_generator(key: &str) -> String {
    let cache_key_prefix = AppConfig::get().await.redis().cache_key_prefix();
    format!("{}:{}", cache_key_prefix, key)
}
pub async fn cache_get_json<V>(key: &str) -> ApiResult<Option<V>>
where
    V: DeserializeOwned,
{
    let json_str = get::<Option<String>>(key_generator(key).await.as_ref()).await?;
    if json_str.is_none() {
        return Ok(None);
    }
    let json_str = json_str.unwrap();
    Ok(serde_json::from_str(json_str.as_ref())?)
}

pub async fn cache_set_json<V>(key: &str, value: &V) -> ApiResult<()>
where
    V: Serialize,
{
    let json_str = serde_json::to_string(value)?;
    cache_set(key_generator(key).await.as_ref(), json_str).await
}
pub async fn cache_set_json_ex<V>(key: &str, value: &V, expire_seconds: u64) -> ApiResult<()>
where
    V: Serialize,
{
    let json_str = serde_json::to_string(value)?;
    cache_set_ex(key_generator(key).await.as_ref(), json_str, expire_seconds).await
}

pub async fn cache_get<V>(key: &str) -> ApiResult<Option<V>>
where
    V: FromRedisValue + Send + Sync + 'static,
{
    let value = get(key_generator(key).await.as_ref()).await?;
    Ok(value)
}
pub async fn cache_set<V>(key: &str, value: V) -> ApiResult<()>
where
    V: ToRedisArgs + Send + Sync + 'static,
{
    let expire_seconds = AppConfig::get().await.redis().expire_seconds();
    cache_set_ex(key, value, expire_seconds).await
}

pub async fn cache_set_ex<V>(key: &str, value: V, expire_seconds: u64) -> ApiResult<()>
where
    V: ToRedisArgs + Send + Sync + 'static,
{
    set_ex(key_generator(key).await.as_ref(), value, expire_seconds).await?;
    Ok(())
}

/// 获取Redis中指定键的值
///
/// # 参数
/// * `key` - 要获取的键
///
/// # 返回值
/// 返回键对应的值，如果键不存在则返回错误
#[allow(dead_code)]
pub async fn get<T: FromRedisValue>(key: &str) -> ApiResult<T> {
    let pool = get_pool()?;
    let mut conn = pool.get().await?;
    let result = conn.get(key).await?;
    Ok(result)
}

/// 设置键值对并指定过期时间
///
/// # 参数
/// * `key` - 键
/// * `value` - 值
/// * `seconds` - 过期时间（秒）
#[allow(dead_code)]
pub async fn set_ex<V>(key: &str, value: V, seconds: u64) -> ApiResult<()>
where
    V: ToRedisArgs + Send + Sync + 'static,
{
    let pool = get_pool()?;
    let mut conn = pool.get().await?;
    let _: () = conn.set_ex(key, value, seconds).await?;
    Ok(())
}

/// 设置键值对
///
/// # 参数
/// * `key` - 键
/// * `value` - 值
#[allow(dead_code)]
pub async fn set<V>(key: &str, value: V) -> ApiResult<()>
where
    V: ToRedisArgs + Send + Sync + 'static,
{
    let pool = get_pool()?;
    let mut conn = pool.get().await?;
    let _: () = conn.set(key, value).await?;
    Ok(())
}

/// 删除指定键
///
/// # 参数
/// * `key` - 要删除的键
#[allow(dead_code)]
pub async fn del(key: &str) -> ApiResult<()> {
    let pool = get_pool()?;
    let mut conn = pool.get().await?;
    let _: () = conn.del(key).await?;
    Ok(())
}

/// 检查键是否存在
///
/// # 参数
/// * `key` - 要检查的键
pub async fn exists(key: &str) -> ApiResult<bool> {
    let pool = get_pool()?;
    let mut conn = pool.get().await?;
    let result = conn.exists(key).await?;
    Ok(result)
}

/// 设置带TTL的键值对
///
/// # 参数
/// * `key` - 键
/// * `value` - 值
/// * `seconds` - 过期时间（秒）
pub async fn set_with_expire<V>(key: &str, value: V, seconds: u64) -> ApiResult<()>
where
    V: ToRedisArgs + Send + Sync + 'static,
{
    let pool = get_pool()?;
    let mut conn = pool.get().await?;
    let _: () = conn.set_ex(key, value, seconds).await?;
    Ok(())
}

/// 获取Redis的原始连接
///
/// # 返回值
/// 返回一个Redis连接
pub async fn raw_connection() -> ApiResult<Connection> {
    let pool = get_pool()?;
    let conn = pool.get().await?;
    Ok(conn)
}
