use anyhow::{Ok, Result};
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
use crate::manager::Client;
use crate::queue;
pub async fn handle(message: &Message) -> Result<()> { 
    Ok(())
}