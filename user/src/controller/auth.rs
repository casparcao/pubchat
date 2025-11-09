use std::time::{SystemTime, UNIX_EPOCH};

use core::auth::Token;
use core::extract::Json;
use core::api::types::auth::{LoginRequest, RegisterRequest};
use log;
use core::auth;
use core::response::{ApiErr, ApiResponse};

pub async fn login<'a>(Json(request): Json<LoginRequest>) -> Result<ApiResponse<Token>, ApiErr>{
    let username = request.username;
    let password = request.password;
    log::info!("login username: {}, password: {}", username, password);
    let result = crate::service::auth::login(&username, &password).await;
    _response(result?,username)
}


fn _response(id: i64, uname: String) -> Result<ApiResponse<Token>, ApiErr>{
    let jwt = auth::issue(id, uname)?;
    Ok(ApiResponse::One(Token{token: jwt, exp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() + 1800000u128}))
}

pub async fn logout() -> ApiResponse<()>{
    ApiResponse::One(())
}

pub async fn signup(Json(request): Json<RegisterRequest>) -> Result<ApiResponse<i64>, ApiErr>{
    Ok(ApiResponse::One(crate::service::auth::signup(request).await?))
}