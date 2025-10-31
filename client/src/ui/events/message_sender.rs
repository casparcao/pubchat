use core::proto::{codec::encode, message::{Chat, ChatType, Message, Type}};

use crate::ui::models::{App, View};
use tokio::io::AsyncWriteExt;

impl App {
    /// 发送消息的主要逻辑
    pub fn send_message(&mut self) -> bool {
        if self.input.is_empty() {
            return false;
        }

        // 处理命令
        let should_exit = if self.input.starts_with('/') {
            self.handle_command()
        } else {
            // 发送普通消息
            match &self.current_view {
                View::Chat { session } => {
                    // 实际通过TCP发送消息
                    let content = self.input.clone();
                    let current_user_id = self.current_user_id; // 获取当前用户ID
                    let current_user = self.current_user.clone(); // 获取当前用户名
                    // 获取接收者ID，如果找不到则使用默认值
                    let session_id = session.id;
                    
                    if let Some(stream) = &self.stream {
                        let stream_clone = stream.clone();
                        tokio::spawn(async move {
                            // 创建聊天请求消息
                            let chat_request = Message {
                                id: 2, // 简化处理，实际应该使用唯一ID生成器
                                ts: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis() as u64,
                                r#type: Type::Chat as i32,
                                content: Some(core::proto::message::message::Content::Chat(Chat{
                                    speaker: current_user_id, // 使用真实的用户ID
                                    receiver: 0, // 使用从好友列表获取的真实ID
                                    room: session_id as u64,
                                    r#type: ChatType::Text as i32,
                                    message: content.clone(),
                                    ts: std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .unwrap()
                                        .as_millis() as u64,
                                    nickname: current_user, // 使用真实的用户名
                                })),
                            };
                            
                            // 发送消息
                            let encoded = match encode(&chat_request) {
                                Ok(data) => data,
                                Err(e) => {
                                    log::error!("Failed to encode message: {}", e);
                                    return;
                                }
                            };
                            
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
                            drop(stream_guard);
                        });
                    }
                    false
                },
                _ => {
                    
                    false
                }
            }
        };

        self.input.clear();
        self.mode = crate::ui::models::Mode::Normal;
        should_exit
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
                self.current_view = View::FriendsList;
                self.selected_friend = None;
                
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