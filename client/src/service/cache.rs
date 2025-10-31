// 缓存ui聊天框的聊天列表
// 实现三级缓存：内存 -> SQLite -> 远程服务器

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use crate::repository::message::{Message, select_messages};
use crate::service::message::{MessageItem, get_session_messages};
use core::request::Page;

// 三级缓存结构
pub struct Cache {
    // 第一级：内存缓存
    memory_cache: Arc<RwLock<HashMap<i64, Vec<Message>>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            memory_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 从三级缓存中获取消息
    /// 1. 首先检查内存缓存
    /// 2. 然后检查SQLite数据库
    /// 3. 最后从远程服务器获取
    pub async fn get_messages(&self, room_id: i64, token: &str, page: Page) -> Result<Vec<Message>> {
        // 1. 检查内存缓存
        {
            let cache = self.memory_cache.read().await;
            if let Some(messages) = cache.get(&room_id) {
                // 对于内存缓存，我们简单地返回所有消息
                // 在实际应用中，可能需要根据分页参数进行处理
                return Ok(messages.clone());
            }
        }

        // 2. 检查SQLite数据库
        match self.get_from_sqlite(room_id, page).await {
            Ok(messages) => {
                // 将结果存入内存缓存
                {
                    let mut cache = self.memory_cache.write().await;
                    cache.insert(room_id, messages.clone());
                }
                return Ok(messages);
            }
            Err(e) => {
                eprintln!("从SQLite获取消息失败: {}", e);
            }
        }

        // 3. 从远程服务器获取
        match self.get_from_remote(room_id, token).await {
            Ok(messages) => {
                // 将结果存入内存缓存和SQLite数据库
                {
                    let mut cache = self.memory_cache.write().await;
                    cache.insert(room_id, messages.clone());
                }
                
                // 异步保存到SQLite，不阻塞当前操作
                let messages_clone = messages.clone();
                for message in &messages_clone {
                    if let Err(e) = self.save_to_sqlite(message).await {
                        eprintln!("保存消息到SQLite失败: {}", e);
                    }
                }
                
                Ok(messages)
            }
            Err(e) => {
                eprintln!("从远程服务器获取消息失败: {}", e);
                // 如果所有缓存级别都失败，返回空列表
                Ok(vec![])
            }
        }
    }

    /// 从SQLite数据库获取消息
    async fn get_from_sqlite(&self, room_id: i64, page: Page) -> Result<Vec<Message>> {
        let (messages, _total) = select_messages(room_id, page).await?;
        Ok(messages)
    }

    /// 将消息保存到SQLite数据库
    async fn save_to_sqlite(&self, message: &Message) -> Result<()> {
        crate::repository::message::save(message).await?;
        Ok(())
    }

    /// 从远程服务器获取消息
    async fn get_from_remote(&self, room_id: i64, token: &str) -> Result<Vec<Message>> {
        // 注意：这里需要将会话ID映射到房间ID
        // 在实际实现中，您可能需要一个映射机制
        let remote_messages = get_session_messages(token, room_id)?;
        
        // 转换为本地消息格式
        let messages: Vec<Message> = remote_messages.into_iter().map(|item| Message {
            id: item.id,
            speaker_id: item.sender_id,
            receiver_id: 0, // 需要根据实际情况设置
            room_id,
            message_type: 1, // 默认文本消息
            content: item.content,
            timestamp: item.createtime,
            nickname: item.sender_name,
        }).collect();
        
        Ok(messages)
    }

    /// 向缓存中添加单条消息
    pub async fn add_message(&self, room_id: i64, message: Message) {
        // 添加到内存缓存
        {
            let mut cache = self.memory_cache.write().await;
            if let Some(messages) = cache.get_mut(&room_id) {
                messages.push(message.clone());
            } else {
                cache.insert(room_id, vec![message.clone()]);
            }
        }

        // 保存到SQLite
        if let Err(e) = self.save_to_sqlite(&message).await {
            eprintln!("保存消息到SQLite失败: {}", e);
        }
    }

    /// 清除指定房间的缓存
    pub async fn invalidate_room_cache(&self, room_id: i64) {
        // 清除内存缓存
        {
            let mut cache = self.memory_cache.write().await;
            cache.remove(&room_id);
        }
        
        // 注意：在实际应用中，您可能还需要清除SQLite中的相关数据
        // 或者标记为需要更新
    }

    /// 清除所有缓存
    pub async fn clear_cache(&self) {
        let mut cache = self.memory_cache.write().await;
        cache.clear();
        // 注意：在实际应用中，您可能还需要处理SQLite数据
    }
}