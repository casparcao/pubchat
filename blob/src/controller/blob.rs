use axum::{
    body::Body,
    extract::{Path, Extension},
    routing::{get, post},
    Router,
    response::Response,
    http::{StatusCode, header},
};
use core::auth::User;
use core::extract::Multipart;
use core::response::{ApiResponse, ApiErr};
use anyhow::Result;
use tokio::fs;
use crate::{service::blob::{get_blob, upload_file}, vo::blob::UploadBlobResponse};

pub async fn upload_blob(
    Extension(user): Extension<User>,
    multipart: Multipart,
) -> Result<ApiResponse<UploadBlobResponse>, ApiErr> {
    Ok(ApiResponse::One(upload_file(user, multipart).await?))
}

pub async fn download_blob(
    Path((id, _filename)): Path<(i64, String)>,
) -> Result<Response, ApiErr> {
    let blob = get_blob(id).await.map_err(|e| ApiErr(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    match blob {
        Some(blob) => {
            // Check if file exists
            if !std::path::Path::new(&blob.path).exists() {
                return Err(ApiErr(StatusCode::NOT_FOUND, "File not found".to_string()));
            }
            
            // Read file
            let file = fs::File::open(&blob.path).await.map_err(|e| ApiErr(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);
            
            let mut response = Response::new(body);
            
            // Set headers
            let headers = response.headers_mut();
            headers.insert(header::CONTENT_TYPE, blob.mime_type.parse().unwrap());
            headers.insert(header::CONTENT_LENGTH, blob.size.to_string().parse().unwrap());
            headers.insert(
                header::CONTENT_DISPOSITION,
                format!("inline; filename=\"{}\"", blob.name).parse().unwrap()
            );
            
            Ok(response)
        },
        None => {
            Err(ApiErr(StatusCode::NOT_FOUND, "Blob not found".to_string()))
        }
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/blobs", post(upload_blob))
        .route("/blobs/:id/:filename", get(download_blob))
}