use axum::Extension;
use core::auth::User;
use core::response::{ApiErr, ApiResponse};
use crate::vo::contact::ContactResponse;
use core::extract::Path;

pub async fn add(
    Extension(claims): Extension<User>,
    Path(there): Path<i64>,
) -> Result<ApiResponse<i64>, ApiErr> {
    Ok(ApiResponse::One(crate::service::contact::add(claims, there).await?))
}

pub async fn get_list(
    Extension(claims): Extension<User>,
) -> Result<ApiResponse<ContactResponse>, ApiErr> {
    let list = crate::service::contact::get_list(claims).await?;
    Ok(ApiResponse::List(list, 0))
}

pub async fn remove(
    Extension(claims): Extension<User>,
    axum::extract::Path(there): axum::extract::Path<i64>,
) -> Result<ApiResponse<bool>, ApiErr> {
    Ok(ApiResponse::One(crate::service::contact::remove(claims, there).await?))
}