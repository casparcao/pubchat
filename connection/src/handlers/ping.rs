use anyhow::{Ok, Result};
use core::proto::message::Message;
pub async fn handle(_message: &Message) -> Result<()> { 
    Ok(())
}