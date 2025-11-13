use core::{api::client::blob::{download_file, upload_file}};
use std::sync::Arc;

use crate::{cache, ui::{component::chat::ChatComponent, models::Me}};
use pubchat::core::{codec::encode, message::{Blob, ChatType, Chrt, Message, Text, Type}};
use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf, sync::Mutex};

impl ChatComponent {
    /// 发送消息的主要逻辑
    pub fn send_message(&mut self, me: &Me, stream: &Arc<Mutex<OwnedWriteHalf>>) -> bool {
        if self.input.is_empty() {
            return false;
        }
        // 处理命令
        if self.input.starts_with('/') {
            return self.handle_command(me, stream);
        }
        if let Some(session) = &self.session {
            // 实际通过TCP发送消息
            let content = self.input.clone();
            // 获取接收者ID，如果找不到则使用默认值
            let session_id = session.id;
            let stream_clone = stream.clone();
            // 创建聊天请求消息
            let chat_request = Message {
                id: snowflaker::next_id().unwrap(), // 简化处理，实际应该使用唯一ID生成器
                ts: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                mtype: Type::Chrt as i32,
                content: Some(pubchat::core::message::message::Content::Chrt(Chrt{
                    sender: me.id, // 使用真实的用户ID
                    session: session_id as u64,
                    receivers: session.members.iter()
                        .map(|m| m.id as u64)
                        .filter(|id| *id != me.id)
                        .collect(),
                    ctype: ChatType::Text as i32,
                    message: Some(pubchat::core::message::chrt::Message::Text(Text{ text: content.clone() })),
                    ts: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                    uname: me.name.to_string(), // 使用真实的用户名
                })),
            };
            self.messages.push(crate::ui::models::Message::new(me.name.to_string(), content.clone(), false));
            let message = core::api::types::message::Message{
                id: chat_request.id as i64,
                sender: me.id as i64,
                receiver: 0,
                uname: me.name.to_string(),
                session: session_id,
                mtype: Type::Chrt as i32,
                content: content,
                timestamp: chat_request.ts as i64,
            };
            cache::message_cache().add_message(session.id, message);
            log::info!("Sending message body: {:?}", chat_request);
            // 发送消息
            let encoded = match encode(&chat_request) {
                Ok(data) => data,
                Err(e) => {
                    log::error!("Failed to encode message: {}", e);
                    return false;
                }
            };
            crate::asynrt::get().block_on(self.do_send_message(stream_clone, encoded));
        }
        self.input.clear();
        self.mode = crate::ui::models::Mode::Normal;
        false
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
    pub fn handle_command(&mut self, me: &Me, stream: &Arc<Mutex<OwnedWriteHalf>>) -> bool {
        let mut should_exit = false;
        let input = self.input.clone();
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).unwrap_or(&"");
        
        // Handle built-in commands
        match *cmd {
            "/help" => {
                self.messages.push(crate::ui::models::Message::new(
                    "SYSTEM".to_string(), 
                    "Available commands:".to_string(), 
                    true
                ));
                self.messages.push(crate::ui::models::Message::new(
                    "SYSTEM".to_string(), 
                    "/help - Show this help".to_string(), 
                    true
                ));
                self.messages.push(crate::ui::models::Message::new(
                    "SYSTEM".to_string(), 
                    "/file <path> - Send a file".to_string(), 
                    true
                ));
                self.messages.push(crate::ui::models::Message::new(
                    "SYSTEM".to_string(), 
                    "/download <file_id> <save_path> - Download a file".to_string(), 
                    true
                ));
                self.messages.push(crate::ui::models::Message::new(
                    "SYSTEM".to_string(), 
                    "/friends - Open friends list".to_string(), 
                    true
                ));
                self.messages.push(crate::ui::models::Message::new(
                    "SYSTEM".to_string(), 
                    "/clear - Clear chat history".to_string(), 
                    true
                ));
                self.messages.push(crate::ui::models::Message::new(
                    "SYSTEM".to_string(), 
                    "/quit or /exit - Exit the application".to_string(), 
                    true
                ));

                // Add extension command help if available
                if let Some(plugin_manager) = &self.plugin_manager {
                    let extensions_help = plugin_manager.list_commands();
                    for help_text in extensions_help {
                        self.messages.push(crate::ui::models::Message::new(
                            "SYSTEM".to_string(),
                            help_text,
                            true
                        ));
                    }
                }
                
                // Add info about new command format
                self.messages.push(crate::ui::models::Message::new(
                    "SYSTEM".to_string(),
                    "You can also use !plugin.command format for direct plugin commands".to_string(),
                    true
                ));
            }
            "/file" => {
                if let Some(file_path) = parts.get(1) {
                    match self.send_file(me, stream, file_path) {
                        Ok(_) => {
                            self.messages.push(crate::ui::models::Message::new(
                                "SYSTEM".to_string(), 
                                format!("File {} sent successfully", file_path), 
                                true
                            ));
                        }
                        Err(e) => {
                            self.messages.push(crate::ui::models::Message::new(
                                "SYSTEM".to_string(), 
                                format!("Failed to send file {}: {}", file_path, e), 
                                true
                            ));
                        }
                    }
                } else {
                    self.messages.push(crate::ui::models::Message::new(
                        "SYSTEM".to_string(), 
                        "Usage: /file <path>".to_string(), 
                        true
                    ));
                }
            }
            "/download" => {
                if parts.len() >= 3 {
                    let file_id = parts[1];
                    let save_path = parts[2];
                    
                    match base62::decode(file_id) {
                        Ok(id) => {
                            match download_file(&self.token, id as i64, save_path) {
                                Ok(_) => {
                                    self.messages.push(crate::ui::models::Message::new(
                                        "SYSTEM".to_string(), 
                                        format!("File downloaded successfully to {}", save_path), 
                                        true
                                    ));
                                }
                                Err(e) => {
                                    self.messages.push(crate::ui::models::Message::new(
                                        "SYSTEM".to_string(),
                                        format!("Failed to download file: {}", e),
                                        true
                                    ));
                                }
                            }
                        }
                        Err(e) => {
                            self.messages.push(crate::ui::models::Message::new(
                                "SYSTEM".to_string(),
                                format!("Invalid file ID: {}", e),
                                true
                            ));
                        }
                    }
                } else {
                    self.messages.push(crate::ui::models::Message::new(
                        "SYSTEM".to_string(),
                        "Usage: /download <file_id> <save_path>".to_string(),
                        true
                    ));
                }
            }
            "/friends" => {
                self.current_view = crate::ui::models::View::FriendsList;
                self.selected_friend = None;
                self.messages.push(crate::ui::models::Message::new(
                    "SYSTEM".to_string(),
                    "Opening friends list...".to_string(),
                    true
                ));
            }
            "/clear" => {
                self.messages.clear();
            }
            "/quit" | "/exit" => {
                should_exit = true;
            }
            _ => {
                // Handle plugin commands with new format !plugin.command
                if let Some(plugin_manager) = &self.plugin_manager {
                    let command_args: Vec<&str> = parts.iter().skip(1).cloned().collect();
                    match plugin_manager.handle_command(cmd, command_args) {
                        Ok(Some(result)) => {
                            match result {
                                pubchat::extension::CommandResult::Success(output) => {
                                    self.messages.push(crate::ui::models::Message::new(
                                        "SYSTEM".to_string(),
                                        output,
                                        true
                                    ));
                                }
                                pubchat::extension::CommandResult::Error(error) => {
                                    self.messages.push(crate::ui::models::Message::new(
                                        "SYSTEM".to_string(),
                                        format!("Error: {}", error),
                                        true
                                    ));
                                }
                                pubchat::extension::CommandResult::NotHandled => {
                                    self.messages.push(crate::ui::models::Message::new(
                                        "SYSTEM".to_string(),
                                        format!("Unknown command: {}", cmd),
                                        true
                                    ));
                                }
                            }
                        }
                        Ok(None) => {
                            self.messages.push(crate::ui::models::Message::new(
                                "SYSTEM".to_string(),
                                format!("Unknown command: {}", cmd),
                                true
                            ));
                        }
                        Err(e) => {
                            self.messages.push(crate::ui::models::Message::new(
                                "SYSTEM".to_string(),
                                format!("Error handling command: {}", e),
                                true
                            ));
                        }
                    }
                } else {
                    self.messages.push(crate::ui::models::Message::new(
                        "SYSTEM".to_string(),
                        format!("Unknown command: {}", cmd),
                        true
                    ));
                }
            }
        }
        
        self.input.clear();
        self.mode = crate::ui::models::Mode::Normal;
        should_exit
    }

