use crate::ui::models::{App, MessageItem, Contact, View, Mode, Status};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::net::tcp::OwnedWriteHalf;
use core::proto::message::{Message, Type, ChatRequest, ChatType};
use core::proto::codec::encode;

impl App {
    pub fn new() -> Self {
        let mut messages = HashMap::new();
        // 为alice初始化一些消息
        messages.insert("alice".to_string(), vec![
            MessageItem::new("alice".to_string(), "Hello there!".to_string(), false),
            MessageItem::new("You".to_string(), "Hi Alice, how are you?".to_string(), true),
        ]);
        
        Self {
            input: String::new(),
            messages,
            contacts: vec![
                Contact { name: "alice".to_string(), status: Status::Online },
                Contact { name: "bob".to_string(), status: Status::Offline },
            ],
            current_view: View::Chat {
                target: "alice".to_string(),
            },
            mode: Mode::Normal,
            scroll_offset: 0,
            selected_contact: None,
            current_user: "user1".to_string(),
            chat_maximized: false,
            token: None,
            stream: None,
        }
    }
    
    pub fn set_token(&mut self, token: Option<String>) {
        self.token = token;
    }
    
    pub fn set_stream(&mut self, stream: Arc<Mutex<OwnedWriteHalf>>) {
        self.stream = Some(stream);
    }
    
    // 添加接收消息的方法
    pub fn add_received_message(&mut self, chat_req: ChatRequest) {
        let target = chat_req.nickname.clone();
        
        // 确保目标有消息列表
        if !self.messages.contains_key(&target) {
            self.messages.insert(target.clone(), vec![]);
        }
        
        // 添加接收到的消息
        if let Some(messages) = self.messages.get_mut(&target) {
            let msg = MessageItem::new(
                chat_req.nickname,
                chat_req.message,
                false
            );
            messages.push(msg);
        }
    }
    
    // 发送消息的方法
    pub async fn send_message_over_tcp(&self, content: String, target: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(stream) = &self.stream {
            // 创建聊天请求消息
            let chat_request = Message {
                id: 2, // 简化处理，实际应该使用唯一ID生成器
                ts: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                r#type: Type::ChatRequest as i32,
                content: Some(core::proto::message::message::Content::ChatRequest(ChatRequest {
                    speaker: 12345, // 这应该从连接响应中获取
                    room: 0, // 私聊
                    r#type: ChatType::Text as i32,
                    message: content,
                    ts: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                    nickname: self.current_user.clone(),
                })),
            };
            
            // 发送消息
            let encoded = encode(&chat_request)?;
            let mut stream_guard = stream.lock().await;
            stream_guard.write_all(&encoded).await?;
            stream_guard.flush().await?;
        }
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}