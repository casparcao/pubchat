use std::fmt::Display;

use serde::{Deserialize, Serialize};

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