    /// Send a file to the current session
    fn send_file(&mut self, me: &Me, stream: &Arc<Mutex<OwnedWriteHalf>>, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(session) = &self.session {
            // Upload file to blob service
            let upload_result = upload_file(&self.token, file_path)?;
            // Create file message
            let session_id = session.id;
            let stream_clone = stream.clone();
            // Create chat request with file type
            let chat_request = Message {
                id: snowflaker::next_id().unwrap(),
                ts: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                mtype: Type::Chrt as i32,
                content: Some(pubchat::core::message::message::Content::Chrt(Chrt{
                    sender: me.id,
                    session: session_id as u64,
                    receivers: session.members.iter()
                        .map(|m| m.id as u64)
                        .filter(|id| *id != me.id)
                        .collect(),
                    ctype: ChatType::File as i32,
                    message: Some(pubchat::core::message::chrt::Message::Blob(Blob{
                        id: upload_result.id as u64,
                        name: upload_result.name.clone(),
                        size: upload_result.size.to_string(),
                        exp: upload_result.exp.as_ref().map(|e| e.clone()),
                    })),
                    ts: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                    uname: me.name.to_string(),
                })),
            };
            
            // Add to local message list
            self.messages.push(crate::ui::models::Message::new(
                me.name.to_string(), 
                format!("{}", upload_result), 
                false
            ));
            
            // Add to cache
            let message = core::api::types::message::Message{
                id: chat_request.id as i64,
                sender: me.id as i64,
                receiver: 0,
                uname: me.name.to_string(),
                session: session_id,
                mtype: Type::Chrt as i32,
                content: format!("{}", upload_result),
                timestamp: chat_request.ts as i64,
            };
            cache::message_cache().add_message(session.id, message);
            // Send message
            let encoded = encode(&chat_request)
                .map_err(|e| anyhow::anyhow!("Failed to encode message: {}", e))?;
            crate::asynrt::get().block_on(self.do_send_message(stream_clone, encoded));
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("No session selected").into())
        }
    }
}