use crossterm::event::{KeyCode, KeyEvent};

#[derive(Default)]
pub struct InputState {
    pub text: String,
    pub cursor: usize,
}

impl InputState {
    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.text.insert(self.cursor, c);
                self.cursor += 1;
            }

            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.text.remove(self.cursor);
                }
            }

            KeyCode::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }

            KeyCode::Right => {
                if self.cursor < self.text.len() {
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
