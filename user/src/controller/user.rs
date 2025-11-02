use axum::Extension;
use core::auth::User;
use core::extract::{Json, Multipart};
use core::response::{ApiErr, ApiResponse};
use crate::vo::user::{SetNameRequest, UserResponse};

pub async fn upload_avatar(Extension(claims): Extension<User>, multipart: Multipart) -> Result<ApiResponse<String>, ApiErr>{
    Ok(ApiResponse::One(crate::service::user::upload_avatar(claims, multipart).await?))
}

pub async fn set_username(Extension(claims): Extension<User>, Json(body): Json<SetNameRequest>) -> Result<ApiResponse<()>, ApiErr>{
    Ok(ApiResponse::One(crate::service::user::set_username(claims, body.name).await?))
}

pub async fn select_current_user(Extension(claims): Extension<User>) -> Result<ApiResponse<UserResponse>, ApiErr>{
    let user = crate::service::user::select_user(claims).await?;
    Ok(ApiResponse::One(UserResponse{id: user.id, name: user.name, avatar: user.avatar}))
}