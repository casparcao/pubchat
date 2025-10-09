use anyhow::{Ok, Result};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use tracing::{info, error, Level};
use tracing_subscriber;
use core::proto::message::{Message, ConnectResponse, Type, message};
use core::proto::codec::{decode, encode};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

// 存储所有连接的客户端
struct Client {
    uid: u64,
    socket: TcpStream,
}

struct ConnectionManager {
    clients: HashMap<u64, Client>,
    // 使用广播通道来转发消息给所有相关客户端
    message_sender: broadcast::Sender<Message>,
}

impl ConnectionManager {
    fn new() -> Self {
        let (message_sender, _) = broadcast::channel(100);
        
        Self {
            clients: HashMap::new(),
            message_sender,
        }
    }
    
    fn add_client(&mut self, uid: u64, client: Client) {
        self.clients.insert(uid, client);
    }
    
    fn broadcast_message(&self, message: &Message) -> Result<()> {
        let _ = self.message_sender.send(message.clone());
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

    // 注册客户端到连接管理器
    let mut manager = connection_manager.lock().unwrap();

    // 使用两个任务分别处理接收和发送
    let client = Client {
        uid,
        socket: socket,
    };
    manager.add_client(uid, client);


    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let connection_manager = Arc::new(Mutex::new(ConnectionManager::new()));

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