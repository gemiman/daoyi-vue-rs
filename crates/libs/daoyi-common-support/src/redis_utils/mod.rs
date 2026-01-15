use crate::configs::AppConfig;
use crate::error::ApiResult;
use crate::id_util;
use crate::vo::MqMsgBody;
use futures::StreamExt;
use redis::aio::ConnectionManager;
use redis::streams::{StreamReadOptions, StreamReadReply};
use redis::{AsyncCommands, Client, FromRedisValue, ToRedisArgs, ToSingleRedisArg};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::sync::OnceCell;

static REDIS: OnceCell<ConnectionManager> = OnceCell::const_new();

const CONNECTION_TEST_KEY: &str = "connection_test_key";

fn get_redis_url() -> String {
    let redis_config = AppConfig::get().redis();
    let host = redis_config.host();
    let port = redis_config.port();
    let db = redis_config.database();
    let passwd = redis_config.password();

    if passwd.is_empty() {
        format!("redis://{host}:{port}/{db}")
    } else {
        format!("redis://:{passwd}@{host}:{port}/{db}")
    }
}

async fn init() -> anyhow::Result<ConnectionManager> {
    let url = get_redis_url();
    let client = Client::open(url)?;
    // 使用 ConnectionManager，它会自动处理重连
    let mut mgr = client.get_connection_manager().await?;

    // 测试连接
    let _: () = mgr.set(CONNECTION_TEST_KEY, id_util::next_string()).await?;
    let val: String = mgr.get(CONNECTION_TEST_KEY).await?;

    tracing::info!("Redis connected successfully, {CONNECTION_TEST_KEY} = {val}");
    Ok(mgr)
}

/// 初始化Redis客户端
pub async fn init_redis() -> anyhow::Result<()> {
    REDIS.get_or_try_init(|| init()).await?;
    Ok(())
}

/// 获取 ConnectionManager 实例
/// ConnectionManager 是廉价克隆的，每次调用返回一个新的克隆，共享底层的连接处理逻辑
fn get_manager() -> ApiResult<ConnectionManager> {
    REDIS
        .get()
        .map(|mgr| mgr.clone())
        .ok_or_else(|| anyhow::anyhow!("Redis not initialized").into())
}

pub async fn send_mq_msg<T>(body: &MqMsgBody<T>) -> ApiResult<String>
where
    T: Serialize,
{
    let json = serde_json::to_string(body)?;
    stream_publish(&body.topic, &json).await
}

/// 发布消息到 Redis Stream (使用独立连接)
async fn stream_publish(key: &str, value: &str) -> ApiResult<String> {
    // Stream 操作可以使用 Manager，因为不需要阻塞或事务
    let mut conn = get_manager()?;
    // 自动生成 ID (*)，存入 key-value 对: payload -> value
    let id: String = conn.xadd(key, "*", &[("payload", value)]).await?;
    Ok(id)
}

