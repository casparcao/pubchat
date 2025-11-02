use anyhow::{Ok, Result};
use tracing::{info, error};
use core::proto::message::Message;
use core::proto::message::message;
use crate::queue;
pub async fn handle(message: Message) -> Result<()> { 
    if let Some(message::Content::Chat(chat_req)) = &message.content {
        info!("Processing ChatRequest from user {}: session={}, message='{}', timestamp={}",
                chat_req.sender, chat_req.session, chat_req.message, chat_req.ts);
        // Publish message to RabbitMQ
        if let Err(e) = queue::publish(&message).await {
            error!("Failed to publish message to RabbitMQ: {}", e);
        }else{
            info!("Broadcast ChatResponse for user {} in room {}", 
                chat_req.sender, chat_req.session);
        }
    }
    Ok(())
}