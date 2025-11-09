use crate::{api::types::message::Message, response::{ApiErr, ApiResult}};
use reqwest;
use anyhow::Result;
use crate::api::client::session_host;


/// 获取指定会话的消息历史
pub fn get_session_messages(token: &str, session_id: i64) -> Result<Vec<Message>> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/session/{}/messages", session_host(), session_id);
    
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()?;
        
    if response.status().is_success() {
        let result: ApiResult<Vec<Message>> = response.json()?;
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