use anyhow::Result;
use tokio::{io::AsyncWriteExt, sync::Mutex};
use tracing::{info, warn};
use core::proto::message::Message;
use std::{collections::HashMap, sync::OnceLock};

use crate::client::Client;
pub static CLIENTS: OnceLock<Mutex<HashMap<u64, Client>>> = OnceLock::new();

pub async fn init() {
    CLIENTS.set(Mutex::new(HashMap::new())).expect("初始化客户端列表失败");
}

pub async fn add_client(uid: u64, client: Client) {
    let mut lock = CLIENTS.get().expect("获取客户端列表失败").lock().await;
    lock.insert(uid, client);
}

// 新增方法：向特定用户发送消息
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