use crate::ui::models::{App, View, Mode, Status};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

impl App {
    pub fn render_contacts_layout(&self, frame: &mut Frame, area: Rect) {
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
    
    pub fn render_sessions_list(&self, frame: &mut Frame, area: Rect) {
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
}