use core::response::ApiErr;

use crate::{cache, remote::session::{CreateSessionRequest, CreateSessionUserRequest, calc_session_id, create_session}, ui::models::{Contact, Me, Session, Status}};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ContactListScreen {
    pub contacts: Vec<Contact>,
    pub selected: Option<Contact>,
    //选中联系人在列表中的索引
    pub index: usize,
}

impl ContactListScreen {
    pub fn new(token: &str) -> Self {
        match cache::contact_cache().get_contacts(token){
            Ok(friends) => {
                let contacts = friends
                    .into_iter()
                    .map(|friend| Contact::from_contact_response(friend))
                    .collect();
                log::info!("Contacts loaded: {:?}", contacts);
                Self {contacts, selected: None, index: 0}
            },
            Err(e) => {
                log::error!("Failed to get contacts: {:?}", e);
                Self {contacts: vec![], selected: None, index: 0}
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
                    Status::Busy => "Busy",
                    Status::Away => "Away",
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
                    Status::Busy => "🔴",
                    Status::Away => "🟡",
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

    pub fn move_up(&mut self){
        if self.contacts.is_empty() {
            return;
        }
        if self.index > 0 {
            self.index -= 1;
        }
        self.selected = Some(self.contacts[self.index].clone());
    }

    pub fn move_down(&mut self){
        if self.contacts.is_empty() {
            return;
        }
        if self.index < self.contacts.len() - 1 {
            self.index += 1;
        }
        self.selected = Some(self.contacts[self.index].clone());
    }

    pub fn create_session(&self, token: &str, me: &Me) -> Result<Session>{
        if let Some(selected) = &self.selected {
             // 计算会话ID
            let session_id = calc_session_id(me.id as i64, selected.id) as i64;
            // 构造创建会话请求
            let request = CreateSessionRequest {
                id: session_id,
                name: format!("{} and {}", me.name, selected.name),
                members: vec![
                    CreateSessionUserRequest {
                        id: me.id as i64,
                        name: me.name.clone(),
                    },
                    CreateSessionUserRequest {
                        id: selected.id,
                        name: selected.name.clone(),
                    }
                ],
            };
            match cache::session_cache().add_session(token, request){
            // 创建会话
                Ok(session_response) => {
                    // 创建或更新本地会话列表
                    let app_session = Session::from_session_response(session_response);
                    Ok(app_session)
                }
                Err(e) => {
                    Err(ApiErr::Error(format!("Failed to create session: {}", e)).into())
                }
            }
        }else{
            log::warn!("No contact selected");
            Err(ApiErr::Error("No contact selected".to_string()).into())
        }
    }

}