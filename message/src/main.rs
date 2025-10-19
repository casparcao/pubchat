use dotenv::dotenv;
use tokio::net::TcpListener;

pub mod model;
pub mod service;
pub mod repository;
pub mod controller;
pub mod vo;
pub mod common;
pub mod test;

use crate::repository::{db, rdb};
use crate::common::router;
use crate::repository::message::MessageRepository;
use crate::service::message::MessageService;
use crate::service::message_consumer::MessageConsumer;


#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv().ok();
    rdb::init().await;
    db::init().await;
    
    // Initialize message service
    let message_repo = MessageRepository::new(db::get().await?);
    let message_service = MessageService::new(message_repo);
    
    // Start RabbitMQ consumer in a separate task
    // let message_service_clone = message_service.clone();
    tokio::spawn(async move {
        let rabbitmq_addr = std::env::var("RABBITMQ_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".to_string());
        let queue_name = std::env::var("RABBITMQ_QUEUE_NAME").unwrap_or_else(|_| "chat_messages".to_string());
        
        let consumer = MessageConsumer::new(rabbitmq_addr, queue_name);
        if let Err(e) = consumer.consume_messages(message_service).await {
            eprintln!("Error consuming messages: {}", e);
        }
    });
    
    let app = router::init().expect("路由模块初始化失败");
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}