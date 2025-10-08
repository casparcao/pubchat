use serde::{Serialize, Deserialize}; // 0.14.0
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct LoginRequest{
    #[validate(length(min = 3, max = 20, message = "用户名长度不能小于3"))]
    pub username: String,
    #[validate(length(min = 6, max = 50, message = "密码长度不能小于6"))]
    pub password: String
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct WxLoginRequest{
    pub code: String
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct RegisterRequest{
    pub username: String,
    pub password: String,
    pub gender: String,
    pub age: i8
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct Token{
    pub token: String,
    pub exp: u128
}