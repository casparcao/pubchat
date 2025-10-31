use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use log::info;
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;
use std::io::stdout;
use std::sync::Arc;
use anyhow::Result;
use tokio::sync::Mutex;

mod ui;
mod repository;
mod service;
mod common;
mod cache;

use crate::{repository::token::{clear_token, is_token_valid, load_token, save_token}, service::session, ui::renderers::login::{LoginResult, LoginScreen}};

use crate::{repository::db, ui::models::App};

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    // 初始化日志
    common::log::init();
    db::init();
    service::init();
    cache::init();
    // 进入原始模式
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    
    // 创建终端
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    
    // 首先尝试从本地文件加载token
    let token = if let Ok(Some(stored_token)) = load_token() {
        if is_token_valid(&stored_token) {
            // Token有效，尝试直接连接
            stored_token.token
        } else {
            // Token过期，清除它并重新登录
            let _ = clear_token();
            show_login_screen(&mut terminal)?
        }
    } else {
        // 没有找到存储的token，显示登录界面
        show_login_screen(&mut terminal)?
    };
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    // 使用token建立TCP连接
    let (stream, user_id) = rt.block_on(service::connection::connect_with_token(&token))?;
    let (reader, writer) = stream.into_split();
    // 开启接收消息任务
    rt.block_on(service::connection::receive_messages(reader));
    show_main_screen(&mut terminal, token, user_id, writer)?;
    // 退出原始模式
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

// 显示登录界面并处理登录逻辑
fn show_login_screen(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<String> {
    let mut login_screen = LoginScreen::new();
    loop {
        terminal.draw(|frame| login_screen.render(frame))?;
        
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            
            let result = login_screen.handle_key_event(key);
            match result {
                LoginResult::Success(token) => {
                    // 登录成功，保存token到本地文件
                    let _ = save_token(&token.token, token.exp);
                    break Ok(token.token);
                }
                LoginResult::Exit => {
                    // 用户按ESC退出
                    disable_raw_mode()?;
                    stdout().execute(LeaveAlternateScreen)?;
                    std::process::exit(0);
                }
                LoginResult::Continue => {
                    // 继续登录循环
                    continue;
                }
            }
        }
    }
}

fn show_main_screen(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, 
    token: String,
    user_id: u64,
    writer: tokio::net::tcp::OwnedWriteHalf
) -> Result<()> {
    // 获取好友列表
    // let friends : Vec<friend::FriendResponse> = service::friend::get_friends(&token).await?;
    let sessions = session::get_sessions(&token)?;
    info!("sessions: {:?}", sessions);
    // 登录成功后，创建应用状态
    let mut app = App::new();
    // 更新联系人列表为从服务器获取的好友列表
    // app.contacts = friends.into_iter()
    //     .map(|f| crate::ui::models::Contact::from_friend_response(f))
    //     .collect();
    app.sessions = sessions.into_iter()
        .map(|s| crate::ui::models::Session::from_session_response(s))
        .collect();
    app.set_token(Some(token.clone()));
    app.current_user_id = user_id; // 设置当前用户ID
    // Split the TCP stream into read and write halves
    let shared_writer = Arc::new(Mutex::new(writer));
    app.set_stream(shared_writer);
    // 主事件循环
    loop {
        terminal.draw(|frame| app.render(frame))?;
        if let Event::Key(key) = event::read()? {
            // 只处理按键按下事件，忽略按键释放事件
            // 这可以解决Windows系统上重复字符输入的问题
            if key.kind != KeyEventKind::Press {
                continue;
            }
            if key.code == KeyCode::Char('c') && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                // 按下Ctrl+C，退出程序
                return Ok(());
            }
            // 将所有按键事件交给应用程序处理
            if app.handle_key_event(key) {
                return Ok(());
            }
        }
    }
}