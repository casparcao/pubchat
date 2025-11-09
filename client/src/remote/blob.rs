use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};
use core::response::{ApiErr, ApiResult};
use std::{fmt::Display, path::Path};

use crate::remote::blob_host;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobUploadResponse {
    pub id: i64,
    pub name: String,
    pub size: i64,
    pub exp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobResponse {
    pub id: i64,
    pub name: String,
    pub size: i64,
    pub exp: Option<String>,
    pub path: String,
}

impl Display for BlobUploadResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[File] {} (size: {}, exp: {}, download id: {})", 
            self.name, self.size, self.exp.as_ref().unwrap_or(&"never".to_string()), base62::encode(self.id as u128))
    }
}

/// Upload a file to the blob service
pub fn upload_file(token: &str, file_path: &str) -> Result<BlobUploadResponse> {
    let client = reqwest::blocking::Client::new();
    
    // Read the file
    let file_data = std::fs::read(file_path)?;
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    // Create multipart form
    let form = reqwest::blocking::multipart::Form::new()
        .part("file", reqwest::blocking::multipart::Part::bytes(file_data)
            .file_name(file_name));
    
    // Send upload request
    let response = client
        .post(&format!("{}/blobs", blob_host()))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()?;
    
    if response.status().is_success() {
        let result: ApiResult<BlobUploadResponse> = response.json()?;
        if result.ok {
            return Ok(result.data.unwrap());
        }else{
            return Err(ApiErr::Error(result.message.unwrap()).into());
        }
    } else {
        let status = response.status();
        let error_text = response.text()?;
        Err(anyhow::anyhow!("Failed to upload file: {} - {}", status, error_text))
    }
}

/// Download a file from the blob service
pub fn download_file(token: &str, file_id: i64, save_path: &str) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    
    // Send download request
    let response = client
        .get(&format!("{}/blobs/{}", blob_host(), file_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()?;
    
    if response.status().is_success() {
        let result: ApiResult<BlobResponse> = response.json()?;
        if result.ok {
            let path = result.data.unwrap().path;
            log::info!("Downloading file: {}", path);
            let file_data = client
                .get(&path)
                .send()?
                .bytes()?;
            std::fs::write(save_path, file_data)?;
            return Ok(());
        }else{
            return Err(ApiErr::Error(result.message.unwrap()).into());
        }
    } else {
        let status = response.status();
        let error_text = response.text()?;
        Err(anyhow::anyhow!("Failed to download file: {} - {}", status, error_text))
    }
}