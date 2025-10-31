use core::response::{ApiErr, ApiResult};

use reqwest;
use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::service::session_host;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SessionResponse {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSessionRequest {
    //必须携带id，这样服务器端判断是否已经创建了会话，如果存在直接返回
    pub id: i64,
    pub name: String,
    pub members: Vec<CreateSessionUserRequest>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSessionUserRequest {
    pub id: i64,
    pub name: String,
}

pub fn get_sessions(token: &str) -> Result<Vec<SessionResponse>> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/user/sessions", session_host());
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()?;
    if response.status().is_success() {
        let result : ApiResult<Vec<SessionResponse>> = response.json()?;
        if result.ok {
            return Ok(result.data.unwrap());
        }else{
            return Err(ApiErr::Error(result.message.unwrap()).into());
        }
    } else {
        let status = response.status();
        let error_text = response.text()?;
        Err(ApiErr::Error(format!("Failed to get sessions: {} - {}", status, error_text).into()).into())
    }
}

pub fn create_session(token: &str, payload: CreateSessionRequest) -> Result<SessionResponse> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/sessions", session_host());
    
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&payload)
        .send()?;
        
    if response.status().is_success() {
        let result: ApiResult<SessionResponse> = response.json()?;
        if result.ok {
            Ok(result.data.unwrap())
        } else {
            Err(ApiErr::Error(result.message.unwrap()).into())
        }
    } else {
        let status = response.status();
        let error_text = response.text()?;
        Err(ApiErr::Error(format!("Failed to create session: {} - {}", status, error_text).into()).into())
    }
}

//计算两个用户的唯一会话ID（始终不变，以便更快的找到两人的会话）
pub fn calc_session_id(uid1: i64, uid2: i64) -> u64{
    let (min, max) = if uid1 < uid2 { (uid1 as u64, uid2 as u64) } else { (uid2 as u64, uid1 as u64) };
    (min << 32) | (max & 0xFFFFFFFF)
}