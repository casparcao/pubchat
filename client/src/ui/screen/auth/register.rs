use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Clear},
};
use crossterm::event::{KeyCode, KeyEvent};
use core::api::{client::login, types::auth::RegisterRequest};

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

    fn validate_username(&self) -> bool {
        // 用户名只能包含英文字符和数字
        !self.username.is_empty() 
            && self.username.chars().all(|c| c.is_ascii_alphanumeric())
            && self.username.len() >= 3
    }

    fn validate_password(&self) -> bool {
        // 密码至少6位
        !self.password.is_empty() && self.password.len() >= 6
    }

    fn validate_confirm_password(&self) -> bool {
        // 确认密码需要与密码一致
        !self.confirm_password.is_empty() && self.password == self.confirm_password
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> RegisterResult {
        // 如果正在注册中，不处理其他按键
        if self.is_registering {
            return RegisterResult::Continue;
        }

        match key.code {
            KeyCode::Enter => {
                // 检查所有字段是否填写并验证
                if self.username.is_empty() || self.password.is_empty() || self.confirm_password.is_empty() {
                    self.error_message = Some("All fields are required".to_string());
                    return RegisterResult::Continue;
                }

                // 验证用户名格式
                if !self.validate_username() {
                    self.error_message = Some("Username must be 3+ chars and contain only letters/numbers".to_string());
                    return RegisterResult::Continue;
                }

                // 验证密码长度
                if !self.validate_password() {
                    self.error_message = Some("Password must be at least 6 characters".to_string());
                    return RegisterResult::Continue;
                }

                // 验证确认密码
                if !self.validate_confirm_password() {
                    self.error_message = Some("Passwords do not match".to_string());
                    return RegisterResult::Continue;
                }

                // 开始注册过程
                self.is_registering = true;
                self.error_message = None;
                self.success_message = None;
                
                // 执行注册操作
                let request = RegisterRequest {
                    username: self.username.clone(),
                    password: self.password.clone(),
                    gender: "U".to_string(), // 默认性别
                    age: 0, // 默认年龄
                };
                
                let result = login::register(&request);
                match result {
                    Ok(_) => {
                        self.is_registering = false;
                        self.success_message = Some("Registration successful! Press Enter to login.".to_string());
                        RegisterResult::Success
                    }
                    Err(e) => {
                        self.is_registering = false;
                        self.error_message = Some(e.to_string());
                        RegisterResult::Continue
                    }
                }
            }
            KeyCode::Tab => {
                // 在输入框之间切换焦点
                self.focus = match self.focus {
                    RegisterFocus::Username => RegisterFocus::Password,
                    RegisterFocus::Password => RegisterFocus::ConfirmPassword,
                    RegisterFocus::ConfirmPassword => RegisterFocus::Username,
                };
                RegisterResult::Continue
            }
            KeyCode::Backspace => {
                match self.focus {
                    RegisterFocus::Username => {
                        self.username.pop();
                    }
                    RegisterFocus::Password => {
                        self.password.pop();
                    }
                    RegisterFocus::ConfirmPassword => {
                        self.confirm_password.pop();
                    }
                }
                RegisterResult::Continue
            }
            KeyCode::Esc => {
                RegisterResult::Exit
            }
            // 切换到登录界面
            KeyCode::Char('l') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                RegisterResult::SwitchToLogin
            }
            KeyCode::Char(c) => {
                // 清除之前的消息
                self.error_message = None;
                self.success_message = None;
                
                match self.focus {
                    RegisterFocus::Username => {
                        // 只允许字母和数字
                        if c.is_ascii_alphanumeric() {
                            self.username.push(c);
                        }
                    }
                    RegisterFocus::Password => {
                        self.password.push(c);
                    }
                    RegisterFocus::ConfirmPassword => {
                        self.confirm_password.push(c);
                    }
                }
                RegisterResult::Continue
            }
            _ => RegisterResult::Continue,
        }
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