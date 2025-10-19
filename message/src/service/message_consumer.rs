use anyhow::Result;
use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
    Connection, ConnectionProperties,
};
use serde_json;
use crate::model::message::Message;
use crate::service::message::MessageService;

pub struct MessageConsumer {
    rabbitmq_addr: String,
    queue_name: String,
}

impl MessageConsumer {
    pub fn new(rabbitmq_addr: String, queue_name: String) -> Self {
        Self {
            rabbitmq_addr,
            queue_name,
        }
    }

    pub async fn consume_messages(&self, message_service: MessageService) -> Result<()> {
        let conn = Connection::connect(&self.rabbitmq_addr, ConnectionProperties::default())
            .await?;

        let channel = conn.create_channel().await?;

        // Declare queue (this won't create a new one if it already exists)
        let queue = channel
            .queue_declare(
                &self.queue_name,
                lapin::options::QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        let mut consumer = channel
            .basic_consume(
                &self.queue_name,
                "message_service_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        println!("Started consuming messages from RabbitMQ queue: {}", self.queue_name);

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                match serde_json::from_slice::<core::proto::message::Message>(&delivery.data) {
                    Ok(proto_message) => {
                        // Convert the proto message to our database message model
                        if let Some(core::proto::message::message::Content::ChatResponse(chat_response)) = 
                            proto_message.content {
                            
                            let message = Message {
                                id: proto_message.id as i64,
                                speaker_id: chat_response.speaker as i64,
                                room_id: chat_response.room as i64,
                                message_type: chat_response.r#type,
                                content: chat_response.message,
                                timestamp: chat_response.ts as i64,
                                nickname: chat_response.nickname,
                            };

                            // Save the message to the database
                            if let Err(e) = message_service.save_message(message).await {
                                eprintln!("Failed to save message to database: {}", e);
                            }
                        }
                        
                        // Acknowledge the message
                        if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                            eprintln!("Failed to acknowledge message: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse message from RabbitMQ: {}", e);
                        // Even if parsing fails, acknowledge the message to avoid message buildup
                        if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                            eprintln!("Failed to acknowledge message: {}", e);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}