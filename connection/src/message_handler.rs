use anyhow::Result;
use futures::StreamExt;
use tracing::{info, error};
use core::proto::message::{Message, Type, message};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::manager::ConnectionManager;
use crate::queue::RabbitMqManager;

// 新增函数：处理来自RabbitMQ的消息
pub async fn handle_rabbitmq_messages(
    connection_manager: Arc<Mutex<ConnectionManager>>,
    rabbitmq_manager: RabbitMqManager,
) -> Result<()> {
    let channel = rabbitmq_manager.get_channel().clone();
    let queue_name = rabbitmq_manager.get_queue_name().to_string();
    
    // 创建消费者
    let mut consumer = channel
        .basic_consume(
            &queue_name,
            "connection_service_consumer",
            lapin::options::BasicConsumeOptions::default(),
            Default::default(),
        )
        .await?;

    info!("Started consuming messages from RabbitMQ queue: {}", queue_name);
    // 处理传入的消息
    while let Some(delivery) = consumer.next().await {
        if let std::result::Result::Ok(delivery) = delivery {
            // 解析消息
            match serde_json::from_slice::<Message>(&delivery.data) {
                std::result::Result::Ok(message) => {
                    info!("Received message from RabbitMQ: type={:?}", message.r#type);
                    
                    // 确定消息接收者
                    let target_uid = match message.r#type {
                        t if t == Type::ChatResponse as i32 => {
                            if let Some(message::Content::ChatResponse(chat_resp)) = &message.content {
                                Some(chat_resp.speaker) // 发送给说话者的客户端
                            } else {
                                None
                            }
                        },
                        _ => None,
                    };
                    
                    // 如果有目标用户，则发送消息到客户端
                    if let Some(uid) = target_uid {
                        let manager = connection_manager.lock().await;
                        if let Err(e) = manager.send_message_to_client(uid, &message).await {
                            error!("Failed to send message to client {}: {}", uid, e);
                        }
                    }
                    
                    // 确认消息处理完成
                    if let Err(e) = delivery.ack(lapin::options::BasicAckOptions::default()).await {
                        error!("Failed to acknowledge message: {}", e);
                    }
                },
                Err(e) => {
                    error!("Failed to parse message from RabbitMQ: {}", e);
                    // 即使解析失败也确认消息，避免消息堆积
                    if let Err(e) = delivery.ack(lapin::options::BasicAckOptions::default()).await {
                        error!("Failed to acknowledge message: {}", e);
                    }
                }
            }
        }
    }
    
    Ok(())
}