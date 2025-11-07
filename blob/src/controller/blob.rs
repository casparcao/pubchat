use axum::{
    routing::{get, post},
    extract::{Extension},
    Router,
};
use core::auth::User;
use core::extract::Multipart;
use core::extract::{Path};
use core::response::{ApiResponse, ApiErr};
use anyhow::Result;
use crate::{service::blob::{get_blob, upload_file}, vo::blob::{BlobResponse, BlobUploadResponse}};

pub async fn upload_blob(
    Extension(user): Extension<User>,
    multipart: Multipart,
) -> Result<ApiResponse<BlobUploadResponse>, ApiErr> {
    Ok(ApiResponse::One(upload_file(user, multipart).await?))
}

pub async fn get_blob_by_id(
    Path(id): Path<i64>,
) -> Result<ApiResponse<BlobResponse>, ApiErr> {
    Ok(ApiResponse::One(get_blob(id).await?))
}

pub fn router() -> Router {
    Router::new()
        .route("/blobs", post(upload_blob))
        .route("/blobs/{id}", get(get_blob_by_id))
}