use axum::Extension;
use crate::common::auth::User;
use crate::common::response::{ApiErr, ApiResponse};
use crate::vo::friend::FriendResponse;

pub async fn add_friend(
    Extension(claims): Extension<User>,
    axum::extract::Path(friend_id): axum::extract::Path<i64>,
) -> Result<ApiResponse<i64>, ApiErr> {
    Ok(ApiResponse::One(crate::service::friend::add_friend(claims, friend_id).await?))
}

pub async fn get_friend_list(
    Extension(claims): Extension<User>,
) -> Result<ApiResponse<FriendResponse>, ApiErr> {
    let list = crate::service::friend::get_friend_list(claims).await?;
    Ok(ApiResponse::List(list, 0))
}

pub async fn remove_friend(
    Extension(claims): Extension<User>,
    axum::extract::Path(friend_id): axum::extract::Path<i64>,
) -> Result<ApiResponse<bool>, ApiErr> {
    Ok(ApiResponse::One(crate::service::friend::remove_friend(claims, friend_id).await?))
}