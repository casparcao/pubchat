use core::proto::{codec::encode, message::{ChatRequest, ChatType, Message, Type}};
use std::sync::Arc;

use crate::ui::component::chat::ChatComponent;
use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf, sync::Mutex};

impl ChatComponent {
    /// 发送消息的主要逻辑
    pub fn send_message(&mut self, stream: &Arc<Mutex<OwnedWriteHalf>>) -> bool {
        if self.input.is_empty() {
            return false;
        }

        // 处理命令
        let should_exit = if self.input.starts_with('/') {
            self.handle_command()
        } else {
            if self.session.is_none() {
                return false;
            }
            // 实际通过TCP发送消息
            let content = self.input.clone();
            // 获取接收者ID，如果找不到则使用默认值
            let session_id = self.session.unwrap().id;
            let stream_clone = stream.clone();
            // 创建聊天请求消息
            let chat_request = Message {
                id: 2, // 简化处理，实际应该使用唯一ID生成器
                ts: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                mtype: Type::ChatRequest as i32,
                content: Some(core::proto::message::message::Content::ChatRequest(ChatRequest{
                    sender: 0, // 使用真实的用户ID
                    session: session_id as u64,
                    receivers: vec![session_id as u64],
                    ctype: ChatType::Text as i32,
                    message: content.clone(),
                    ts: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                    uname: "".to_string(), // 使用真实的用户名
                })),
            };
            
            // 发送消息
            let encoded = match encode(&chat_request) {
                Ok(data) => data,
                Err(e) => {
                    log::error!("Failed to encode message: {}", e);
                    return false;
                }
            };
            let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(self.do_send_message(stream_clone, encoded));
            false
        };
        self.input.clear();
        self.mode = crate::ui::models::Mode::Normal;
        should_exit
    }

    pub async fn do_send_message(&self, stream_clone: Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>, encoded: Vec<u8>) {
        // Write to the stream directly without contention
        let mut stream_guard = stream_clone.lock().await;
        match stream_guard.write_all(&encoded).await {
            Ok(_) => {
                if let Err(e) = stream_guard.flush().await {
                    log::error!("Failed to flush stream: {}", e);
                } 
            },
            Err(e) => {
                log::error!("Failed to send message: {}", e);
            }
        }
    }

    /// 处理命令
    pub fn handle_command(&mut self) -> bool {
        let mut should_exit = false;
        let cmd = self.input.split_whitespace().next().unwrap_or("");
        
        match cmd {
            "/help" => {
                // if let Some(messages) = self.messages.get_mut(&target) {
                //     messages.push(MessageItem::system("Available commands:"));
                //     messages.push(MessageItem::system("/help - Show this help"));
                //     messages.push(MessageItem::system("/friends - Open friends list"));
                //     messages.push(MessageItem::system("/clear - Clear chat history"));
                //     messages.push(MessageItem::system("/quit or /exit - Exit the application"));
                // }
            }
            "/friends" => {
                // 切换到好友列表视图
                // self.view = View::Contact;
                
                // if let Some(messages) = self.messages.get_mut(&target) {
                //     messages.push(MessageItem::system("Opening friends list..."));
                // }
            }
            "/clear" => {
                // if let Some(messages) = self.messages.get_mut(&target) {
                //     messages.clear();
                // }
            }
            "/quit" | "/exit" => {
                should_exit = true;
            }
            _ => {
                // if let Some(messages) = self.messages.get_mut(&target) {
                //     messages.push(MessageItem::system(&format!("Unknown command: {}", cmd)));
                // }
            }
        }
        
        self.input.clear();
        self.mode = crate::ui::models::Mode::Normal;
        should_exit
    }
}