use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct CrawlRequest{
    pub kw: String
}