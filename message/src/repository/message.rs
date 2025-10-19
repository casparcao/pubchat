use anyhow::Result;
use sqlx::mysql::MySqlPool;
use crate::model::message::Message;

pub struct MessageRepository {
    pool: MySqlPool,
}

impl MessageRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, message: &Message) -> Result<()> {
        sqlx::query!(
            "INSERT INTO messages (id, speaker_id, room_id, message_type, content, timestamp, nickname) VALUES (?, ?, ?, ?, ?, ?, ?)",
            message.id,
            message.speaker_id,
            message.room_id,
            message.message_type,
            message.content,
            message.timestamp,
            message.nickname
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn find_by_room_id(&self, room_id: i64, limit: u32) -> Result<Vec<Message>> {
        let messages = sqlx::query_as!(
            Message,
            "SELECT id, speaker_id, room_id, message_type, content, timestamp, nickname FROM messages WHERE room_id = ? ORDER BY timestamp DESC LIMIT ?",
            room_id,
            limit
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(messages)
    }

    pub async fn find_by_speaker_id(&self, speaker_id: i64, limit: u32) -> Result<Vec<Message>> {
        let messages = sqlx::query_as!(
            Message,
            "SELECT id, speaker_id, room_id, message_type, content, timestamp, nickname FROM messages WHERE speaker_id = ? ORDER BY timestamp DESC LIMIT ?",
            speaker_id,
            limit
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(messages)
    }
}