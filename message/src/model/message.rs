use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: i64,
    pub speaker_id: i64,
    pub room_id: i64,
    pub message_type: i32,
    pub content: String,
    pub timestamp: i64,
    pub nickname: String,
}