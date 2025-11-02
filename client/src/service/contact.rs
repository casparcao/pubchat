use core::response::{ApiErr, ApiResult};

use reqwest;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use sqlx::prelude::FromRow;

use crate::service::user_host;

#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct ContactResponse {
    pub id: i64,
    pub name: String,
    pub avatar: Option<String>,
}

pub async fn get_contacts(token: &str) -> Result<Vec<ContactResponse>> {
    let client = reqwest::Client::new();
    let url = format!("{}/contacts", user_host());
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    if response.status().is_success() {
        let result : ApiResult<Vec<ContactResponse>> = response.json().await?;
        if result.ok {
            return Ok(result.data.unwrap());
        }else{
            return Err(ApiErr::Error(result.message.unwrap()).into());
        }
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        Err(ApiErr::Error(format!("Failed to get contacts: {} - {}", status, error_text).into()).into())
    }
}
