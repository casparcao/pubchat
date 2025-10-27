use anyhow::Result;
use tokio::net::TcpListener;
use tracing::{info, error, Level};
use tracing_subscriber;
use dotenv::dotenv;
mod manager;
mod queue;
mod handlers;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    // Initialize RabbitMQ
    queue::init().await?;
    manager::init().await;
    
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("Connection service listening on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            if let Err(e) = manager::handle_client(socket).await {
                error!("Error handling client: {}", e);
            }
        });
    }
}