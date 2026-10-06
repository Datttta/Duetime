use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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
        if key.modifiers.contains(KeyModifiers::CONTROL)
            && key.code == KeyCode::Char('v')
            && *mode != InputMode::Insert
        {
            return InputResult::Consumed;
        }

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

            KeyCode::Char('0') | KeyCode::Char('_') => {
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
            
            KeyCode::Char('c') => {
                if self.pending_command == Some('c') {
                    self.text.clear();
                    self.pending_command = None;
                    self.cursor = 0;
                
                    *mode = InputMode::Insert;
                    InputResult::TextChanged
                } else {
                    self.pending_command = Some('c');
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
                if self.cursor > 0 {
                    self.cursor -= 1;
                }

                *mode = InputMode::Normal;
                self.pending_command = None;
                InputResult::Consumed
            }

            _ => InputResult::Ignored,
        }
    }

    fn handle_visual(&mut self, key: KeyEvent, mode: &mut InputMode) -> InputResult {
        match key.code {
            KeyCode::Esc => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }

                *mode = InputMode::Normal;
                self.visual_start = None;
                self.pending_command = None;
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
            
            KeyCode::Char('0') | KeyCode::Char('_') => {
                self.cursor = 0;
                InputResult::Consumed
            }
            
            KeyCode::Char(c) if c == 'x' || c == 'd' || c == 'c' || c == 'y' => {
                if let Some(start) = self.visual_start {
                    let mut chars: Vec<char> = self.text.chars().collect();
                    let min_idx = std::cmp::min(start, self.cursor);
                    let max_idx = std::cmp::max(start, self.cursor);

                    if max_idx < chars.len() {
                        let selected: String = chars[min_idx..=max_idx].iter().collect();

                        if let Ok(mut clipboard) = arboard::Clipboard::new() {
                            let _ = clipboard.set_text(selected);
                        }
                        
                        if c != 'y' {
                            chars.drain(min_idx..=max_idx);
                            self.text = chars.into_iter().collect();
                        }
                        self.cursor = min_idx;
                        if self.cursor >= self.text.chars().count() && self.cursor > 0 {
                            self.cursor -= 1;
                        }
                    }
                }
                
                if c == 'c' {
                    *mode = InputMode::Insert;
                } else {
                    *mode = InputMode::Normal;
                }

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

    pub fn insert_str(&mut self, s: &str, max_len: usize) -> InputResult {
        // Calculate the maximum number of characters allowed to be added
        let current_len = self.text.len();
        if current_len >= max_len {
            return InputResult::Consumed;
        }

        // Limit the pasted text if it exceeds max_len
        let available_space = max_len - current_len;
        let text_to_insert = if s.len() > available_space {
            // Find a safe UTF-8 boundary to truncate if needed
            let mut end_idx = available_space;
            while !s.is_char_boundary(end_idx) && end_idx > 0 {
                end_idx -= 1;
            }
            &s[..end_idx]
        } else {
            s
        };

        if text_to_insert.is_empty() {
            return InputResult::Consumed;
        }

        // Find the byte index matching the current character cursor position
        let byte_index = self
            .text
            .char_indices()
            .nth(self.cursor)
            .map(|(i, _)| i)
            .unwrap_or(self.text.len());

        // Insert the string and advance the cursor by the number of characters inserted
        self.text.insert_str(byte_index, text_to_insert);
        self.cursor += text_to_insert.chars().count();

        InputResult::TextChanged
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor = 0;
        self.visual_start = None;
    }
}
