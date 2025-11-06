pub mod send;
pub mod chat;
pub mod contact;

use crate::{cache, ui::{models::{App, Session, View}, screen::chat::Focus}};
use crossterm::event::{KeyCode, KeyEvent};

pub enum EventResult {
    //创建session
    CreateSession(Session),
    //导航到联系人
    Nav2Contact,
    //发送消息
    SendMessage(),
    //切换焦点到session列表
    Nav2SessionList,
    //什么都不做
    None,
}

impl App {
    pub fn handle(&mut self, key: KeyEvent) {
        let result : EventResult = match self.view {
            View::Contact => self.contact.handle(key),
            View::Chat => self.chat.handle(key)
        };
        match result {
            EventResult::CreateSession(session) => {
                // 切换到聊天页面
                self.change_view(View::Chat);
                let session = cache::session_cache().get_session(&self.token, session.id);
                log::info!("Create session event: {:?}", session);
                let session = session
                    .map(|s| Session::from_session_detail_response(s))
                    .ok();
                self.chat.chat.change_session(session);
                self.chat.focus = Focus::Chat;
            },
            EventResult::Nav2Contact => {
                // 切换到联系人页面
                self.change_view(View::Contact);
            },
            EventResult::SendMessage() => {
                // 发送消息
                self.chat.chat.send_message(&self.me, &self.stream);
            },
            EventResult::None => {}
            _ => {}
        }
        match key.code {
            KeyCode::Tab => {
                // 在不同视图间切换
                self.view = match self.view {
                    View::Chat => View::Contact,
                    View::Contact => View::Chat,
                };
            }
            _ => {}
        }
    }
}