use anyhow::{Ok, Result};
use pubchat::core::message::Message;
pub async fn handle(_message: &Message) -> Result<()> { 
    Ok(())
}