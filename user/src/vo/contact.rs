use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ContactResponse {
    pub id: i64,
    pub name: String,
    pub avatar: Option<String>,
}
