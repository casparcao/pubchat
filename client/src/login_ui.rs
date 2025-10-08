use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Clear},
};
use crossterm::event::{KeyCode, KeyEvent};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct LoginState {
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
    Success(String), // 登录成功，返回token
    Continue,
    Exit,
}

impl Default for LoginState {
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

impl LoginState {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn handle_key_event(&mut self, key: KeyEvent) -> LoginResult {
        // 如果正在登录中，不处理其他按键
        if self.is_logging_in {
            return LoginResult::Continue;
        }

        match key.code {
            KeyCode::Enter => {
                if !self.username.is_empty() && !self.password.is_empty() {
                    // 开始登录过程
                    self.is_logging_in = true;
                    self.error_message = None;
                    
                    // 执行登录操作
                    match self.perform_login().await {
                        Ok(token) => {
                            self.is_logging_in = false;
                            self.error_message = Some("Login successful".to_string());
                            return LoginResult::Success(token);
                        }
                        Err(e) => {
                            self.is_logging_in = false;
                            self.error_message = Some(e);
                            return LoginResult::Continue;
                        }
                    }
                } else {
                    self.error_message = Some("Username and password required".to_string());
                    LoginResult::Continue
                }
            }
            KeyCode::Tab => {
                // 在用户名和密码之间切换焦点
                self.focus = match self.focus {
                    LoginFocus::Username => LoginFocus::Password,
                    LoginFocus::Password => LoginFocus::Username,
                };
                LoginResult::Continue
            }
            KeyCode::Backspace => {
                match self.focus {
                    LoginFocus::Username => {
                        self.username.pop();
                    }
                    LoginFocus::Password => {
                        self.password.pop();
                    }
                }
                LoginResult::Continue
            }
            KeyCode::Esc => {
                LoginResult::Exit
            }
            KeyCode::Char(c) => {
                // 清除之前的错误消息
                self.error_message = None;
                
                match self.focus {
                    LoginFocus::Username => {
                        self.username.push(c);
                    }
                    LoginFocus::Password => {
                        self.password.push(c);
                    }
                }
                LoginResult::Continue
            }
            _ => LoginResult::Continue,
        }
    }

    // 执行登录操作
    pub async fn perform_login(&mut self) -> Result<String, String> {
        // 创建HTTP客户端
        let client = reqwest::Client::new();
        
        // 构造登录请求
        let login_request = LoginRequest {
            username: self.username.clone(),
            password: self.password.clone(),
        };
        
        // 发送登录请求
        let response = client
            .post("http://127.0.0.1:3000/login")
            .json(&login_request)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;
        
        // 检查响应状态
        if response.status().is_success() {
            let token_response: TokenResponse = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(token_response.data.token)
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Login failed with status {}: {}", status, error_text))
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let size = frame.size();
        
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
                frame.set_cursor(
                    chunks[1].x + self.username.len() as u16 + 1,
                    chunks[1].y + 1,
                );
            } else if matches!(self.focus, LoginFocus::Password) {
                frame.set_cursor(
                    chunks[3].x + self.password.len() as u16 + 1,
                    chunks[3].y + 1,
                );
            }
        }
    }
    
    // 获取登录状态
    pub fn is_logging_in(&self) -> bool {
        self.is_logging_in
    }
}

// 登录请求结构
#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

// Token响应结构
#[derive(Deserialize)]
struct TokenResponse {
    data: Token,
    ok: bool,
}

#[derive(Debug, Deserialize)]
struct Token {
    token: String,
    exp: u128,
}