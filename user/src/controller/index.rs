use core::response::{ApiErr, ApiResponse};
use axum::{body::Body, http::Request};
use log;
use serde::{Deserialize, Serialize};
use core::auth::User;
use core::extract::Query;

pub async fn index<'a>(Query(param): Query<IndexRequest>, request: Request<Body>) -> Result<ApiResponse<String>, ApiErr>{
    log::info!("index param: {},{},{}", param.sn, param.ts, param.n);
    let claims = request.extensions().get::<User>();
    log::info!("index claims: {:?}", claims);
    Ok(ApiResponse::One("index".to_string()))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexRequest{
    pub sn: String,
    pub ts: i64,
    pub n: i64,
}