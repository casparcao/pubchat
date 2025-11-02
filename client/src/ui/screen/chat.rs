use core::request::Page;

use crate::{cache, ui::{component::{chat::ChatComponent, session::SessionListComponent}, models::{App, MessageItem, Mode, Session}}};
use ratatui::{
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct ChatScreen {
    pub sessions: Vec<Session>,
}

impl ChatScreen {
    pub fn new(token: &str) -> Self {
        // Split the TCP stream into read and write halves
        match cache::session_cache().get_sessions(token, Page::default()){
            Ok(sessions) => {
                Self {sessions: sessions
                    .into_iter()
                    .map(|s| crate::ui::models::Session::from_session_response(s))
                    .collect()}
            },
            Err(e) => {
                log::error!("Failed to get sessions: {:?}", e);
                Self {sessions: vec![]}
            },
        }
    }
}

impl ChatScreen {
    pub fn render_maximized_chat_layout(&self, frame: &mut Frame, area: Rect, session: Session) {
        // 最大化聊天窗口布局：只显示聊天窗口和输入框
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),         // 消息区域（占据大部分空间）
                Constraint::Length(5),      // 输入框区域
            ])
            .split(area);

        // 获取当前聊天目标的消息
        let messages = vec![];
        
        // 创建聊天组件并渲染
        let chat_component = ChatComponent::new(
            Some(session),
            messages,
            // self.chat_maximized,
            // self.mode.clone(),
            // self.input.clone(),
            true,
            Mode::Normal,
            "".to_string(),
        );
        chat_component.render(frame, chunks[0]);
        
    }

    pub fn render_main_layout(&self, frame: &mut Frame, area: Rect, session: Session) {
        // 两栏布局：会话列表(1/3) + 聊天窗口(2/3)
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // 会话列表
                Constraint::Percentage(70), // 聊天窗口
            ])
            .split(area);

        let session_list_component = SessionListComponent::new(self.sessions.clone());
        // 左侧会话列表
        session_list_component.render(frame, chunks[0]);

        // 右侧聊天区域
        let chat_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),         // 消息区域
                Constraint::Length(5),      // 增大输入框区域
            ])
            .split(chunks[1]);

        // 获取当前聊天目标的消息
        match cache::message_cache().get_messages(session.id, "", Page::default()){
            Ok(messages) => {
                // 创建聊天组件并渲染
                let chat_component = ChatComponent::new(
                    Some(session),
                    messages.iter().map(|m| 
                        MessageItem::new(m.sender.to_string(), m.content.clone(), true)).collect(),
                    // self.chat_maximized,
                    // self.mode.clone(),
                    // self.input.clone(),
                    true,
                    Mode::Normal,
                    "".to_string(),
                );
                chat_component.render(frame, chat_chunks[0]);
            },
            Err(err) => {
                log::error!("Error fetching messages: {}", err);
            }
        }
    }


}
