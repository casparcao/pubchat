
use crate::{ui::{models::{MessageItem, Mode, Session}}};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
pub struct ChatComponent {
    // 聊天窗口布局
    pub session: Option<Session>,
    pub messages: Vec<MessageItem>,
    pub chat_maximized: bool,
    pub mode: Mode,
    pub input: String,
}

impl ChatComponent {
    pub fn new(session: Option<Session>, messages: Vec<MessageItem>, chat_maximized: bool, mode: Mode, input: String) -> Self {
        Self {
            session,
            messages,
            chat_maximized,
            mode,
            input,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),         // 消息区域
                Constraint::Length(3),      // 输入框区域
            ])
            .split(area);

        // 渲染消息区域
        self.render_messages(frame, chunks[0]);
        
        // 渲染输入框
        self.render_input(frame, chunks[1]);
    }

    fn render_messages(&self, frame: &mut Frame, area: Rect) {
        let list_items: Vec<ListItem> = self.messages.iter().map(|m| {
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

        let title = if let Some(session) = &self.session {
            format!("Chat with {} {}", 
                session.name,
                if self.chat_maximized { 
                    "[M] (Press 'm' to restore)" 
                } else { 
                    "[M] (Press 'm' to maximize)" 
                })
        } else {
            "Messages".to_string()
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