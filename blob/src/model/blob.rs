use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Blob {
    pub id: i64,
    pub name: String, // Client original filename
    pub path: String, // Real storage path or object key
    pub size: i64,    // File size in bytes
    pub btype: String, // MIME type
    pub provider: String, // Storage provider (local, s3, etc.)
    pub bucket: Option<String>, // Optional bucket name
    pub open: bool,   // Publicly readable?
    pub exp: Option<DateTime<Utc>>, // Expiration time
    pub createtime: DateTime<Utc>, // Upload time
    pub uid: i64, // Uploader user ID
    pub hash: Option<String>, // Content hash
    pub deleted: bool, // Soft delete flag
}