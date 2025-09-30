use crate::ui::App;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;
use std::io::{stdout, Result};

mod ui;

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
            if key.code == KeyCode::Char('c') && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                break;
            }
            
            // 处理导航键
            match key.code {
                KeyCode::Up => {
                    match app.current_view {
                        ui::View::Contacts => {
                            if let Some(selected) = app.selected_contact {
                                app.selected_contact = Some(selected.saturating_sub(1));
                            } else {
                                app.selected_contact = Some(0);
                            }
                        }
                        ui::View::Groups => {
                            if let Some(selected) = app.selected_group {
                                app.selected_group = Some(selected.saturating_sub(1));
                            } else {
                                app.selected_group = Some(0);
                            }
                        }
                        _ => {
                            if app.handle_key_event(key) {
                                break;
                            }
                        }
                    }
                }
                KeyCode::Down => {
                    match app.current_view {
                        ui::View::Contacts => {
                            if app.contacts.len() > 0 {
                                if let Some(selected) = app.selected_contact {
                                    app.selected_contact = Some((selected + 1).min(app.contacts.len() - 1));
                                } else {
                                    app.selected_contact = Some(0);
                                }
                            }
                        }
                        ui::View::Groups => {
                            if app.groups.len() > 0 {
                                if let Some(selected) = app.selected_group {
                                    app.selected_group = Some((selected + 1).min(app.groups.len() - 1));
                                } else {
                                    app.selected_group = Some(0);
                                }
                            }
                        }
                        _ => {
                            if app.handle_key_event(key) {
                                break;
                            }
                        }
                    }
                }
                _ => {
                    if app.handle_key_event(key) {
                        break;
                    }
                }
            }
        }
    }
    // 退出原始模式
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}