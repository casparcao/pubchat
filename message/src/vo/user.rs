use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserResponse{
    pub id: i64,
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetNameRequest{
    pub name: String
}