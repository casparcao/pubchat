use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct MessageRequest {
    _limit: Option<u32>,
}