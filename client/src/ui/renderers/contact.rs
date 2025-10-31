use crate::ui::models::{Contact, Status};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

#[derive(Debug, Clone)]
pub struct ContactListComponent {
    pub contacts: Vec<Contact>,
    pub selected: Option<usize>,
}

impl ContactListComponent {
    pub fn new(contacts: Vec<Contact>, selected: Option<usize>) -> Self {
        Self { contacts, selected }
    }

    pub fn render_friends_list_layout(&self, frame: &mut Frame, area: Rect) {
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
            
        let info_text = if let Some(index) = self.selected {
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

    pub fn render_friends_list(&self, frame: &mut Frame, area: Rect) {
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
                if let Some(selected) = self.selected {
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
}