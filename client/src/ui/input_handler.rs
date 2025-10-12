use core::proto::{codec::encode, message::{ChatRequest, ChatType, Message, Type}};

use crate::ui::{models::{App, Mode, View, MessageItem}};
use crossterm::event::KeyEvent;
use tokio::io::AsyncWriteExt;

impl App {
    pub fn handle_key_event(&mut self, key: KeyEvent) -> bool {
        let mut should_exit = false;
        match self.mode {
            Mode::Normal => match key.code {
                crossterm::event::KeyCode::Char('q') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                    // Ctrl+Q 退出
                    should_exit = true;
                }
                crossterm::event::KeyCode::Char('i') => {
                    self.mode = Mode::Insert;
                }
                crossterm::event::KeyCode::Char('k') => {
                    // 在联系人视图中向上导航
                    match self.current_view {
                        View::Contacts => {
                            if !self.contacts.is_empty() {
                                if let Some(selected) = self.selected_contact {
                                    self.selected_contact = Some(selected.saturating_sub(1));
                                } else {
                                    self.selected_contact = Some(0);
                                }
                            }
                        }
                        _ => {
                            // 在聊天视图中，k键用于滚动消息
                            if self.scroll_offset > 0 {
                                self.scroll_offset -= 1;
                            }
                        }
                    }
                }
                crossterm::event::KeyCode::Char('j') => {
                    // 在联系人视图中向下导航
                    match self.current_view {
                        View::Contacts => {
                            if !self.contacts.is_empty() {
                                if let Some(selected) = self.selected_contact {
                                    self.selected_contact = Some((selected + 1).min(self.contacts.len() - 1));
                                } else {
                                    self.selected_contact = Some(0);
                                }
                            }
                        }
                        _ => {
                            // 在聊天视图中，j键用于滚动消息
                            self.scroll_offset += 1;
                        }
                    }
                }
                crossterm::event::KeyCode::Char('h') => {
                    // 切换到联系人视图
                    self.current_view = View::Contacts;
                }
                crossterm::event::KeyCode::Char('m') => {
                    // 切换聊天窗口最大化状态
                    if matches!(self.current_view, View::Chat { .. }) {
                        self.chat_maximized = !self.chat_maximized;
                    }
                }
                crossterm::event::KeyCode::Enter => {
                    // 在联系人视图中按Enter选择
                    match &self.current_view {
                        View::Contacts => {
                            if let Some(index) = self.selected_contact {
                                if index < self.contacts.len() {
                                    let target = self.contacts[index].name.clone();
                                    self.current_view = View::Chat { target: target.clone() };
                                    
                                    // 确保目标有消息列表
                                    if !self.messages.contains_key(&target) {
                                        self.messages.insert(target.clone(), vec![]);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                crossterm::event::KeyCode::Tab => {
                    // 在不同视图间切换
                    self.current_view = match self.current_view {
                        View::Chat { .. } => View::Contacts,
                        View::Contacts => View::Chat { target: "alice".to_string() },
                    };
                }
                _ => {}
            },
            Mode::Insert => match key.code {
                crossterm::event::KeyCode::Esc => {
                    self.mode = Mode::Normal;
                }
                crossterm::event::KeyCode::Enter => {
                    self.send_message();
                }
                crossterm::event::KeyCode::Char(c) => {
                    self.input.push(c);
                }
                crossterm::event::KeyCode::Backspace => {
                    self.input.pop();
                }
                _ => {}
            },
        }
        should_exit
    }

    fn send_message(&mut self) -> bool {
        if self.input.is_empty() {
            return false;
        }

        // 处理命令
        let should_exit = if self.input.starts_with('/') {
            self.handle_command()
        } else {
            // 发送普通消息
            match &self.current_view {
                View::Chat { target } => {
                    // 确保目标有消息列表
                    if !self.messages.contains_key(target) {
                        self.messages.insert(target.clone(), vec![]);
                    }
                    
                    // 添加发送的消息到UI
                    if let Some(messages) = self.messages.get_mut(target) {
                        let msg = MessageItem::new(
                            "You".to_string(), 
                            self.input.clone(), 
                            true
                        );
                        messages.push(msg);
                    }
                    
                    // 实际通过TCP发送消息
                    let content = self.input.clone();
                    let target_clone = target.clone();
                    if let Some(stream) = &self.stream {
                        let stream_clone = stream.clone();
                        tokio::spawn(async move {
                            // 创建聊天请求消息
                            let room_id = 0; // 私聊
                            
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
                                    room: room_id,
                                    r#type: ChatType::Text as i32,
                                    message: content.clone(),
                                    ts: std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .unwrap()
                                        .as_millis() as u64,
                                    nickname: "You".to_string(), // 这应该从用户信息中获取
                                })),
                            };
                            
                            // 发送消息
                            let encoded = match encode(&chat_request) {
                                Ok(data) => data,
                                Err(e) => {
                                    eprintln!("Failed to encode message: {}", e);
                                    return;
                                }
                            };
                            
                            // Write to the stream directly without contention
                            let mut stream_guard = stream_clone.lock().await;
                            match stream_guard.write_all(&encoded).await {
                                Ok(_) => {
                                    if let Err(e) = stream_guard.flush().await {
                                        eprintln!("Failed to flush stream: {}", e);
                                    } 
                                },
                                Err(e) => {
                                    eprintln!("Failed to send message: {}", e);
                                }
                            }
                            drop(stream_guard);
                        });
                    }
                },
                _ => {
                    // 不在聊天视图中，无法发送消息
                    // 创建一个临时消息向量来显示系统消息
                    let system_target = "system".to_string();
                    if !self.messages.contains_key(&system_target) {
                        self.messages.insert(system_target.clone(), vec![]);
                    }
                    if let Some(messages) = self.messages.get_mut(&system_target) {
                        let msg = MessageItem::system("Cannot send message: not in chat view");
                        messages.push(msg);
                    }
                }
            };
            false
        };

        self.input.clear();
        self.mode = Mode::Normal;
        should_exit
    }

    fn handle_command(&mut self) -> bool {
        let mut should_exit = false;
        let cmd = self.input.split_whitespace().next().unwrap_or("");
        let target = match &self.current_view {
            View::Chat { target } => target.clone(),
            _ => "system".to_string(),
        };

        match cmd {
            "/help" => {
                if let Some(messages) = self.messages.get_mut(&target) {
                    messages.push(MessageItem::system("Available commands:"));
                    messages.push(MessageItem::system("/help - Show this help"));
                    messages.push(MessageItem::system("/quit or /exit - Exit the application"));
                    messages.push(MessageItem::system("/clear - Clear chat history"));
                    messages.push(MessageItem::system("/status <status> - Change your status"));
                }
            }
            "/quit" | "/exit" => {
                should_exit = true;
            }
            "/clear" => {
                match &self.current_view {
                    View::Chat { target } => {
                        if let Some(messages) = self.messages.get_mut(target) {
                            messages.clear();
                        }
                    }
                    _ => {
                        if let Some(messages) = self.messages.get_mut(&target) {
                            messages.push(MessageItem::system("Use /clear in chat view"));
                        }
                    }
                }
            }
            "/status" => {
                let parts: Vec<&str> = self.input.split_whitespace().collect();
                if parts.len() >= 2 {
                    let status_str = parts[1];
                    if let Some(messages) = self.messages.get_mut(&target) {
                        messages.push(MessageItem::system(&format!("Status changed to: {}", status_str)));
                    }
                    // TODO: 实际更改状态
                } else {
                    if let Some(messages) = self.messages.get_mut(&target) {
                        messages.push(MessageItem::system("Usage: /status <status>"));
                    }
                }
            }
            _ => {
                if let Some(messages) = self.messages.get_mut(&target) {
                    messages.push(MessageItem::system(&format!("Unknown command: {}", cmd)));
                    messages.push(MessageItem::system("Type /help for available commands"));
                }
            }
        }
        self.input.clear();
        self.mode = Mode::Normal;
        should_exit
    }
}