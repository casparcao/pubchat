use crate::{ui::{component::{chat::ChatComponent, session::SessionListComponent}}};
use ratatui::{
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct ChatScreen {
    pub sessions: SessionListComponent,
    pub chat: ChatComponent,
    pub maximized: bool,
}

impl ChatScreen {
    pub fn new(token: &str) -> Self {
        // Split the TCP stream into read and write halves
        Self {
            sessions: SessionListComponent::new(token),
            chat: ChatComponent::new(token),
            maximized: false,
        }
        
    }
}

impl ChatScreen {

    pub fn render(&self, frame: &mut Frame, area: Rect){
        if self.maximized {
            self.render_maximized_chat_layout(frame, area)
        } else {
            self.render_main_layout(frame, area)
        }
    }

    fn render_maximized_chat_layout(&self, frame: &mut Frame, area: Rect) {
        // 最大化聊天窗口布局：只显示聊天窗口和输入框
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),         // 消息区域（占据大部分空间）
                Constraint::Length(5),      // 输入框区域
            ])
            .split(area);

        // 获取当前聊天目标的消息
        self.chat.render(frame, chunks[0]);
        
    }

    fn render_main_layout(&self, frame: &mut Frame, area: Rect) {
        // 两栏布局：会话列表(1/3) + 聊天窗口(2/3)
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // 会话列表
                Constraint::Percentage(70), // 聊天窗口
            ])
            .split(area);
        // 左侧会话列表
        self.sessions.render(frame, chunks[0]);

        // 右侧聊天区域
        let chat_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),         // 消息区域
                Constraint::Length(5),      // 增大输入框区域
            ])
            .split(chunks[1]);
        self.chat.render(frame, chat_chunks[0]);
    }

}
