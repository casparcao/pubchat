use anyhow::{Ok, Result};
use futures::StreamExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use tracing::{info, error, warn, Level};
use tracing_subscriber;
use core::proto::message::{Message, ConnectResponse, Type, message, ChatResponse};
use core::proto::codec::{decode, encode};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::broadcast;
use lapin::{BasicProperties, message::DeliveryResult};
use serde_json;

use crate::queue::RabbitMqManager;

mod queue;

// 存储所有连接的客户端
struct Client {
    uid: u64,
    writer: Arc<Mutex<OwnedWriteHalf>>,
}

struct ConnectionManager {
    clients: HashMap<u64, Client>,
    // 使用广播通道来转发消息给所有相关客户端
    message_sender: broadcast::Sender<Message>,
    // RabbitMQ管理器用于消息投递
    rabbitmq_manager: queue::RabbitMqManager,
}

impl ConnectionManager {
    fn new(rabbitmq_manager: queue::RabbitMqManager) -> Self {
        let (message_sender, _) = broadcast::channel(100);
        
        Self {
            clients: HashMap::new(),
            message_sender,
            rabbitmq_manager,
        }
    }
    
    fn add_client(&mut self, uid: u64, client: Client) {
        self.clients.insert(uid, client);
    }
    
    fn broadcast_message(&self, message: &Message) -> Result<()> {
        let _ = self.message_sender.send(message.clone());
        Ok(())
    }
    
    async fn publish_to_rabbitmq(&self, message: &Message) -> Result<()> {
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
    async fn send_message_to_client(&self, target_uid: u64, message: &Message) -> Result<()> {
        if let Some(client) = self.clients.get(&target_uid) {
            let encoded = encode(message)?;
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

async fn handle_client(
    mut socket: TcpStream,
    connection_manager: Arc<Mutex<ConnectionManager>>
) -> Result<()> {
    let peer_addr = socket.peer_addr()?;
    info!("New client connected: {}", peer_addr);

    // Read the connection request
    let connect_request = decode::<Message, _>(&mut socket).await?;
    info!("Received message type: {:?}", connect_request.r#type);

    let mut uid = 0u64;
    if connect_request.r#type == Type::ConnectRequest as i32 {
        if let Some(message::Content::ConnectRequest(req)) = connect_request.content {
            info!("Connect request with token: {}", req.token);
            
            // 在实际应用中，这里需要验证token并获取用户ID
            uid = 12345; // 临时用户ID
            
            // Create a connection response
            let response = Message {
                id: connect_request.id,
                ts: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                r#type: Type::ConnectResponse as i32,
                content: Some(message::Content::ConnectResponse(ConnectResponse {
                    code: 0,
                    message: "Connected successfully".to_string(),
                    uid,
                })),
            };

            // Send the response
            let encoded = encode(&response)?;
            socket.write_all(&encoded).await?;
            socket.flush().await?; // Ensure the data is sent immediately
            info!("Sent connection response to client");
        }
    }

    // 处理客户端消息
    handle_client_messages(socket, connection_manager.clone(), uid).await
}

async fn handle_client_messages(
    socket: TcpStream,
    connection_manager: Arc<Mutex<ConnectionManager>>,
    uid: u64,
) -> Result<()> {
    // Split the socket into read and write halves
    let (mut reader, writer) = socket.into_split();
    
    // 注册客户端到连接管理器
    {
        let mut manager = connection_manager.lock().await;
        let client = Client {
            uid,
            writer: Arc::new(Mutex::new(writer)),
        };
        manager.add_client(uid, client);
    }
    
    info!("Client {} registered in connection manager", uid);
    // Main message receiving loop
    loop {
        // Read incoming message
        let message = match decode::<Message, _>(&mut reader).await {
            std::result::Result::Ok(msg) => msg,
            Err(e) => {
                error!("Failed to decode message from client {}: {}", uid, e);
                break;
            }
        };
        
        info!("Received message from client {}: type={:?}, id={}", 
              uid, message.r#type, message.id);
        
        // Process different message types
        match message.r#type {
            t if t == Type::ChatRequest as i32 => {
                if let Some(message::Content::ChatRequest(chat_req)) = message.content.clone() {
                    info!("Processing ChatRequest from user {}: room={}, message='{}', timestamp={}",
                          chat_req.speaker, chat_req.room, chat_req.message, chat_req.ts);
                    
                    // Create a chat response mirroring the request
                    let chat_response = Message {
                        id: message.id,
                        ts: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as u64,
                        r#type: Type::ChatResponse as i32,
                        content: Some(message::Content::ChatResponse(ChatResponse {
                            speaker: chat_req.speaker,
                            room: chat_req.room,
                            r#type: chat_req.r#type,
                            message: chat_req.message,
                            ts: chat_req.ts,
                            nickname: chat_req.nickname,
                        })),
                    };
                    
                    // Broadcast the message to all clients
                    {
                        let manager = connection_manager.lock().await;
                        if let Err(e) = manager.broadcast_message(&chat_response) {
                            error!("Failed to broadcast message: {}", e);
                        }
                        
                        // Publish message to RabbitMQ
                        if let Err(e) = manager.publish_to_rabbitmq(&chat_response).await {
                            error!("Failed to publish message to RabbitMQ: {}", e);
                        }
                    }
                    info!("Broadcast ChatResponse for user {} in room {}", 
                          chat_req.speaker, chat_req.room);
                }
            },
            t if t == Type::Ping as i32 => {
                info!("Received ping from client {}", uid);
                // Handle ping-pong for heartbeat
                // Implementation would go here
            },
            _ => {
                warn!("Unknown message type received from client {}: {:?}", uid, message.r#type);
            }
        }
    }
    
    info!("Client message handling ended for user {}", uid);
    Ok(())
}

// 新增函数：处理来自RabbitMQ的消息
async fn handle_rabbitmq_messages(
    connection_manager: Arc<Mutex<ConnectionManager>>,
    rabbitmq_manager: queue::RabbitMqManager,
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
        if let Err(e) = handle_rabbitmq_messages(manager_clone, rabbitmq_manager).await {
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
            if let Err(e) = handle_client(socket, manager).await {
                error!("Error handling client: {}", e);
            }
        });
    }
}