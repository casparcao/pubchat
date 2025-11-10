use crate::{cache, ui::models::{Contact, Me, Status}};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

#[derive(Debug, Clone)]
pub struct ContactListScreen {
    pub contacts: Vec<Contact>,
    pub selected: Option<Contact>,
    //选中联系人在列表中的索引
    pub index: usize,
    pub me: Me,
    pub token: String,
}

impl ContactListScreen {
    pub fn new(token: &str, me: Me) -> Self {
        match cache::contact_cache().get_contacts(token){
            Ok(friends) => {
                let contacts = friends
                    .into_iter()
                    .map(|friend| Contact::from(friend))
                    .collect();
                log::info!("Contacts loaded: {:?}", contacts);
                Self {contacts, selected: None, index: 0, me, token: token.to_string()}
            },
            Err(e) => {
                log::error!("Failed to get contacts: {:?}", e);
                Self {contacts: vec![], selected: None, index: 0, me, token: token.to_string()}
            },
        }
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
            
        let info_text = if let Some(selected) = &self.selected {
            format!("Name: {}\nStatus: {}\n\nPress Enter to start chat", 
                selected.name,
                match selected.status {
                    Status::Online => "Online",
                    Status::Offline => "Offline",
                })
        }else{
            "Select a friend to view their information".to_string()
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
                    // Status::Busy => "🔴",
                    // Status::Away => "🟡",
                };
                let content = format!("{} {}", status_char, friend.name);
                let mut item = ListItem::new(content);
                if self.index == i {
                    item = item.style(Style::default().bg(Color::Blue));
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