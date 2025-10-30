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
    pub name: String,
    pub members: Vec<CreateSessionUserRequest>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSessionUserRequest {
    pub id: i64,
    pub name: String,
}

pub async fn get_sessions(token: &str) -> Result<Vec<SessionResponse>> {
    let client = reqwest::Client::new();
    let url = format!("{}/user/sessions", session_host());
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    if response.status().is_success() {
        let result : ApiResult<Vec<SessionResponse>> = response.json().await?;
        if result.ok {
            return Ok(result.data.unwrap());
        }else{
            return Err(ApiErr::Error(result.message.unwrap()).into());
        }
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        Err(ApiErr::Error(format!("Failed to get sessions: {} - {}", status, error_text).into()).into())
    }
}

pub async fn create_session(token: &str, payload: CreateSessionRequest) -> Result<SessionResponse> {
    let client = reqwest::Client::new();
    let url = format!("{}/sessions", session_host());
    
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&payload)
        .send()
        .await?;
        
    if response.status().is_success() {
        let result: ApiResult<SessionResponse> = response.json().await?;
        if result.ok {
            Ok(result.data.unwrap())
        } else {
            Err(ApiErr::Error(result.message.unwrap()).into())
        }
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        Err(ApiErr::Error(format!("Failed to create session: {} - {}", status, error_text).into()).into())
    }
}
