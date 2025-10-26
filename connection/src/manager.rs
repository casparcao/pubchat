use anyhow::Result;
use tokio::io::AsyncWriteExt;
use tracing::{info, warn};
use core::proto::message::Message;
use std::collections::HashMap;
use tokio::sync::broadcast;
use lapin::{BasicProperties};
use serde_json;

use crate::queue::RabbitMqManager;
use crate::client::Client;

pub struct ConnectionManager {
    clients: HashMap<u64, Client>,
    // 使用广播通道来转发消息给所有相关客户端
    message_sender: broadcast::Sender<Message>,
    // RabbitMQ管理器用于消息投递
    rabbitmq_manager: RabbitMqManager,
}

impl ConnectionManager {
    pub fn new(rabbitmq_manager: RabbitMqManager) -> Self {
        let (message_sender, _) = broadcast::channel(100);
        
        Self {
            clients: HashMap::new(),
            message_sender,
            rabbitmq_manager,
        }
    }
    
    pub fn add_client(&mut self, uid: u64, client: Client) {
        self.clients.insert(uid, client);
    }
    
    pub fn broadcast_message(&self, message: &Message) -> Result<()> {
        let _ = self.message_sender.send(message.clone());
        Ok(())
    }
    
    pub async fn publish_to_rabbitmq(&self, message: &Message) -> Result<()> {
        let payload = serde_json::to_vec(message)?;
        let _confirm = &self.rabbitmq_manager.get_channel()
            .basic_publish(
                "", // default exchange
                &self.rabbitmq_manager.get_queue_name(),
                lapin::options::BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await?
            .await?;
        info!("Message published to RabbitMQ");
        Ok(())
    }

    // 新增方法：向特定用户发送消息
    pub async fn send_message_to_client(&self, target_uid: u64, message: &Message) -> Result<()> {
        if let Some(client) = self.clients.get(&target_uid) {
            let encoded = core::proto::codec::encode(message)?;
            let mut writer = client.writer.lock().await;
            writer.write_all(&encoded).await?;
            writer.flush().await?;
            info!("Message sent to client {}", target_uid);
        } else {
            warn!("Client {} not found, message not sent", target_uid);
        }
        Ok(())
    }
}