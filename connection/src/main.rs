use anyhow::{Ok, Result};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{info, error, Level};
use tracing_subscriber;
use core::proto::message::{Message, ConnectRequest, ConnectResponse, Type, message};
use core::proto::codec::{decode, encode};

async fn handle_client(mut socket: TcpStream) -> Result<()> {
    let peer_addr = socket.peer_addr()?;
    info!("New client connected: {}", peer_addr);

    // Read the connection request
    let connect_request = decode::<Message, _>(&mut socket).await?;
    info!("Received message type: {:?}", connect_request.r#type);

    if connect_request.r#type == Type::ConnectRequest as i32 {
        if let Some(message::Content::ConnectRequest(req)) = connect_request.content {
            info!("Connect request with token: {}", req.token);
            
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
                    uid: 12345, // This would normally come from authentication
                })),
            };

            // Send the response
            let encoded = encode(&response)?;
            socket.write_all(&encoded).await?;
            
            // Keep the connection alive for further communication
            // In a real implementation, this would be a more complex loop
            // handling multiple message types
            loop {
                // For now, just echo back any messages received
                match decode::<Message, _>(&mut socket).await {
                    std::result::Result::Ok(msg) => {
                        info!("Received message: {:?}", msg);
                        // Echo the message back
                        let echo = encode(&msg)?;
                        socket.write_all(&echo).await?;
                    }
                    Err(e) => {
                        error!("Error decoding message: {}", e);
                        break;
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

    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("Connection service listening on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                error!("Error handling client: {}", e);
            }
        });
    }
}
