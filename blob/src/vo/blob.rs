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