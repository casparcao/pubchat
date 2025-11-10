use core::{api::types::auth::LoginRequest, auth::Token};
use log::info;
use crossterm::event::{KeyCode, KeyEvent};
use core::api::client::login;
use anyhow::Result;

use crate::ui::screen::auth::login::{LoginFocus, LoginResult, LoginScreen};


impl LoginScreen {
    pub fn handle_key_event(&mut self, key: KeyEvent) -> LoginResult {
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
                    let request = LoginRequest {
                        username: self.username.clone(),
                        password: self.password.clone(),
                    };
                    let result : Result<Token> = login::login(&request);
                    match result {
                        Ok(token) => {
                            self.is_logging_in = false;
                            self.error_message = Some("Login successful".to_string());
                            info!("Login successful");
                            return LoginResult::Success(token);
                        }
                        Err(e) => {
                            self.is_logging_in = false;
                            self.error_message = Some(e.to_string());
                            info!("Login failed: {}", e);
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
    
}
