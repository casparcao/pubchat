use anyhow::Result;
use lapin::{
    options::*, Connection, ConnectionProperties, Channel,
};
use tracing::{info, error};
use std::env;

pub struct RabbitMqConnection {
    connection: Connection,
}

pub struct RabbitMqManager {
    channel: Channel,
    queue_name: String,
}

impl RabbitMqConnection {
    pub async fn new() -> Result<Self> {
        let rabbitmq_addr = env::var("RABBITMQ_ADDR")
            .unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".to_string());
        
        info!("Connecting to RabbitMQ at {}", rabbitmq_addr);
        
        let connection = Connection::connect(&rabbitmq_addr, ConnectionProperties::default()).await?;
        info!("Connected to RabbitMQ");
        
        Ok(Self { connection })
    }
    
    pub async fn create_channel(&self) -> Result<Channel> {
        let channel = self.connection.create_channel().await?;
        Ok(channel)
    }
}

impl RabbitMqManager {
    pub async fn new(channel: Channel, queue_name: &str) -> Result<Self> {
        // Declare a queue for chat messages
        let _queue = channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                Default::default(),
            )
            .await?;
        
        info!("Declared queue '{}'", queue_name);
        
        Ok(Self {
            channel,
            queue_name: queue_name.to_string(),
        })
    }
    
    pub fn get_channel(&self) -> &Channel {
        &self.channel
    }
    
    pub fn get_queue_name(&self) -> &str {
        &self.queue_name
    }
}

pub async fn init() -> Result<Option<RabbitMqManager>> {
    match RabbitMqConnection::new().await {
        Ok(rabbit_conn) => {
            let channel = rabbit_conn.create_channel().await?;
            let manager = RabbitMqManager::new(channel, "chat_messages").await?;
            Ok(Some(manager))
        }
        Err(e) => {
            error!("Failed to initialize RabbitMQ: {}", e);
            Ok(None)
        }
    }
}