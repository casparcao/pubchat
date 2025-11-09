use crate::{api::types::contact::ContactResponse, response::{ApiErr, ApiResult}};

use reqwest;
use anyhow::Result;

use crate::api::client::user_host;

pub fn get_contacts(token: &str) -> Result<Vec<ContactResponse>> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/contacts", user_host());
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()?;
    if response.status().is_success() {
        let result : ApiResult<Vec<ContactResponse>> = response.json()?;
        if result.ok {
            return Ok(result.data.unwrap());
        }else{
            return Err(ApiErr::Error(result.message.unwrap()).into());
        }
    } else {
        let status = response.status();
        let error_text = response.text()?;
        Err(ApiErr::Error(format!("Failed to get contacts: {} - {}", status, error_text).into()).into())
    }
}
