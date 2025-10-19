use axum::extract::State;
use redis::{aio::Connection, AsyncCommands};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

pub struct Cache {
    conn: Connection,
    ttl: Duration,
}

impl Cache {
    pub async fn new(redis_url: &str, ttl: Duration) -> anyhow::Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let conn = client.get_async_connection().await?;
        Ok(Self { conn, ttl })
    }

    pub async fn get<T: DeserializeOwned>(&mut self, key: &str) -> anyhow::Result<Option<T>> {
        let data: Option<String> = self.conn.get(key).await?;
        Ok(data.map(|d| serde_json::from_str(&d)).transpose()?)
    }

    pub async fn set<T: Serialize>(&mut self, key: &str, value: &T) -> anyhow::Result<()> {
        let data = serde_json::to_string(value)?;
        self.conn.set_ex(key, data, self.ttl.as_secs() as usize).await?;
        Ok(())
    }
}