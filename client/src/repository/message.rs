use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, Sqlite, prelude::FromRow};
use core::request::Page;
use crate::db;


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: i64,
    pub speaker_id: i64,
    pub receiver_id: i64,
    pub room_id: i64,
    pub message_type: i32,
    pub content: String,
    pub timestamp: i64,
    pub nickname: String,
}

//查询指定聊天室的最新的n条消息
pub(crate) async fn select_messages(rid: i64, page: Page) -> Result<(Vec<Message>, i64)>{
    let mut connection = db::connection().await?;
    let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT *
            FROM messages 
            where room_id= ");
    builder.push_bind(rid);
    builder
        .push(" ORDER BY timestamp DESC")
        .push(" LIMIT ")
        .push_bind(page.ps)
        .push(" OFFSET ")
        .push_bind(page.offset());
    let user:Vec<Message> = builder
        .build_query_as()
        .fetch_all(connection.as_mut())
        .await?;
    let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT COUNT(1)
            FROM messages
            where room_id = ");
    builder.push_bind(rid);
    let count:(i64,) = builder
        .build_query_as()
        .fetch_one(connection.as_mut())
        .await?;
    Ok((user, count.0))
}

pub async fn save(message: &Message) -> Result<()> {
    let mut connection = db::connection().await?;
    sqlx::query("INSERT INTO messages (id, speaker_id, receiver_id, room_id, message_type, content, timestamp, nickname) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(&message.id)
        .bind(&message.speaker_id)
        .bind(&message.receiver_id)
        .bind(&message.room_id)
        .bind(&message.message_type)
        .bind(&message.content)
        .bind(&message.timestamp)
        .bind(&message.nickname)
        .execute(connection.as_mut())
        .await?;
    Ok(())
}