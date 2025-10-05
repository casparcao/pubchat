use crate::ui::App;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;
use std::io::{stdout, Result};

mod ui;
mod codec;
mod proto;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    // 进入原始模式
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    // 创建终端
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    // 创建应用状态
    let mut app = App::new();
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