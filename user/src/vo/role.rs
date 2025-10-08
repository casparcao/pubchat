use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleListRequest{

    pub name: Option<String>,
    pub code: Option<String>

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleCreateRequest{
    pub name: String,
    pub code: String
}