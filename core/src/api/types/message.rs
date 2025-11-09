use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct MessageRequest {
    _limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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