use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;
use std::io::stdout;
use std::sync::Arc;
use anyhow::Result;
use tokio::sync::{Mutex};
use core::api::client;

mod ui;
mod repository;
mod remote;
mod cache;
mod asynrt;
mod ext;

use crate::{ui::{models::Me}};

use crate::{repository::db, ui::models::App};


fn main() -> Result<()> {
    dotenv::dotenv().ok();
    // 初始化日志
    core::log::init(Some(".pubchat_client.log"));
    asynrt::init();
    db::init();
    client::init();
    cache::init();
    ext::init();
    
    // 进入原始模式
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    
    // 创建终端
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    
    // 登录或注册，获取token
    let token = crate::ui::screen::auth::auth(&mut terminal);
    if token.is_err() {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        return Err(token.unwrap_err());
    }
    let token = token.unwrap();
    // 使用token建立TCP连接
    let (stream, user_id, user_name) = asynrt::get().block_on(remote::connection::connect_with_token(&token))?;
    let (reader, writer) = stream.into_split();
    let (sx, mut rx) = tokio::sync::mpsc::channel::<core::api::types::message::Message>(100);
    // 开启接收消息任务
    asynrt::get().block_on(remote::connection::receive_messages(reader, sx));
    show_main_screen(&mut terminal, token, Me {id: user_id, name: user_name}, writer, &mut rx)?;
    
    // 退出原始模式
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn show_main_screen(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, 
    token: String,
    me: Me,
    writer: tokio::net::tcp::OwnedWriteHalf,
    rx: &mut tokio::sync::mpsc::Receiver<core::api::types::message::Message>,
) -> Result<()> {
    
    let shared_writer = Arc::new(Mutex::new(writer));
    // 登录成功后，创建应用状态
    let mut app = App::new(token, me, shared_writer);
    // 主事件循环
    loop {
        terminal.draw(|frame| app.render(frame))?;
        if event::poll(std::time::Duration::from_millis(50))? {
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
                app.handle(key);
            }
        }
        if let Ok(message) = rx.try_recv() {
            // 接收到消息，将消息添加到缓存中
            app.chat.chat.messages.push(crate::ui::models::Message::new(message.uname, message.content, false));
        }
    }
}