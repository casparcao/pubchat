use crossterm::event::{KeyCode, KeyEvent};
use core::{api::{client::session::calc_session_id, types::{contact::ContactResponse, session::CreateSessionRequest}}, response::ApiErr};

use crate::{cache, ui::models::{Contact, Session, Status}};

use anyhow::Result;

use crate::ui::{events::EventResult, screen::contact::ContactListScreen};

impl ContactListScreen {
    pub fn handle(&mut self, key: KeyEvent) -> EventResult  {
        match key.code {
            KeyCode::Char('k') => {
                self.move_up();
            }
            KeyCode::Char('j') => {
                self.move_down();
            }
            KeyCode::Enter | KeyCode::Char('l') => {
                if let Ok(session ) = self.create_session(){
                    return EventResult::CreateSession(session);
                }
            }
            _ => {}
        }
        EventResult::None
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

    pub fn create_session(&self) -> Result<Session>{
        if let Some(selected) = &self.selected {
             // 计算会话ID
            let session_id = calc_session_id(self.me.id as i64, selected.id) as i64;
            let session_name = format!("{} and {}", self.me.name, selected.name);
            // 构造创建会话请求
            let request = CreateSessionRequest {
                id: session_id,
                name: session_name.clone(),
                members: vec![
                    ContactResponse {
                        id: self.me.id as i64,
                        name: self.me.name.clone(),
                        avatar: None,
                    },
                    ContactResponse {
                        id: selected.id,
                        name: selected.name.clone(),
                        avatar: None,
                    }
                ],
            };
            match cache::session_cache().add_session(&self.token, request){
            // 创建会话
                Ok(_) => {
                    // 创建或更新本地会话列表
                    Ok(Session { id: session_id, 
                        name: session_name, 
                        members: vec![Contact{id: selected.id, 
                            name: selected.name.clone(), 
                            avatar: None,
                            status: Status::Online}]
                     })
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