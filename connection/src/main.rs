use anyhow::Result;
use tokio::net::TcpListener;
use tracing::{info, error, Level};
use tracing_subscriber;
use dotenv::dotenv;
mod connection;
mod queue;
mod handlers;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    core::log::init(Some(".pubchat_connection.log"));
    // Initialize RabbitMQ
    core::auth::init();
    queue::init().await?;
    connection::init().await;
    
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("Connection service listening on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            if let Err(e) = connection::handle_client(socket).await {
                error!("Error handling client: {}", e);
            }
        });
    }
}