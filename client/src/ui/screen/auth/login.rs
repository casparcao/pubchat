use core::{auth::Token};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Clear},
};

#[derive(Debug, Clone)]
pub struct LoginScreen {
    pub username: String,
    pub password: String,
    pub focus: LoginFocus,
    pub error_message: Option<String>,
    pub is_logging_in: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoginFocus {
    Username,
    Password,
}

pub enum LoginResult {
    Success(Token), // 登录成功，返回token
    Continue,
    Exit,
}

impl Default for LoginScreen {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            focus: LoginFocus::Username,
            error_message: None,
            is_logging_in: false,
        }
    }
}

impl LoginScreen {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn render(&mut self, frame: &mut Frame) {
        let size = frame.area();
        
        // 创建登录框区域
        let popup_area = Rect {
            x: size.width.saturating_sub(50) / 2,
            y: size.height.saturating_sub(12) / 2,
            width: std::cmp::min(50, size.width),
            height: std::cmp::min(12, size.height),
        };
        
        // 清除背景
        frame.render_widget(Clear, popup_area);
        
        // 创建登录表单
        let block = Block::default()
            .title("Login")
            .borders(Borders::ALL);
            
        let inner_area = block.inner(popup_area);
        frame.render_widget(block, popup_area);
        
        // 布局表单元素
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Username标签
                Constraint::Length(3), // Username输入框
                Constraint::Length(1), // Password标签
                Constraint::Length(3), // Password输入框
                Constraint::Length(2), // 错误消息或状态消息
            ])
            .split(inner_area);
        
        // 显示用户名标签和输入框
        let username_label = Paragraph::new("Username:")
            .style(Style::default().fg(Color::White));
        frame.render_widget(username_label, chunks[0]);
        
        let username_style = if matches!(self.focus, LoginFocus::Username) {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        
        let username_input = Paragraph::new(self.username.as_str())
            .style(username_style)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(username_input, chunks[1]);
        
        // 显示密码标签和输入框
        let password_label = Paragraph::new("Password:")
            .style(Style::default().fg(Color::White));
        frame.render_widget(password_label, chunks[2]);
        
        // 隐藏密码字符
        let hidden_password: String = self.password.chars().map(|_| '*').collect();
        
        let password_style = if matches!(self.focus, LoginFocus::Password) {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        
        let password_input = Paragraph::new(hidden_password)
            .style(password_style)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(password_input, chunks[3]);
        
        // 显示错误消息或状态消息
        let message_paragraph = if self.is_logging_in {
            Paragraph::new("Logging in...")
                .style(Style::default().fg(Color::Blue))
        } else if let Some(error) = &self.error_message {
            Paragraph::new(error.as_str())
                .style(Style::default().fg(Color::Red))
        } else {
            Paragraph::new("")
        };
        
        frame.render_widget(message_paragraph, chunks[4]);
        
        // 设置光标位置
        if !self.is_logging_in {
            if matches!(self.focus, LoginFocus::Username) {
                frame.set_cursor_position(
                    (chunks[1].x + self.username.len() as u16 + 1,
                    chunks[1].y + 1,)
                );
            } else if matches!(self.focus, LoginFocus::Password) {
                frame.set_cursor_position(
                    (chunks[3].x + self.password.len() as u16 + 1,
                    chunks[3].y + 1,)
                );
            }
        }
    }
    
}
