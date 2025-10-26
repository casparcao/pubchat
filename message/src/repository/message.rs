use anyhow::Result;
use crate::model::message::Message;
use crate::repository::db;


pub async fn save(message: &Message) -> Result<()> {
    let mut connection = db::connection().await?;
    sqlx::query("INSERT INTO messages (id, speaker_id, room_id, message_type, content, timestamp, nickname) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(&message.id)
        .bind(&message.speaker_id)
        .bind(&message.room_id)
        .bind(&message.message_type)
        .bind(&message.content)
        .bind(&message.timestamp)
        .bind(&message.nickname)
        .execute(connection.as_mut())
        .await?;
    Ok(())
}

pub async fn find_by_room_id(room_id: i64, limit: u32) -> Result<Vec<Message>> {
    let mut connection = db::connection().await?;
    let result : Vec<Message> = sqlx::query_as(r#"
        SELECT * FROM messages 
        WHERE room_id = ? 
        ORDER BY timestamp DESC
        LIMIT ?
        "#)
    .bind(room_id)
    .bind(limit)
    .fetch_all(connection.as_mut())
    .await?;
    Ok(result)
}

pub async fn find_by_speaker_id(speaker_id: i64, limit: u32) -> Result<Vec<Message>> {
    let mut connection = db::connection().await?;
    let result : Vec<Message> = sqlx::query_as(r#"
        SELECT * FROM messages 
        WHERE speaker_id = ? 
        ORDER BY timestamp DESC
        LIMIT ?
        "#)
    .bind(speaker_id)
    .bind(limit)
    .fetch_all(connection.as_mut())
    .await?;
    Ok(result)
}