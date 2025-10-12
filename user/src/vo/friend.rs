use serde::Serialize;

#[derive(Serialize)]
pub struct FriendResponse {
    pub id: i64,
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Serialize)]
pub struct FriendListResponse {
    pub friends: Vec<FriendResponse>,
}