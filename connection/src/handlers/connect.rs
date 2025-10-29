use anyhow::{Ok, Result};
use tokio::{io::AsyncWriteExt, sync::Mutex};
use tracing::info;
use core::proto::message::Message;
use core::response::ApiErr;
use tokio::net::tcp::OwnedWriteHalf;
use core::proto::message::{ConnectResponse, Type, message};
use core::proto::codec::encode;
use std::sync::Arc;
use crate::manager::Client;
pub async fn handle(message: &Message, mut writer: OwnedWriteHalf) -> Result<Client> { 
    let mut uid = 0u64;
    if message.r#type == Type::ConnectRequest as i32 {
        info!("Client {} registered in connection manager", uid);
        if let Some(message::Content::ConnectRequest(req)) = &message.content {
            info!("Connect request with token: {}", req.token);
            let user = core::auth::verify(&req.token)?;
            uid = user.id as u64;
            // Create a connection response
            let response = Message {
                id: message.id,
                ts: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                r#type: Type::ConnectResponse as i32,
                content: Some(message::Content::ConnectResponse(ConnectResponse {
                    code: 0,
                    message: "Connected successfully".to_string(),
                    uid,
                })),
            };
            // Send the response
            let encoded = encode(&response)?;
            writer.write_all(&encoded).await?;
            writer.flush().await?; // Ensure the data is sent immediately
            info!("Sent connection response to client");
            // 注册客户端到连接管理器
            let client = Client {
                uid: uid,
                writer: Arc::new(Mutex::new(writer)),
            };
            return Ok(client);
        }
    }
    Err(ApiErr::Bad(401, "unauthorized".to_string()).into())
}