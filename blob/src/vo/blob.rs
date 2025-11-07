use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadBlobResponse {
    pub id: i64,
    pub url: String,
}