
use axum::extract::Path;
use axum::Extension;

use core::auth::User;
use core::extract::Json;
use core::{extract::Query, request::Page, response::ApiResponse};
use core::response::ApiErr;
use crate::model::role::Role;
use crate::service::role;
use crate::vo::role::{RoleCreateRequest, RoleListRequest};



pub async fn select_roles(Query(page): Query<Page>, Query(param): Query<RoleListRequest>) -> Result<ApiResponse<Role>, ApiErr>{
    let result = role::select_roles(page, param).await?;
    Ok(ApiResponse::List(result.0, result.1))
}

pub async fn create_role(Extension(claims): Extension<User>, Json(body): Json<RoleCreateRequest>) -> Result<ApiResponse<i64>, ApiErr>{
    Ok(ApiResponse::One(role::create_role(body, &claims).await?))
}

pub async fn delete_role(Path(id): Path<i64>) ->  Result<ApiResponse<()>, ApiErr>{
   Ok(ApiResponse::One(role::delete_role(id).await?))
}