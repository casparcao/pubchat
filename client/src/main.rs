use crate::ui::App;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;
use std::io::{stdout, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

mod ui;
mod login_ui;

use login_ui::{LoginState, LoginResult};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use core::proto::message::{Message, ConnectRequest, Type};
use core::proto::codec::{encode, decode};

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 进入原始模式
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    
    // 创建终端
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    
    // 显示登录界面
    let mut login_state = LoginState::new();
    let token = loop {
        terminal.draw(|frame| login_state.render(frame))?;
        
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            
            let result = login_state.handle_key_event(key).await;
            match result {
                LoginResult::Success(token) => {
                    break token;
                }
                LoginResult::Exit => {
                    // 用户按ESC退出
                    disable_raw_mode()?;
                    stdout().execute(LeaveAlternateScreen)?;
                    return Ok(());
                }
                LoginResult::Continue => {
                    // 继续登录循环
                    continue;
                }
            }
        }
    };
    
    // 使用token建立TCP连接
    let stream = connect_with_token(&token).await?;
    
    // 登录成功后，创建应用状态
    let mut app = App::new();
    app.set_token(Some(token));
    
    // 创建Arc<Mutex<TcpStream>>用于发送和接收
    let shared_stream = Arc::new(Mutex::new(stream));
    app.set_stream(shared_stream.clone());
    
    // 启动接收消息的任务
    let recv_app = app.clone();
    tokio::spawn(async move {
        receive_messages(shared_stream, recv_app).await;
    });
    
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
                break;
            }
            
            // 将所有按键事件交给应用程序处理
            if app.handle_key_event(key) {
                break;
            }
        }
    }
    
    // 退出原始模式
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

// 使用token建立TCP连接
async fn connect_with_token(token: &str) -> Result<TcpStream> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
    // 创建连接请求消息
    let connect_request = Message {
        id: 1,
        ts: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        r#type: Type::ConnectRequest as i32,
        content: Some(core::proto::message::message::Content::ConnectRequest(ConnectRequest {
            token: token.to_string(),
        })),
    };
    
    // 发送连接请求
    let encoded = encode(&connect_request)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    stream.write_all(&encoded)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    // 读取连接响应
    let _response = decode::<Message, _>(&mut stream)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
    Ok(stream)
}

// 接收消息的异步任务
async fn receive_messages(stream: Arc<Mutex<TcpStream>>, mut app: App) {
    loop {
        let mut stream_guard = stream.lock().await;
        match decode::<Message, _>(&mut *stream_guard).await {
            Ok(msg) => {
                drop(stream_guard); // 释放锁
                
                // 处理接收到的消息
                match msg.r#type {
                    t if t == Type::ChatRequest as i32 => {
                        if let Some(core::proto::message::message::Content::ChatRequest(chat_req)) = msg.content {
                            // 更新UI中的消息
                            app.add_received_message(chat_req);
                        }
                    }
                    _ => {
                        // 其他类型消息暂不处理
                    }
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }
}