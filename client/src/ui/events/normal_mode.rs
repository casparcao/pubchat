use crate::ui::models::{App, View};
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};

impl App {
    /// 处理 Normal 模式下的按键事件
    pub fn handle_normal_mode_key_event(&mut self, key: KeyEvent) -> bool {
        let mut should_exit = false;
        
        match key.code {
            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                // Ctrl+Q 退出
                should_exit = true;
            }
            KeyCode::Char('i') => {
                self.mode = crate::ui::models::Mode::Insert;
            }
            KeyCode::Char('k') => {
                // 在联系人视图中向上导航
                match self.current_view {
                    View::Contacts => {
                        if !self.sessions.is_empty() {
                            if let Some(selected) = self.selected_contact {
                                self.selected_contact = Some(selected.saturating_sub(1));
                            } else {
                                self.selected_contact = Some(0);
                            }
                        }
                    }
                    View::FriendsList => {
                        // 在好友列表视图中向上导航
                        if !self.contacts.is_empty() {
                            if let Some(selected) = self.selected_friend {
                                self.selected_friend = Some(selected.saturating_sub(1));
                            } else {
                                self.selected_friend = Some(0);
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
            KeyCode::Char('j') => {
                // 在联系人视图中向下导航
                match self.current_view {
                    View::Contacts => {
                        if !self.sessions.is_empty() {
                            if let Some(selected) = self.selected_contact {
                                self.selected_contact = Some((selected + 1).min(self.sessions.len() - 1));
                            } else {
                                self.selected_contact = Some(0);
                            }
                        }
                    }
                    View::FriendsList => {
                        // 在好友列表视图中向下导航
                        if !self.contacts.is_empty() {
                            if let Some(selected) = self.selected_friend {
                                self.selected_friend = Some((selected + 1).min(self.contacts.len() - 1));
                            } else {
                                self.selected_friend = Some(0);
                            }
                        }
                    }
                    _ => {
                        // 在聊天视图中，j键用于滚动消息
                        self.scroll_offset += 1;
                    }
                }
            }
            KeyCode::Char('h') => {
                // 切换到联系人视图
                self.current_view = View::Contacts;
            }
            KeyCode::Char('f') => {
                // 切换到好友列表视图
                self.current_view = View::FriendsList;
            }
            KeyCode::Char('m') => {
                // 切换聊天窗口最大化状态
                if matches!(self.current_view, View::Chat { .. }) {
                    self.chat_maximized = !self.chat_maximized;
                }
            }
            KeyCode::Enter => {
                // 在联系人视图中按Enter选择
                match &self.current_view {
                    View::Contacts => {
                        if let Some(index) = self.selected_contact {
                            if index < self.sessions.len() {
                                let target = self.sessions[index].name.clone();
                                self.current_view = View::Chat { target: target.clone() };
                                
                                // 确保目标有消息列表
                                if !self.messages.contains_key(&target) {
                                    self.messages.insert(target.clone(), vec![]);
                                }
                            }
                        }
                    }
                    View::FriendsList => {
                        // 在好友列表视图中按Enter选择
                        if let Some(index) = self.selected_friend {
                            if index < self.contacts.len() {
                                let friend = &self.contacts[index];
                                let friend_name = friend.name.clone();
                                let friend_id = friend.id;
                                
                                // 创建一个任务来创建会话并加载消息
                                match self.create_or_get_session(friend_name.clone(), friend_id) {
                                    Ok(session) => {
                                        // 加载会话消息
                                        if let Err(e) = self.load_session_messages(session.id, friend_name.clone()){
                                            eprintln!("Failed to load session messages: {}", e);
                                        }
                                        // 将会话添加到本地列表（如果不存在）
                                        if !self.sessions.iter().any(|s| s.id == session.id) {
                                            self.sessions.push(session);
                                        }
                                        
                                        // 切换到聊天视图
                                        self.current_view = View::Chat { target: friend_name };
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to create session: {}", e);
                                        // 如果创建会话失败，仍然切换到聊天视图
                                        self.current_view = View::Chat { target: friend_name.clone() };
                                        
                                        // 确保目标有消息列表
                                        if !self.messages.contains_key(&friend_name) {
                                            self.messages.insert(friend_name.clone(), vec![]);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Tab => {
                // 在不同视图间切换
                self.current_view = match self.current_view {
                    View::Chat { .. } => View::Contacts,
                    View::Contacts => View::FriendsList,
                    View::FriendsList => View::Chat { target: "alice".to_string() },
                };
            }
            _ => {}
        }
        
        should_exit
    }
}