use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateSessionRequest {
    pub name: String,
    pub members: Vec<i64>,
}

#[derive(Serialize)]
pub struct UserSessionResponse{
    pub id: i64,
    pub name: String,
}

#[derive(Serialize)]
pub struct SessionDetailResponse {
    pub id: i64,
    pub name: String,
    pub members: Vec<UserSessionResponse>,
}