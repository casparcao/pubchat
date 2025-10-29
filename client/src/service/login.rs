use core::{auth::Token, response::{ApiErr, ApiResult}};
use anyhow::Result;
use serde::Serialize;


// 登录请求结构
#[derive(Serialize)]
pub(crate) struct LoginRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}


    // 执行登录操作
pub async fn login(body: &LoginRequest) -> Result<Token> {
    // 创建HTTP客户端
    let client = reqwest::Client::new();
    
    // 发送登录请求
    let response = client
        .post("http://127.0.0.1:3000/login")
        .json(body)
        .send()
        .await?;
    
    // 检查响应状态
    if response.status().is_success() {
        let response: ApiResult<Token> = response
            .json()
            .await?;
        if response.ok {
            Ok(response.data.unwrap())
        } else {
            let message = response.message.unwrap_or("".to_string());
            Err(ApiErr::Error(message).into())
        }
    } else {
        let status = response.status();
        let error_text = response
            .text()
            .await?;
        Err(ApiErr::Error(format!("{} {}", status.as_u16(), error_text)).into())
    }
}