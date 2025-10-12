use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct FriendResponse {
    pub id: i64,
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct FriendListResponse {
    pub friends: Vec<FriendResponse>,
}