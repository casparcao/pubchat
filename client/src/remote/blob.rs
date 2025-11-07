use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::remote::blob_host;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobUploadResponse {
    pub id: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobResponse {
    pub id: i64,
    pub name: String,
    pub size: i64,
    pub exp: Option<String>,
    pub path: String,
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
        let result: BlobUploadResponse = response.json()?;
        Ok(result)
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
        let file_data = response.bytes()?;
        std::fs::write(save_path, file_data)?;
        Ok(())
    } else {
        let status = response.status();
        let error_text = response.text()?;
        Err(anyhow::anyhow!("Failed to download file: {} - {}", status, error_text))
    }
}