use axum::{
    body::Body, 
    http::header::AUTHORIZATION, response::IntoResponse,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{sync::OnceLock, time::{SystemTime, UNIX_EPOCH}};
use tower_http::validate_request::ValidateRequest;
use anyhow::Result;

use crate::common::response::ApiErr;

// Quick instructions
//
// - get an authorization token:
//
// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -d '{"client_id":"foo","client_secret":"bar"}' \
//     http://localhost:3000/authorize
//
// - visit the protected area using the authorized token
//
// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.M3LAZmrzUkXDC1q5mSzFAs_kJrwuKz3jOoDmjJ0G4gM' \
//     http://localhost:3000/protected
//
// - try to visit the protected area using an invalid token
//
// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -H 'Authorization: Bearer blahblahblah' \
//     http://localhost:3000/protected

static KEYS: OnceLock<Keys> = OnceLock::new();

pub fn init() {
    let secret = std::env::var("JWT_SECRET").expect("未设置JWT_SECRET");
    let keys = Keys::new(secret.as_bytes());
    KEYS.set(keys).or(Err("")).expect("设置jwtsecret异常");
}

pub fn issue(id: i64) -> Result<String> {
    let keys = KEYS.get();
    if keys.is_none(){
        return Err(ApiErr::Error("JWT密钥异常").into());
    }
    let claims = User {
        id,
        oid: 0,
        // Mandatory expiry time as UTC timestamp
        exp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() + 1800000u128, // May 2033
    };
    // Create the authorization token
    encode(&Header::default(), &claims, &keys.unwrap().encoding)
    .map_err(|_| ApiErr::Error("Token颁发失败").into())
}

#[derive(Debug, Clone, Copy)]
pub struct AuthHeader;

impl<B> ValidateRequest<B> for AuthHeader {
    type ResponseBody = Body;
    fn validate(&mut self, 
        request: &mut axum::http::Request<B>
    ) -> Result<(), axum::http::Response<Self::ResponseBody>> {
        let keys = KEYS.get();
        if keys.is_none(){
            return Err(ApiErr::Error("JWT密钥异常").into_response());
        }
        let auth_header = request.headers().get(AUTHORIZATION);
        if auth_header.is_none(){
            return Err(ApiErr::Bad(400, "Authorization请求头缺失").into_response());
        }
        let token = auth_header.unwrap();
        let token = token.to_str();
        if token.is_err(){
            return Err(ApiErr::Bad(400, "提取Token失败").into_response());
        }
        let token = token.unwrap();
        log::info!("token:{}", token);
        let token = &token[7..];
        let token_data = decode::<User>(token, &keys.unwrap().decoding, &Validation::default());
        if token_data.is_err(){
            log::error!("token解析异常>>{}", token_data.unwrap_err());
            return Err(ApiErr::Bad(400, "Token解析异常").into_response());
        }
        let token_data = token_data.unwrap();
        let claims = token_data.claims;
        let exp = claims.exp;
        if exp  < SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() {
            return Err(ApiErr::Bad(401, "Token已过期").into_response());
        }
        request.extensions_mut().insert(claims);
        Ok(())
    }
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

//token编码前的组成内容，如用户id等信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    //用户id
    pub id: i64,
    //所在组织id
    pub oid: i64,
    pub exp: u128,
}
