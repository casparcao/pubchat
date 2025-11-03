use crate::ui::models::{App, Mode};
use crossterm::event::{KeyEvent, KeyCode};

impl App {
    /// 处理 Insert 模式下的按键事件
    pub fn handle_insert_mode_key_event(&mut self, key: KeyEvent) -> bool {
        let should_exit = false;
        
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
            }
            KeyCode::Enter => {
                self.chat.chat.send_message(&self.stream);
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
        
        should_exit
    }
}