use anyhow::Result;
use tokio::{io::AsyncWriteExt, sync::Mutex};
use tracing::{info, warn, error};
use core::proto::message::Message;
use core::response::ApiErr;
use std::{collections::HashMap, sync::OnceLock};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use core::proto::message::{ConnectResponse, Type, message, ChatResponse};
use core::proto::codec::{decode, encode};
use std::sync::Arc;
use crate::queue;
use crate::handlers;


// 存储所有连接的客户端
#[derive(Debug)]
pub struct Client {
    pub uid: u64,
    pub writer: Arc<Mutex<OwnedWriteHalf>>,
}

pub static CLIENTS: OnceLock<Mutex<HashMap<u64, Client>>> = OnceLock::new();

pub async fn init() {
    CLIENTS.set(Mutex::new(HashMap::new())).expect("初始化客户端列表失败");
}

pub async fn add_client(uid: u64, client: Client) {
    let mut lock = CLIENTS.get().expect("获取客户端列表失败").lock().await;
    lock.insert(uid, client);
}

//向特定用户发送消息
pub async fn send_message(target_uid: u64, message: &Message) -> Result<()> {
    let lock = CLIENTS.get().expect("获取客户端列表失败").lock().await;
    if let Some(client) = lock.get(&target_uid) {
        let encoded = core::proto::codec::encode(message)?;
        let mut writer = client.writer.lock().await;
        writer.write_all(&encoded).await?;
        writer.flush().await?;
        info!("Message sent to client {}", target_uid);
    } else {
        warn!("Client {} not found, message not sent", target_uid);
    }
    Ok(())
}


pub async fn handle_client(
    socket: TcpStream,
) -> Result<()> {
    let peer_addr = socket.peer_addr()?;
    info!("New client connected: {}", peer_addr);

    let (mut reader, writer) = socket.into_split();
    let message = decode::<Message, _>(&mut reader).await?;
    info!("Received message type: {:?}", message.r#type);

    if message.r#type == Type::ConnectRequest as i32 {
        let client: Client = handlers::connect::handle(&message, writer).await?;
        add_client(client.uid, client).await;
    }
    loop {
        // Read the connection request
        let message = decode::<Message, _>(&mut reader).await?;
        info!("Received message type: {:?}", message.r#type);

        if message.r#type == Type::Ping as i32 {
            let r = handlers::ping::handle(&message).await;
            if let Err(e) = r {
                error!("Failed to handle ping: {}", e);
            }
        }
        if message.r#type == Type::ChatRequest as i32 {
            let r = handlers::chat::handle(message).await;
            if let Err(e) = r {
                error!("Failed to handle chat: {}", e);
            }
        }
    }
}

async fn receive_messages(
    mut reader: OwnedReadHalf,
    uid: u64,
) -> Result<()> {
    loop {
        
    }
    
    info!("Client message handling ended for user {}", uid);
    Ok(())
}