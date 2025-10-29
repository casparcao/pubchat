use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct MessageRequest {
    limit: Option<u32>,
}