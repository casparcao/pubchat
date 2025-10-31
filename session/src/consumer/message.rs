use anyhow::Result;
use futures::StreamExt;
use lapin::{
    Channel, options::{BasicAckOptions, BasicConsumeOptions}, types::FieldTable
};
use serde_json;
use crate::model::message::Message;
use crate::service::message;

pub async fn consume_messages(channel: Channel, queue_name: &str) -> Result<()> {
        let mut consumer = channel
            .basic_consume(
                queue_name,
                "message_service_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        log::info!("Started consuming messages from RabbitMQ queue: {}", queue_name);

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                match serde_json::from_slice::<core::proto::message::Message>(&delivery.data) {
                    Ok(proto_message) => {
                        // Convert the proto message to our database message model
                        if let Some(core::proto::message::message::Content::Chat(chat_response)) = 
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
                            if let Err(e) = message::save_message(message).await {
                                log::error!("Failed to save message to database: {}", e);
                            }
                        }
                        
                        // Acknowledge the message
                        if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                            log::error!("Failed to acknowledge message: {}", e);
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to parse message from RabbitMQ: {}", e);
                        // Even if parsing fails, acknowledge the message to avoid message buildup
                        if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                            log::error!("Failed to acknowledge message: {}", e);
                        }
                    }
                }
            }
        }

        Ok(())
    }
