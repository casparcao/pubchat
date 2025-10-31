use core::response::{ApiErr, ApiResult};
use reqwest;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::service::session_host;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MessageItem {
    pub id: i64,
    pub session_id: i64,
    pub sender_id: i64,
    pub sender_name: String,
    pub content: String,
    pub createtime: i64, // 使用字符串格式的时间
}

/// 获取指定会话的消息历史
pub fn get_session_messages(token: &str, session_id: i64) -> Result<Vec<MessageItem>> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/sessions/{}/messages", session_host(), session_id);
    
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()?;
        
    if response.status().is_success() {
        let result: ApiResult<Vec<MessageItem>> = response.json()?;
        if result.ok {
            Ok(result.data.unwrap())
        } else {
            Err(ApiErr::Error(result.message.unwrap()).into())
        }
    } else {
        let status = response.status();
        let error_text = response.text()?;
        Err(ApiErr::Error(format!("Failed to get session messages: {} - {}", status, error_text).into()).into())
    }
}