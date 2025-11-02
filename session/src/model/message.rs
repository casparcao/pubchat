use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: i64,
    pub sender: i64,
    pub session: i64,
    pub mtype: i32,
    pub content: String,
    pub timestamp: i64,
    pub uname: String,
}