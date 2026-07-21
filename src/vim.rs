use crossterm::event::{KeyCode, KeyEvent};
use crate::App;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Insert,
    Visual,
}

pub struct InputState {
    pub text: String,
    pub cursor: usize,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
        }
    }

}

impl InputState {
    pub fn handle_key(&mut self, key: KeyEvent, mode: &mut InputMode, max_len: usize) {
        match *mode {
            InputMode::Normal => self.handle_normal(key, mode),
            InputMode::Insert => self.handle_insert(key, mode, max_len),
            InputMode::Visual => self.handle_visual(key, mode),
        }
    }

    fn handle_normal(&mut self, key: KeyEvent, mode: &mut InputMode) {
        match key.code {
            KeyCode::Esc => {
                *mode = InputMode::Normal;
            }

            KeyCode::Char('v') => {
                *mode = InputMode::Visual;
            }

            KeyCode::Char('i') => {
                *mode = InputMode::Insert;
            }

            KeyCode::Char('a') => {
                if self.cursor < self.text.len() {
                    self.cursor += 1;
                } 
                *mode = InputMode::Insert;
            }

            KeyCode::Char('h') => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }

            KeyCode::Char('l') => {
                if self.cursor < self.text.len(){
                    self.cursor += 1;
                }
            }

            KeyCode::Char('0') => {
                self.cursor = 0;
            }

            KeyCode::Char('$') => {
                self.cursor = self.text.len();
            }
            
            _ => {}
        }
    }

    pub fn handle_insert(&mut self, key: KeyEvent, mode: &mut InputMode, max_len: usize) {
        match key.code {
            KeyCode::Char(c) => {
                if self.text.len() < max_len {
                    self.text.insert(self.cursor, c);
                    self.cursor += 1;
                }
            }

            KeyCode::Esc => {
                *mode = InputMode::Normal;
            }

            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.text.remove(self.cursor);
                }
            }

            _ => {}
        }
    }

    fn handle_visual(&mut self, key: KeyEvent, mode: &mut InputMode) {
        match key.code {
            KeyCode::Esc => {
                *mode = InputMode::Normal;
            }

            KeyCode::Char('h') => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }

            KeyCode::Char('l') => {
                if self.cursor < self.text.len(){
                    self.cursor += 1;
                }
            }

            _ => {}
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor = 0;
    }
}


