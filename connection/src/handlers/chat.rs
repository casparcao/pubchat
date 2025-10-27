use anyhow::{Ok, Result};
use tracing::{info, error};
use core::proto::message::Message;
use core::proto::message::message;
use crate::queue;
pub async fn handle(message: Message) -> Result<()> { 
    if let Some(message::Content::Chat(chat_req)) = &message.content {
        info!("Processing ChatRequest from user {}: room={}, message='{}', timestamp={}",
                chat_req.speaker, chat_req.room, chat_req.message, chat_req.ts);
        // Publish message to RabbitMQ
        if let Err(e) = queue::publish(&message).await {
            error!("Failed to publish message to RabbitMQ: {}", e);
        }else{
            info!("Broadcast ChatResponse for user {} in room {}", 
                chat_req.speaker, chat_req.room);
        }
    }
    Ok(())
}