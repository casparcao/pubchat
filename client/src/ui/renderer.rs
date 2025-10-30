use crate::ui::models::{App, View, Mode, Status};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

impl App {
    pub fn render(&self, frame: &mut Frame) {
        let size = frame.area();
        match &self.current_view {
            View::Chat { target } => {
                if self.chat_maximized {
                    self.render_maximized_chat_layout(frame, size, target)
                } else {
                    self.render_main_layout(frame, size, target)
                }
            },
            View::Contacts => self.render_contacts_layout(frame, size),
            View::FriendsList => self.render_friends_list_layout(frame, size), // 添加好友列表视图
        }
    }

    fn render_maximized_chat_layout(&self, frame: &mut Frame, area: Rect, target: &str) {
        // 最大化聊天窗口布局：只显示聊天窗口和输入框
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),         // 消息区域（占据大部分空间）
                Constraint::Length(5),      // 输入框区域
            ])
            .split(area);

        self.render_messages(frame, chunks[0]);
        self.render_input(frame, chunks[1]);
    }

    fn render_main_layout(&self, frame: &mut Frame, area: Rect, target: &str) {
        // 两栏布局：会话列表(1/3) + 聊天窗口(2/3)
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // 会话列表
                Constraint::Percentage(70), // 聊天窗口
            ])
            .split(area);

        // 左侧会话列表
        self.render_sessions_list(frame, chunks[0]);

        // 右侧聊天区域
        let chat_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),         // 消息区域
                Constraint::Length(5),      // 增大输入框区域
            ])
            .split(chunks[1]);

        self.render_messages(frame, chat_chunks[0]);
        self.render_input(frame, chat_chunks[1]);
    }

    fn render_contacts_layout(&self, frame: &mut Frame, area: Rect) {
        // 主要显示联系人列表，带一些聊天区域
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // 会话列表
                Constraint::Percentage(70), // 信息区域
            ])
            .split(area);

        self.render_sessions_list(frame, chunks[0]);
        
        // 右侧显示会话详细信息或帮助
        let info_block = Block::default()
            .title("Session Info")
            .borders(Borders::ALL);
            
        let info_text = if let Some(index) = self.selected_contact {
            if index < self.sessions.len() {
                let session = &self.sessions[index];
                format!("Name: {}\n\nPress Enter to chat", 
                    session.name)
            } else {
                "Select a session".to_string()
            }
        } else {
            "Select a session".to_string()
        };
        
        let info = Paragraph::new(info_text)
            .block(info_block);
            
        frame.render_widget(info, chunks[1]);
    }

    // 添加好友列表布局渲染
    fn render_friends_list_layout(&self, frame: &mut Frame, area: Rect) {
        // 主要显示好友列表
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // 好友列表
                Constraint::Percentage(70), // 信息区域
            ])
            .split(area);

        self.render_friends_list(frame, chunks[0]);
        
        // 右侧显示好友详细信息或帮助
        let info_block = Block::default()
            .title("Friend Info")
            .borders(Borders::ALL);
            
        let info_text = if let Some(index) = self.selected_friend {
            if index < self.contacts.len() {
                let friend = &self.contacts[index];
                format!("Name: {}\nStatus: {}\n\nPress Enter to start chat", 
                    friend.name,
                    match friend.status {
                        Status::Online => "Online",
                        Status::Offline => "Offline",
                        Status::Busy => "Busy",
                        Status::Away => "Away",
                    })
            } else {
                "Select a friend".to_string()
            }
        } else {
            "Select a friend".to_string()
        };
        
        let info = Paragraph::new(info_text)
            .block(info_block);
            
        frame.render_widget(info, chunks[1]);
    }

    fn render_friends_list(&self, frame: &mut Frame, area: Rect) {
        let friends: Vec<ListItem> = self.contacts
            .iter()
            .enumerate()
            .map(|(i, friend)| {
                let status_char = match friend.status {
                    Status::Online => "🟢",
                    Status::Offline => "🔴",
                    Status::Busy => "🔴",
                    Status::Away => "🟡",
                };
                let content = format!("{} {}", status_char, friend.name);
                let mut item = ListItem::new(content);
                if let Some(selected) = self.selected_friend {
                    if selected == i {
                        item = item.style(Style::default().bg(Color::Blue));
                    }
                }
                item
            })
            .collect();

        let title = "Friends (↑/↓ to select, Enter to chat)";

        let friends_list = List::new(friends)
            .block(Block::default().title(title).borders(Borders::ALL));

        frame.render_widget(friends_list, area);
    }

    fn render_sessions_list(&self, frame: &mut Frame, area: Rect) {
        let sessions: Vec<ListItem> = self.sessions
            .iter()
            .enumerate()
            .map(|(i, session)| {
                let content = format!("💬 {}", session.name);
                let mut item = ListItem::new(content);
                if let Some(selected) = self.selected_contact {
                    if selected == i {
                        item = item.style(Style::default().bg(Color::Blue));
                    }
                }
                item
            })
            .collect();

        let title = match self.current_view {
            View::Contacts => "Sessions (↑/↓ to select)",
            _ => "Sessions"
        };

        let sessions_list = List::new(sessions)
            .block(Block::default().title(title).borders(Borders::ALL));

        frame.render_widget(sessions_list, area);
    }

    fn render_messages(&self, frame: &mut Frame, area: Rect) {
        // 获取当前聊天目标的消息
        let messages = match &self.current_view {
            View::Chat { target } => {
                self.messages.get(target).cloned().unwrap_or_default()
            },
            _ => vec![]
        };

        let list_items: Vec<ListItem> = messages.iter().map(|m| {
            let style = if m.is_user {
                Style::default().fg(Color::Blue)
            } else if m.sender == "SYSTEM" {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Green)
            };

            let content = format!("[{}] <{}> {}", m.timestamp, m.sender, m.content);
            ListItem::new(content).style(style)
        }).collect();

        // 获取当前聊天目标
        let title = match &self.current_view {
            View::Chat { target } => {
                // 检查目标是会话还是群组
                if self.sessions.iter().any(|s| s.name == *target) {
                    format!("Chat with {} {}", target, 
                        if self.chat_maximized { "[M] (Press 'm' to restore)" } else { "[M] (Press 'm' to maximize)" })
                } else {
                    format!("Chat with {} {}",
                        target,
                        if self.chat_maximized { "[M] (Press 'm' to restore)" } else { "[M] (Press 'm' to maximize)" })
                }
            },
            _ => "Messages".to_string(),
        };

        let messages_list = List::new(list_items)
            .block(Block::default().title(title).borders(Borders::ALL))
            .scroll_padding(1);

        frame.render_widget(messages_list, area);
    }

    fn render_input(&self, frame: &mut Frame, area: Rect) {
        let (text, style) = match self.mode {
            Mode::Normal => ("Normal Mode (i to insert)", Style::default().fg(Color::Yellow)),
            Mode::Insert => ("INSERT (Esc to normal)", Style::default().fg(Color::Green)),
        };

        // 创建一个内部区域，保留底部一行用于模式提示
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),        // 输入区域
                Constraint::Length(1),     // 模式提示
            ])
            .split(area);

        let input = Paragraph::new(self.input.as_str())
            .block(Block::default().borders(Borders::ALL));

        let mode = Paragraph::new(text)
            .style(style)
            .alignment(Alignment::Left);

        frame.render_widget(input, chunks[0]);
        frame.render_widget(mode, chunks[1]);
        
        // 只在插入模式下设置光标位置
        if let Mode::Insert = self.mode {
            frame.set_cursor_position(
                (chunks[0].x + self.input.len() as u16 + 1,
                 chunks[0].y + 1)
            );
        }
    }
}