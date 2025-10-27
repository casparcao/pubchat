use anyhow::Result;
use deadpool::{managed::Pool, Runtime};
use deadpool_lapin::{Config, Manager};
use futures::StreamExt;
use lapin::{
    options::*, BasicProperties
};
use tracing::{info, error};
use core::proto::message::{message::Content, Message, Type};
use std::{sync::OnceLock};
use crate::manager;


pub static POOL: OnceLock<Pool<Manager>> = OnceLock::new();

pub async fn init() -> Result<()> {
    let connstring = dotenv::var("RABBITMQ_ADDR").expect("请设置RabbitMQ连接地址RABBITMQ_ADDR");
    let queue_name = dotenv::var("RABBITMQ_QUEUE_NAME").expect("请设置RabbitMQ队列名称RABBITMQ_QUEUE_NAME");
    let mut cfg = Config::default();
    cfg.url = Some(connstring.into());
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
    let channel = pool.get().await?.create_channel().await?;
    let _queue = channel
            .queue_declare(
                &queue_name,
                QueueDeclareOptions::default(),
                Default::default(),
            )
            .await?; 
    // 启动RabbitMQ消息消费
    tokio::spawn(async move {
        if let Err(e) = receive().await {
            error!("Error handling RabbitMQ messages: {}", e);
        }
    });
    POOL.set(pool).expect("Failed to set pool");
    Ok(())
}



pub async fn receive() -> Result<()> {
    let queue_name = dotenv::var("RABBITMQ_QUEUE_NAME").expect("请设置RabbitMQ队列名称RABBITMQ_QUEUE_NAME");
    let pool = POOL.get();
    let pool = pool.expect("");
    let conn = pool.get().await?;
    let channel= conn.create_channel().await?;

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
                        t if t == Type::Chat as i32 => {
                            if let Some(Content::Chat(chat_resp)) = &message.content {
                                Some(chat_resp.speaker) // 发送给说话者的客户端
                            } else {
                                None
                            }
                        },
                        _ => None,
                    };
                    
                    // 如果有目标用户，则发送消息到客户端
                    if let Some(uid) = target_uid {
                        if let Err(e) = manager::send_message(uid, &message).await {
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


pub async fn publish(message: &Message) -> Result<()> {
    let queue_name = dotenv::var("RABBITMQ_QUEUE_NAME").expect("请设置RabbitMQ队列名称RABBITMQ_QUEUE_NAME");
    let payload = serde_json::to_vec(message)?;
    let pool = POOL.get();
    let pool = pool.expect("");
    let conn = pool.get().await?;
    let channel= conn.create_channel().await?;
    
    let _confirm = channel.basic_publish(
            "", // default exchange
            &queue_name,
            lapin::options::BasicPublishOptions::default(),
            &payload,
            BasicProperties::default(),
        )
        .await?
        .await?;
    info!("Message published to RabbitMQ");
    Ok(())
}