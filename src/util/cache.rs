use std::env;

use redis::{Client, AsyncCommands};
use anyhow::Result;

/// 缓存数据
pub async fn put(client: &Client, key: &str, value: &str) -> Result<()> {
    let mut conn = client.get_multiplexed_tokio_connection().await?;
    let key = get_prefix_key(key);
    let _: () = conn.set(key, value).await?;
    Ok(())
}

/// 缓存数据 (有效时间)
pub async fn put_ttl(client: &Client, key: &str, value: &str, ttl: u64) -> Result<()> {
    let mut conn = client.get_multiplexed_tokio_connection().await?;
    let key = get_prefix_key(key);
    let _: () = conn.set_ex(key, value, ttl).await?;
    Ok(())
}

/// 获取缓存
pub async fn get(client: &Client, key: &str) -> Result<Option<String>> {
    let mut conn = client.get_multiplexed_tokio_connection().await?;
    let key = get_prefix_key(key);
    let text: Option<String> = conn.get(key).await?;
    Ok(text)
}

/// 删除缓存
pub async fn del(client: &Client, key: &str) -> Result<()> {
    let mut conn = client.get_multiplexed_tokio_connection().await?;
    let key = get_prefix_key(key);
    let _: () = conn.del(key).await?;
    Ok(())
}

pub fn get_prefix_key(key: &str) -> String {
    let prefix = match env::var("REDIS_PREFIX") {
        Ok(prefix) => prefix,
        Err(_err) => {
            "".into()
        }
    };

    format!("{}{}", prefix, key)
}