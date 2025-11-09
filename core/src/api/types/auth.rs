use serde::{Serialize, Deserialize}; // 0.14.0

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest{
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WxLoginRequest{
    pub code: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterRequest{
    pub username: String,
    pub password: String,
    pub gender: String,
    pub age: i8
}
