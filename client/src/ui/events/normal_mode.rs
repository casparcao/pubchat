use crate::ui::models::{App, Session, View};
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
                // 在好友列表视图中向上导航
                match self.view {
                    View::FriendsList => {
                        // 在好友列表视图中向上导航
                        self.contact.move_up();
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
                // 在好友列表视图中向下导航
                match self.view {
                    View::FriendsList => {
                        // 在好友列表视图中向下导航
                        self.contact.move_down();
                    }
                    _ => {
                        // 在聊天视图中，j键用于滚动消息
                        self.scroll_offset += 1;
                    }
                }
            }
            KeyCode::Char('f') => {
                // 切换到好友列表视图
                self.view = View::FriendsList;
            }
            KeyCode::Char('m') => {
                // 切换聊天窗口最大化状态
                if matches!(self.view, View::Chat { .. }) {
                    self.chat_maximized = !self.chat_maximized;
                }
            }
            KeyCode::Enter => {
                // 在好友列表视图中按Enter选择
                match &self.view {
                    View::FriendsList => {
                        if let Ok(session ) = self.contact.create_session(&self.token, &self.me){
                            self.view = View::Chat { session: session };
                        }else{
                            log::warn!("Failed to create session when entering chat view");
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Tab => {
                // 在不同视图间切换
                self.view = match self.view {
                    View::Chat { .. } => View::FriendsList,
                    View::FriendsList => {
                        if let Ok(session ) = self.contact.create_session(&self.token, &self.me){
                            View::Chat { session: session}
                        }else{
                            log::warn!("Failed to create session when switching to chat view");
                            View::FriendsList
                        }
                    },
                };
            }
            _ => {}
        }
        
        should_exit
    }
}