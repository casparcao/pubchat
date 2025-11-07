use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateSessionRequest {
    //必须携带id，这样服务器端判断是否已经创建了会话，如果存在直接返回
    pub id: i64,
    pub name: String,
    pub members: Vec<CreateSessionUserRequest>,
}

#[derive(Deserialize)]
pub struct CreateSessionUserRequest{
    pub id: i64,
    pub name: String,
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