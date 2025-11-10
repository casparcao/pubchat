use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Clear},
};

#[derive(Debug, Clone)]
pub struct RegisterScreen {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    pub focus: RegisterFocus,
    pub error_message: Option<String>,
    pub is_registering: bool,
    pub success_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RegisterFocus {
    Username,
    Password,
    ConfirmPassword,
}

pub enum RegisterResult {
    Success, // 注册成功
    SwitchToLogin, // 切换到登录界面
    Continue,
    Exit,
}

impl Default for RegisterScreen {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            focus: RegisterFocus::Username,
            error_message: None,
            is_registering: false,
            success_message: None,
        }
    }
}

impl RegisterScreen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let size = frame.area();
        
        // 创建注册框区域，增加高度以提供更多空间
        let popup_area = Rect {
            x: size.width.saturating_sub(50) / 2,
            y: size.height.saturating_sub(18) / 2, // 增加窗口高度从15到18
            width: std::cmp::min(50, size.width),
            height: std::cmp::min(18, size.height), // 增加窗口高度从15到18
        };
        
        // 清除背景
        frame.render_widget(Clear, popup_area);
        
        // 创建注册表单
        let block = Block::default()
            .title("Register (Ctrl+L to switch to Login)")
            .borders(Borders::ALL);
            
        let inner_area = block.inner(popup_area);
        frame.render_widget(block, popup_area);
        
        // 布局表单元素，为每个输入框提供更多空间
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Username标签
                Constraint::Length(3), // Username输入框
                Constraint::Length(1), // Password标签
                Constraint::Length(3), // Password输入框
                Constraint::Length(1), // Confirm Password标签
                Constraint::Length(3), // Confirm Password输入框
                Constraint::Length(3), // 消息区域，增加空间
            ])
            .split(inner_area);
        
        // 显示用户名标签和输入框
        let username_label = Paragraph::new("Username (letters/numbers only):")
            .style(Style::default().fg(Color::White));
        frame.render_widget(username_label, chunks[0]);
        
        let username_style = if matches!(self.focus, RegisterFocus::Username) {
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
        
        let password_style = if matches!(self.focus, RegisterFocus::Password) {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        
        let password_input = Paragraph::new(hidden_password)
            .style(password_style)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(password_input, chunks[3]);
        
        // 显示确认密码标签和输入框
        let confirm_password_label = Paragraph::new("Confirm Password:")
            .style(Style::default().fg(Color::White));
        frame.render_widget(confirm_password_label, chunks[4]);
        
        // 隐藏确认密码字符
        let hidden_confirm_password: String = self.confirm_password.chars().map(|_| '*').collect();
        
        let confirm_password_style = if matches!(self.focus, RegisterFocus::ConfirmPassword) {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        
        let confirm_password_input = Paragraph::new(hidden_confirm_password)
            .style(confirm_password_style)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(confirm_password_input, chunks[5]);
        
        // 显示错误消息或状态消息
        let message_paragraph = if self.is_registering {
            Paragraph::new("Registering...")
                .style(Style::default().fg(Color::Blue))
        } else if let Some(success) = &self.success_message {
            Paragraph::new(success.as_str())
                .style(Style::default().fg(Color::Green))
        } else if let Some(error) = &self.error_message {
            Paragraph::new(error.as_str())
                .style(Style::default().fg(Color::Red))
        } else {
            Paragraph::new("")
        };
        
        frame.render_widget(message_paragraph, chunks[6]);
        
        // 设置光标位置
        if !self.is_registering {
            match self.focus {
                RegisterFocus::Username => {
                    frame.set_cursor_position(
                        (chunks[1].x + self.username.len() as u16 + 1,
                        chunks[1].y + 1,)
                    );
                }
                RegisterFocus::Password => {
                    frame.set_cursor_position(
                        (chunks[3].x + self.password.len() as u16 + 1,
                        chunks[3].y + 1,)
                    );
                }
                RegisterFocus::ConfirmPassword => {
                    frame.set_cursor_position(
                        (chunks[5].x + self.confirm_password.len() as u16 + 1,
                        chunks[5].y + 1,)
                    );
                }
            }
        }
    }
}