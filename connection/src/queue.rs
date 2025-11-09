use anyhow::Result;
use deadpool::{managed::Pool, Runtime};
use deadpool_lapin::{Config, Manager};
use futures::StreamExt;
use lapin::{
    options::*, BasicProperties
};
use log::{info, error};
use core::proto::message::{Chrs, Message, Type, message::Content};
use std::{sync::OnceLock};
use crate::connection;


pub static POOL: OnceLock<Pool<Manager>> = OnceLock::new();

pub async fn init() -> Result<()> {
    let connstring = dotenv::var("RABBITMQ_ADDR").expect("请设置RabbitMQ连接地址RABBITMQ_ADDR");
    let session_message_queue_name = dotenv::var("RABBITMQ_SESSION_MESSAGE_QUEUE_NAME").expect("请设置RabbitMQ队列名称RABBITMQ_SESSION_MESSAGE_QUEUE_NAME");
    let exc_name = dotenv::var("RABBITMQ_MESSAGE_EXCHANGE").expect("请设置RabbitMQ交换机名称RABBITMQ_MESSAGE_EXCHANGE");
    let mut cfg = Config::default();
    cfg.url = Some(connstring.into());
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
    let channel = pool.get().await?.create_channel().await?;
    let _queue = channel
            .queue_declare(
                &session_message_queue_name,
                QueueDeclareOptions::default(),
                Default::default(),
            )
            .await?; 
    let _exchange = channel
            .exchange_declare(
                &exc_name,
                lapin::ExchangeKind::Topic,
                ExchangeDeclareOptions::default(),
                Default::default(),
            )
            .await?;
    channel
            .queue_bind(
                &session_message_queue_name, 
                &exc_name, 
                "#", 
                QueueBindOptions::default(), 
                Default::default()).await?;
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
    let session_message_queue_name = dotenv::var("RABBITMQ_SESSION_MESSAGE_QUEUE_NAME").expect("请设置RabbitMQ队列名称RABBITMQ_SESSION_MESSAGE_QUEUE_NAME");
    let pool = POOL.get();
    let pool = pool.expect("");
    let conn = pool.get().await?;
    let channel= conn.create_channel().await?;
    // 创建消费者
    let mut consumer = channel
        .basic_consume(
            &session_message_queue_name,
            "connection_consumer",
            lapin::options::BasicConsumeOptions::default(),
            Default::default(),
        )
        .await?;

    info!("Started consuming messages from RabbitMQ queue: {}", session_message_queue_name);
    // 处理传入的消息
    while let Some(delivery) = consumer.next().await {
        if let std::result::Result::Ok(delivery) = delivery {
            // 解析消息
            match serde_json::from_slice::<Message>(&delivery.data) {
                std::result::Result::Ok(message) => {
                    info!("Received message from RabbitMQ: type={:?}", message.mtype);
                    // 确定消息接收者
                    match message.mtype {
                        t if t == Type::Chrt as i32 => {
                            if let Some(Content::Chrt(resp)) = &message.content {
                                // 如果有目标用户，则发送消息到客户端
                                for receiver in &resp.receivers {
                                    let response = Message {
                                        id: message.id,
                                        ts: message.ts,
                                        mtype: Type::Chrs as i32,
                                        content: Some(Content::Chrs(Chrs{
                                            sender: resp.sender,
                                            receiver: *receiver,
                                            session: resp.session,
                                            ctype: resp.ctype,
                                            message: resp.message
                                                .as_ref()
                                                .map(|m|core::proto::message::chrs::Message::from(m)),
                                            ts: resp.ts,
                                            uname: resp.uname.clone(),

                                        }))
                                    };
                                    if let Err(e) = connection::send_message(*receiver , &response).await {
                                        error!("Failed to send message to client {}: {}", receiver, e);
                                    }
                                }
                            } else {
                                error!("Invalid chat message");
                            }
                        },
                        _ => {
                            error!("Unhandled message type: {}", message.mtype);
                        },
                    };
                    
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
    let queue_name = dotenv::var("RABBITMQ_SESSION_MESSAGE_QUEUE_NAME").expect("请设置RabbitMQ队列名称RABBITMQ_SESSION_MESSAGE_QUEUE_NAME");
    let exc_name = dotenv::var("RABBITMQ_MESSAGE_EXCHANGE").expect("请设置RabbitMQ交换机名称RABBITMQ_MESSAGE_EXCHANGE");
    let payload = serde_json::to_vec(message)?;
    let pool = POOL.get();
    let pool = pool.expect("");
    let conn = pool.get().await?;
    let channel= conn.create_channel().await?;
    
    let _confirm = channel.basic_publish(
            &exc_name, // default exchange
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