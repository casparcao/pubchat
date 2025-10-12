use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FriendResponse {
    pub id: i64,
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FriendListResponse {
    pub friends: Vec<FriendResponse>,
}

pub struct FriendService {
    base_url: String,
    client: reqwest::Client,
}

impl FriendService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_friends(&self, token: &str) -> Result<Vec<FriendResponse>, Box<dyn std::error::Error>> {
        let url = format!("{}/friends", self.base_url);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        if response.status().is_success() {
            let friends_response: FriendListResponse = response.json().await?;
            Ok(friends_response.friends)
        } else {
            let status = response.status();
            let error_text = response.text().await?;
            Err(format!("Failed to get friends: {} - {}", status, error_text).into())
        }
    }
}