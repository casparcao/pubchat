
use core::request::Page;

use crate::{cache, ui::models::{Message, Mode, Session}};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

#[derive(Debug, Clone)]
pub struct ChatComponent {
    // 聊天窗口布局
    pub session: Option<Session>,
    pub messages: Vec<Message>,
    pub mode: Mode,
    pub input: String,
    pub token: String,
}

impl ChatComponent {
    pub fn new(token: &str) -> Self {
        Self { session: None, messages: vec![], mode: Mode::Normal, input: String::new(), token: token.to_string() }
    }

    pub fn change_session(&mut self, session: Option<Session>) {
        self.session = session;
        if let Some(session) = &self.session {
            // 获取当前聊天目标的消息
            match cache::message_cache().get_messages(session.id, &self.token, Page::default()){
                Ok(messages) => {
                    self.messages = messages
                    .iter()
                    .map(|m| Message::new(m.uname.clone(), m.content.clone(), false))
                    .collect();
                },
                Err(err) => {
                    log::error!("Error fetching messages: {}", err);
                }
            }
        }
    }
    
    pub fn change_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn input(&mut self, c: char) {
        if self.mode == Mode::Normal {
            return;
        }
        self.input.push(c);
    }

    pub fn delete(&mut self) {
        if self.mode == Mode::Normal {
            return;
        }
        if !self.input.is_empty() {
            self.input.pop();
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
            let style = if m.system {
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
                "[M] (Press 'm' to switch maximized)" )
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