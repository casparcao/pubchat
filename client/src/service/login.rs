use core::{auth::Token, response::{ApiErr, ApiResult}};
use anyhow::Result;
use serde::Serialize;

use crate::service::user_host;


// 登录请求结构
#[derive(Serialize)]
pub(crate) struct LoginRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}


    // 执行登录操作
pub fn login(body: &LoginRequest) -> Result<Token> {
    // 创建HTTP客户端
    let client = reqwest::blocking::Client::new();
    
    // 发送登录请求
    let response = client
        .post(format!("{}/login", user_host()))
        .json(body)
        .send()?;
    
    // 检查响应状态
    if response.status().is_success() {
        let response: ApiResult<Token> = response
            .json()?;
        if response.ok {
            Ok(response.data.unwrap())
        } else {
            let message = response.message.unwrap_or("".to_string());
            Err(ApiErr::Error(message).into())
        }
    } else {
        let status = response.status();
        let error_text = response
            .text()?;
        Err(ApiErr::Error(format!("{} {}", status.as_u16(), error_text)).into())
    }
}