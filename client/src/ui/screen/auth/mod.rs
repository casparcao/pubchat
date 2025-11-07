pub mod login;
pub mod register;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
};
use ratatui::Terminal;
use anyhow::Result;


use crate::{repository::token::{clear_token, is_token_valid, load_token, save_token}, ui::{screen::auth::{login::{LoginResult, LoginScreen}, register::{RegisterResult, RegisterScreen}}}};



enum AuthPage {
    Login(LoginScreen),
    Register(RegisterScreen),
}

pub fn auth<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<String> { 
    // 首先尝试从本地文件加载token
    if let Ok(Some(stored_token)) = load_token() && false {
        if is_token_valid(&stored_token) {
            // Token有效，尝试直接连接
            Ok(stored_token.token)
        } else {
            // Token过期，清除它并重新登录
            let _ = clear_token();
            run_auth_loop(terminal)
        }
    } else {
        // 没有找到存储的token，进入认证循环
        run_auth_loop(terminal)
    }
}

// 运行认证循环，支持登录和注册界面切换
fn run_auth_loop<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<String> {
    // 初始化应用状态为登录界面
    let mut app_state = AuthPage::Login(LoginScreen::new());
    
    loop {
        // 渲染当前界面
        terminal.draw(|f| {
            match &mut app_state {
                AuthPage::Login(login_screen) => login_screen.render(f),
                AuthPage::Register(register_screen) => register_screen.render(f),
            }
        })?;
        
        // 处理事件
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // 只处理按键按下事件
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                
                // 全局快捷键：Ctrl+C 退出程序
                if key.code == KeyCode::Char('c') && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                    return Err(anyhow::anyhow!("Exiting..."));
                }
                
                match &mut app_state {
                    AuthPage::Login(login_screen) => {
                        match login_screen.handle_key_event(key) {
                            LoginResult::Success(token) => {
                                // 登录成功，保存token
                                let _ = save_token(&token.token, token.exp);
                                return Ok(token.token);
                            }
                            LoginResult::Continue => {
                                // 继续循环
                            }
                            LoginResult::Exit => {
                                // 切换到注册界面或退出
                                app_state = AuthPage::Register(RegisterScreen::new());
                            }
                        }
                    }
                    AuthPage::Register(register_screen) => {
                        match register_screen.handle_key_event(key) {
                            RegisterResult::Success => {
                                // 注册成功，切换回登录界面
                                app_state = AuthPage::Login(LoginScreen::new());
                            }
                            RegisterResult::SwitchToLogin => {
                                // 切换到登录界面
                                app_state = AuthPage::Login(LoginScreen::new());
                            }
                            RegisterResult::Continue => {
                                // 继续循环
                            }
                            RegisterResult::Exit => {
                                // 切换回登录界面
                                app_state = AuthPage::Login(LoginScreen::new());
                            }
                        }
                    }
                }
            }
        }
    }
}