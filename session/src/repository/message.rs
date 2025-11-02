use anyhow::Result;
use crate::model::message::Message;
use crate::repository::db;


pub async fn save(message: &Message) -> Result<()> {
    let mut connection = db::connection().await?;
    sqlx::query("INSERT INTO messages (id, sender, session, mtype, content, timestamp, uname) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(&message.id)
        .bind(&message.sender)
        .bind(&message.session)
        .bind(&message.mtype)
        .bind(&message.content)
        .bind(&message.timestamp)
        .bind(&message.uname)
        .execute(connection.as_mut())
        .await?;
    Ok(())
}

pub async fn find_by_session(session: i64, limit: u32) -> Result<Vec<Message>> {
    let mut connection = db::connection().await?;
    let result : Vec<Message> = sqlx::query_as(r#"
        SELECT * FROM messages 
        WHERE session = ? 
        ORDER BY timestamp DESC
        LIMIT ?
        "#)
    .bind(session)
    .bind(limit)
    .fetch_all(connection.as_mut())
    .await?;
    Ok(result)
}
