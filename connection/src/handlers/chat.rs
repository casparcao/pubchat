use anyhow::{Ok, Result};
use tokio::{io::AsyncWriteExt, sync::Mutex};
use tracing::{info, warn, error};
use core::proto::message::Message;
use core::response::ApiErr;
use std::{collections::HashMap, sync::OnceLock};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use core::proto::message::{ConnectResponse, Type, message, Chat};
use core::proto::codec::{decode, encode};
use std::sync::Arc;
use crate::manager::Client;
use crate::queue;
pub async fn handle(message: Message) -> Result<()> { 
    if let Some(message::Content::Chat(chat_req)) = message.content {
        info!("Processing ChatRequest from user {}: room={}, message='{}', timestamp={}",
                chat_req.speaker, chat_req.room, chat_req.message, chat_req.ts);
        // Create a chat response mirroring the request
        let chat_response = Message {
            id: message.id,
            ts: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            r#type: Type::Chat as i32,
            content: Some(message::Content::Chat(Chat{
                speaker: chat_req.speaker,
                room: chat_req.room,
                r#type: chat_req.r#type,
                message: chat_req.message,
                ts: chat_req.ts,
                nickname: chat_req.nickname,
            })),
        };
        
        // Broadcast the message to all clients
        {
            // Publish message to RabbitMQ
            if let Err(e) = queue::publish(&chat_response).await {
                error!("Failed to publish message to RabbitMQ: {}", e);
            }
        }
        info!("Broadcast ChatResponse for user {} in room {}", 
                chat_req.speaker, chat_req.room);
    }
    Ok(())
}