/// 消费 Redis Stream (消费者组模式，使用独立连接)
/// 该函数会无限循环，直到发生严重错误
pub async fn consume_stream<F, Fut>(
    stream_key: &str,
    group_name: &str,
    consumer_name: &str,
    f: F,
) -> anyhow::Result<()>
where
    F: Fn(String) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = anyhow::Result<()>> + Send + 'static,
{
    // 消费者组需要阻塞读取 (Block)，因此不能使用共享的 ConnectionManager
    // 必须建立独立的专用连接
    let url = get_redis_url();
    let client = Client::open(url)?;

    // 1. 尝试创建消费者组 (忽略 BUSYGROUP 错误)
    {
        let mut conn = client.get_multiplexed_async_connection().await?;
        let result: redis::RedisResult<()> = conn
            .xgroup_create_mkstream(stream_key, group_name, "$")
            .await;
        if let Err(e) = result {
            if !e.to_string().contains("BUSYGROUP") {
                return Err(e.into());
            }
        }
    }

    tracing::info!(
        "Redis Stream consumer started. Stream: {}, Group: {}, Consumer: {}",
        stream_key,
        group_name,
        consumer_name
    );

    let opts = StreamReadOptions::default()
        .group(group_name, consumer_name)
        .block(2000) // 阻塞 2s
        .count(1);

    loop {
        // 在循环中使用独立的连接，如果断开需要重连
        // 这里简化处理：每次循环获取连接，实际上 get_multiplexed_async_connection 建立连接开销不大
        // 但为了性能，最好复用。不过 MultiplexedConnection 断开后需要重新获取。
        let mut conn = match client.get_multiplexed_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Failed to get redis connection: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                continue;
            }
        };

        // 读取消息
        let result: redis::RedisResult<StreamReadReply> =
            conn.xread_options(&[stream_key], &[">"], &opts).await;

        match result {
            Ok(reply) => {
                for stream_key_result in reply.keys {
                    for message in stream_key_result.ids {
                        let msg_id = message.id;
                        // 获取 payload 字段
                        if let Some(val) = message.map.get("payload") {
                            // 使用 FromRedisValue 自动转换
                            let payload: String =
                                match redis::FromRedisValue::from_redis_value(val.clone()) {
                                    Ok(s) => s,
                                    Err(e) => {
                                        tracing::error!("Failed to parse payload string: {}", e);
                                        continue;
                                    }
                                };

                            // 调用回调处理
                            match f(payload).await {
                                Ok(_) => {
                                    // 处理成功，确认消息 (ACK)
                                    let ack_res: redis::RedisResult<()> =
                                        conn.xack(stream_key, group_name, &[&msg_id]).await;
                                    if let Err(e) = ack_res {
                                        tracing::error!("Failed to ACK message {}: {}", msg_id, e);
                                    }
                                }
                                Err(e) => {
                                    tracing::error!(
                                        "Error processing stream message {}: {}",
                                        msg_id,
                                        e
                                    );
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                // 读取超时或网络错误等
                if !e.is_io_error() && !e.is_timeout() {
                    tracing::error!("Error reading from stream: {}", e);
                }
            }
        }
    }
}

/// 订阅 Redis 频道
pub async fn subscribe<F, Fut>(channel: &str, f: F) -> anyhow::Result<()>
where
    F: Fn(String) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    let url = get_redis_url();
    // PubSub 需要独占的连接
    let client = Client::open(url)?;
    // 使用 get_async_pubsub 获取支持 PubSub 的连接
    let mut pubsub = client.get_async_pubsub().await?;
    pubsub.subscribe(channel).await?;
    let mut stream = pubsub.on_message();

    tracing::info!("Redis subscribed to channel: {}", channel);

    while let Some(msg) = stream.next().await {
        let payload_result: redis::RedisResult<String> = msg.get_payload();
        match payload_result {
            Ok(p) => f(p).await,
            Err(e) => {
                tracing::error!("Failed to get payload from redis msg: {}", e);
                continue;
            }
        }
    }
    Ok(())
}

/// 获取Redis的原始连接 (这里返回 ConnectionManager)
pub async fn raw_connection() -> ApiResult<ConnectionManager> {
    get_manager()
}

/// 关闭 Redis 连接池 (不再需要，ConnectionManager 自动管理)
pub async fn shutdown() -> anyhow::Result<()> {
    // No-op for ConnectionManager
    Ok(())
}

/// 测试Redis连接
pub async fn test_redis() -> anyhow::Result<String> {
    let v: String = get(CONNECTION_TEST_KEY).await?;
    tracing::info!("Redis test success...{CONNECTION_TEST_KEY}={v}");
    Ok(v)
}

fn key_generator(key: &str) -> String {
    let cache_key_prefix = AppConfig::get().redis().cache_key_prefix();
    format!("{}:{}", cache_key_prefix, key)
}
pub async fn cache_get_json<V>(key: &str) -> ApiResult<Option<V>>
where
    V: DeserializeOwned,
{
    let json_str = get::<Option<String>>(key_generator(key).as_str()).await?;
    if let Some(json_str) = json_str {
        Ok(serde_json::from_str(json_str.as_ref())?)
    } else {
        Ok(None)
    }
}

pub async fn cache_set_json<V>(key: &str, value: &V) -> ApiResult<()>
where
    V: Serialize,
{
    let json_str = serde_json::to_string(value)?;
    cache_set(key, json_str).await
}
pub async fn cache_set_json_ex<V>(key: &str, value: &V, expire_seconds: u64) -> ApiResult<()>
where
    V: Serialize,
{
    let json_str = serde_json::to_string(value)?;
    cache_set_ex(key, json_str, expire_seconds).await
}

pub async fn cache_get<V>(key: &str) -> ApiResult<Option<V>>
where
    V: FromRedisValue + Send + Sync + 'static,
{
    let value = get(key_generator(key).as_ref()).await?;
    Ok(value)
}
pub async fn cache_set<V>(key: &str, value: V) -> ApiResult<()>
where
    V: ToRedisArgs + ToSingleRedisArg + Send + Sync + 'static,
{
    let expire_seconds = AppConfig::get().redis().expire_seconds();
    cache_set_ex(key, value, expire_seconds).await
}

pub async fn cache_set_ex<V>(key: &str, value: V, expire_seconds: u64) -> ApiResult<()>
where
    V: ToRedisArgs + ToSingleRedisArg + Send + Sync + 'static,
{
    set_ex(key_generator(key).as_ref(), value, expire_seconds).await?;
    Ok(())
}

/// 获取Redis中指定键的值
#[allow(dead_code)]
pub async fn get<T: FromRedisValue>(key: &str) -> ApiResult<T> {
    let mut conn = get_manager()?;
    let result = conn.get(key).await?;
    Ok(result)
}

/// 设置键值对并指定过期时间
#[allow(dead_code)]
pub async fn set_ex<V>(key: &str, value: V, seconds: u64) -> ApiResult<()>
where
    V: ToRedisArgs + ToSingleRedisArg + Send + Sync + 'static,
{
    let mut conn = get_manager()?;
    let _: () = conn.set_ex(key, value, seconds).await?;
    Ok(())
}

/// 设置键值对
#[allow(dead_code)]
pub async fn set<V>(key: &str, value: V) -> ApiResult<()>
where
    V: ToRedisArgs + ToSingleRedisArg + Send + Sync + 'static,
{
    let mut conn = get_manager()?;
    let _: () = conn.set(key, value).await?;
    Ok(())
}

/// 删除指定键
#[allow(dead_code)]
pub async fn del(key: &str) -> ApiResult<()> {
    let mut conn = get_manager()?;
    let _: () = conn.del(key).await?;
    Ok(())
}

/// 检查键是否存在
pub async fn exists(key: &str) -> ApiResult<bool> {
    let mut conn = get_manager()?;
    let result = conn.exists(key).await?;
    Ok(result)
}

/// 设置带TTL的键值对
pub async fn set_with_expire<V>(key: &str, value: V, seconds: u64) -> ApiResult<()>
where
    V: ToRedisArgs + ToSingleRedisArg + Send + Sync + 'static,
{
    let mut conn = get_manager()?;
    let _: () = conn.set_ex(key, value, seconds).await?;
    Ok(())
}

pub async fn publish<V>(channel: &str, value: V) -> ApiResult<()>
where
    V: ToRedisArgs + ToSingleRedisArg + Send + Sync + 'static,
{
    let mut conn = get_manager()?;
    let _: () = conn.publish(channel, value).await?;
    Ok(())
}
