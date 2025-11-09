use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ContactResponse {
    pub id: i64,
    pub name: String,
    pub avatar: Option<String>,
}
