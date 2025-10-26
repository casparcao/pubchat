use anyhow::Result;
use tokio::net::TcpListener;
use tracing::{info, error, Level};
use tracing_subscriber;
use std::sync::Arc;
use tokio::sync::Mutex;
mod client;
mod manager;
mod message_handler;
mod queue;

use manager::ConnectionManager;
use queue::RabbitMqManager;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Initialize RabbitMQ
    let rabbitmq_manager: RabbitMqManager = queue::init().await?;
    
    let connection_manager = Arc::new(Mutex::new(ConnectionManager::new(rabbitmq_manager.clone())));

    // 启动RabbitMQ消息消费（如果RabbitMQ可用）
    let manager_clone = connection_manager.clone();
    tokio::spawn(async move {
        if let Err(e) = message_handler::handle_rabbitmq_messages(manager_clone, rabbitmq_manager).await {
            error!("Error handling RabbitMQ messages: {}", e);
        }
    });

    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("Connection service listening on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        let manager = connection_manager.clone();
        
        tokio::spawn(async move {
            if let Err(e) = client::handle_client(socket, manager).await {
                error!("Error handling client: {}", e);
            }
        });
    }
}