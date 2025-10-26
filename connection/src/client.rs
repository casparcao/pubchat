use anyhow::Result;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tracing::{info, error, warn};
use core::proto::message::{Message, ConnectResponse, Type, message, ChatResponse};
use core::proto::codec::{decode, encode};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::manager;
use crate::queue;

// 存储所有连接的客户端
#[derive(Debug)]
pub struct Client {
    pub uid: u64,
    pub writer: Arc<Mutex<OwnedWriteHalf>>,
}

pub async fn handle_client(
    mut socket: TcpStream,
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
    handle_client_messages(socket, uid).await
}

async fn handle_client_messages(
    socket: TcpStream,
    uid: u64,
) -> Result<()> {
    // Split the socket into read and write halves
    let (mut reader, writer) = socket.into_split();
    
    // 注册客户端到连接管理器
    let client = Client {
        uid,
        writer: Arc::new(Mutex::new(writer)),
    };
    manager::add_client(uid, client).await;
    
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
                        // Publish message to RabbitMQ
                        if let Err(e) = queue::publish(&chat_response).await {
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