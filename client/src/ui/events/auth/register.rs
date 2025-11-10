use crossterm::event::{KeyCode, KeyEvent};
use core::api::{client::login, types::auth::RegisterRequest};

use crate::ui::screen::auth::register::{RegisterFocus, RegisterResult, RegisterScreen};


impl RegisterScreen {

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
}