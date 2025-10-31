use crate::ui::models::{App, Mode, Session, View, Me};
use crate::ui::renderers::contact::ContactListComponent;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::tcp::OwnedWriteHalf;
use core::proto::message::Chat;

impl App {
    pub fn new(token: String, me: Me) -> Self {
        Self {
            input: String::new(),
            contact: ContactListComponent::new(&token),
            sessions: vec![],
            current_view: View::Chat {
                session: Session {id:0, name:"session1".to_string()},
            },
            mode: Mode::Normal,
            scroll_offset: 0,
            selected_friend: None, // 初始化选中的好友
            me: me,
            chat_maximized: false,
            token: token,
            stream: None,
        }
    }
    
    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }
    
    pub fn set_stream(&mut self, stream: Arc<Mutex<OwnedWriteHalf>>) {
        self.stream = Some(stream);
    }
    
    // 添加接收消息的方法
    pub fn add_received_message(&mut self, chat_req: Chat) {
        let target = chat_req.nickname.clone();
        
        
    }
}
