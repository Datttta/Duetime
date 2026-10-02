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
    pub visual_start: Option<usize>,
    pub pending_command: Option<char>,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            visual_start: None,
            pending_command: None,
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
            KeyCode::Char('h') => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                InputResult::Consumed
            }

            KeyCode::Char('l') => {
                let max_cursor = self.text.chars().count().saturating_sub(1);

                if self.cursor < max_cursor {
                    self.cursor += 1;
                }

                InputResult::Consumed
            }

            KeyCode::Char('v') => {
                if !self.text.is_empty() {
                    *mode = InputMode::Visual;

                    let last = self.text.chars().count() - 1;

                    self.cursor = self.cursor.min(last);
                    self.visual_start = Some(self.cursor);
                }

                InputResult::Consumed
            }

            KeyCode::Char('V') => {
                *mode = InputMode::Visual;
                self.visual_start = Some(0);
                let count = self.text.chars().count();
                self.cursor = if count > 0 { count - 1 } else { 0 };
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
                self.cursor = self.text.chars().count();
                *mode = InputMode::Insert;
                InputResult::Consumed
            }

            KeyCode::Char('0') => {
                self.cursor = 0;
                InputResult::Consumed
            }

            KeyCode::Char('$') => {
                let count = self.text.chars().count();
                self.cursor = if count > 0 { count - 1 } else { 0 };
                InputResult::Consumed
            }

            KeyCode::Char('x') => {
                let mut chars: Vec<char> = self.text.chars().collect();
                if self.cursor < chars.len() {
                    let removed = chars.remove(self.cursor);

                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                        let _ = clipboard.set_text(removed.to_string());
                    }

                    self.text = chars.clone().into_iter().collect();

                    if self.cursor >= chars.len() && self.cursor > 0 {
                        self.cursor -= 1;
                    }
                    InputResult::TextChanged
                } else {
                    InputResult::Consumed
                }
            }

            KeyCode::Char('d') => {
                if self.pending_command == Some('d') {
                    self.text.clear();
                    self.pending_command = None;
                    self.cursor = 0;
                    InputResult::TextChanged
                } else {
                    self.pending_command = Some('d');
                    InputResult::Consumed
                }
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
                let char_count = self.text.chars().count();
                if self.cursor < char_count {
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
                self.visual_start = None;
                InputResult::Consumed
            }

            KeyCode::Char('h') => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                InputResult::Consumed
            }

            KeyCode::Char('l') => {
                let max_cursor = self.text.chars().count();
                if self.cursor < max_cursor {
                    self.cursor += 1;
                }
                InputResult::Consumed
            }
            
            KeyCode::Char('d') => {
                if let Some(start) = self.visual_start {
                    let mut chars: Vec<char> = self.text.chars().collect();
                    let min_idx = std::cmp::min(start, self.cursor);
                    let max_idx = std::cmp::max(start, self.cursor);

                    if max_idx < chars.len() {
                        chars.drain(min_idx..=max_idx);
                        self.text = chars.into_iter().collect();
                        self.cursor = min_idx;
                        if self.cursor >= self.text.chars().count() && self.cursor > 0 {
                            self.cursor -= 1;
                        }
                    }
                }

                *mode = InputMode::Normal;
                self.visual_start = None;
                InputResult::TextChanged
            }

            KeyCode::Char('x') => {
                if let Some(start) = self.visual_start {
                    let mut chars: Vec<char> = self.text.chars().collect();
                    let min_idx = std::cmp::min(start, self.cursor);
                    let max_idx = std::cmp::max(start, self.cursor);

                    if max_idx < chars.len() {
                        let selected: String = chars[min_idx..=max_idx].iter().collect();

                        if let Ok(mut clipboard) = arboard::Clipboard::new() {
                            let _ = clipboard.set_text(selected);
                        }

                        chars.drain(min_idx..=max_idx);
                        self.text = chars.into_iter().collect();
                        self.cursor = min_idx;
                        if self.cursor >= self.text.chars().count() && self.cursor > 0 {
                            self.cursor -= 1;
                        }
                    }
                }
                *mode = InputMode::Normal;
                self.visual_start = None;
                InputResult::TextChanged
            }

            KeyCode::Char('V') => {
                *mode = InputMode::Visual;
                self.visual_start = Some(0);
                let count = self.text.chars().count();
                self.cursor = if count > 0 { count - 1 } else { 0 };
                InputResult::Consumed
            }

            _ => InputResult::Ignored,
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor = 0;
        self.visual_start = None;
    }
}
