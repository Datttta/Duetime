use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Insert,
    Visual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputResult {
    Ignored,
    Consumed,
    TextChanged,
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
    pub fn handle_vim_mode(&mut self, key: KeyEvent, mode: &mut InputMode, max_len: usize) -> InputResult {
        match *mode {
            InputMode::Normal => self.handle_normal(key, mode),
            InputMode::Insert => self.handle_insert(key, mode, max_len),
            InputMode::Visual => self.handle_visual(key, mode),
        }
    }

    fn handle_normal(&mut self, key: KeyEvent, mode: &mut InputMode) -> InputResult {
        match key.code {
            KeyCode::Char('v') => {
                *mode = InputMode::Visual;
                InputResult::Consumed
            }

            KeyCode::Char('i') => {
                *mode = InputMode::Insert;
                InputResult::Consumed
            }
            
            KeyCode::Char('I') => {
                self.cursor = 0;
                *mode = InputMode::Insert;
                InputResult::Consumed
            }

            KeyCode::Char('a') => {
                if self.cursor < self.text.chars().count() {
                    self.cursor += 1;
                } 
                *mode = InputMode::Insert;
                InputResult::Consumed
            }
            
            KeyCode::Char('A') => {
                self.cursor = self.text.len();
                *mode = InputMode::Insert;
                InputResult::Consumed
            }

            KeyCode::Char('h') => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                InputResult::Consumed
            }

            KeyCode::Char('l') => {
                if self.cursor < self.text.len(){
                    self.cursor += 1;
                }
                InputResult::Consumed
            }

            KeyCode::Char('0') => {
                self.cursor = 0;
                InputResult::Consumed
            }

            KeyCode::Char('$') => {
                self.cursor = self.text.chars().count();
                InputResult::Consumed
            }
            
            _ => InputResult::Ignored,
        }
    }

    pub fn handle_insert(&mut self, key: KeyEvent, mode: &mut InputMode, max_len: usize) -> InputResult {
        match key.code {
            KeyCode::Char(c) => {
                if self.text.len() < max_len {
                    let byte_index = self
                        .text
                        .char_indices()
                        .nth(self.cursor)
                        .map(|(i, _)| i)
                        .unwrap_or(self.text.len());

                    self.text.insert(byte_index, c);
                    self.cursor += 1;
                
                    InputResult::TextChanged
                } else {
                    InputResult::Consumed
                }
            }

            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;

                    let byte_index = self
                        .text
                        .char_indices()
                        .nth(self.cursor)
                        .map(|(i, _)| i)
                        .unwrap_or(self.text.len());

                    self.text.remove(byte_index);
                
                    InputResult::TextChanged
                } else {
                    InputResult::Consumed
                }
            }
            
            KeyCode::Delete => {
                if self.cursor < self.text.len() {

                    let byte_index = self
                        .text
                        .char_indices()
                        .nth(self.cursor)
                        .map(|(i, _)| i)
                        .unwrap_or(self.text.len());

                    self.text.remove(byte_index);
                
                    InputResult::TextChanged
                } else {
                    InputResult::Consumed
                }
            }

            KeyCode::Esc => {
                *mode = InputMode::Normal;
                InputResult::Consumed
            }

            _ => InputResult::Ignored,
        }
    }

    fn handle_visual(&mut self, key: KeyEvent, mode: &mut InputMode) -> InputResult {
        match key.code {
            KeyCode::Esc => {
                *mode = InputMode::Normal;
                InputResult::Consumed
            }

            KeyCode::Char('h') => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                InputResult::Consumed
            }

            KeyCode::Char('l') => {
                if self.cursor < self.text.chars().count() {
                    self.cursor += 1;
                }
                InputResult::Consumed
            }

            _ => InputResult::Ignored,
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor = 0;
    }
}


