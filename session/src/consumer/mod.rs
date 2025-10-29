mod message;

use anyhow::Result;

pub async fn init() {
    // Start RabbitMQ consumer in a separate task
    // let message_service_clone = message_service.clone();
    tokio::spawn(async move {
        let rabbitmq_addr = std::env::var("RABBITMQ_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".to_string());
        let queue_name = std::env::var("RABBITMQ_QUEUE_NAME").unwrap_or_else(|_| "chat_messages".to_string());
        
        if let Err(e) = message::consume_messages(&rabbitmq_addr, &queue_name).await {
            eprintln!("Error consuming messages: {}", e);
        }
    });
}
