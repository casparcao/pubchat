use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, Sqlite, prelude::FromRow};
use core::request::Page;
use crate::db;


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: i64,
    pub sender: i64,
    pub receiver: i64,
    pub session: i64,
    pub mtype: i32,
    pub content: String,
    pub timestamp: i64,
    pub uname: String,
}

//查询指定聊天室的最新的n条消息
pub(crate) async fn select_messages(sid: i64, page: Page) -> Result<(Vec<Message>, i64)>{
    let mut connection = db::connection().await?;
    let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT *
            FROM messages 
            where session = ");
    builder.push_bind(sid);
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
            where session = ");
    builder.push_bind(sid);
    let count:(i64,) = builder
        .build_query_as()
        .fetch_one(connection.as_mut())
        .await?;
    Ok((user, count.0))
}

pub async fn save(message: &Message) -> Result<()> {
    let mut connection = db::connection().await?;
    sqlx::query("INSERT INTO messages (id, sender, receiver, session, mtype, content, timestamp, uname) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(&message.id)
        .bind(&message.sender)
        .bind(&message.receiver)
        .bind(&message.session)
        .bind(&message.mtype)
        .bind(&message.content)
        .bind(&message.timestamp)
        .bind(&message.uname)
        .execute(connection.as_mut())
        .await?;
    Ok(())
}