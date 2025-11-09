use serde::{Deserialize, Serialize};

use crate::api::types::contact::ContactResponse;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateSessionRequest {
    //必须携带id，这样服务器端判断是否已经创建了会话，如果存在直接返回
    pub id: i64,
    pub name: String,
    pub members: Vec<ContactResponse>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SessionResponse {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SessionDetailResponse {
    pub id: i64,
    pub name: String,
    pub members: Vec<ContactResponse>,
}
