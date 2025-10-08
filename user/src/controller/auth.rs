use std::time::{SystemTime, UNIX_EPOCH};

use crate::common::extract::Json;
use crate::vo::auth::{LoginRequest, RegisterRequest, Token};
use log;
use crate::common::auth;
use crate::common::response::{ApiErr, ApiResponse};

pub async fn login<'a>(Json(request): Json<LoginRequest>) -> Result<ApiResponse<Token>, ApiErr>{
    let username = request.username;
    let password = request.password;
    log::info!("login username: {}, password: {}", username, password);
    let result = crate::service::auth::login(&username, &password).await;
    _response(result)
}


fn _response(result: anyhow::Result<i64>) -> Result<ApiResponse<Token>, ApiErr>{
    let id = result?;
    let jwt = auth::issue(id)?;
    Ok(ApiResponse::One(Token{token: jwt, exp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() + 1800000u128}))
}

pub async fn logout() -> ApiResponse<()>{
    ApiResponse::One(())
}

pub async fn signup(Json(request): Json<RegisterRequest>) -> Result<ApiResponse<i64>, ApiErr>{
    Ok(ApiResponse::One(crate::service::auth::signup(request).await?))
}