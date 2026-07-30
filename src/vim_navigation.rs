use crossterm::event::{KeyCode, KeyEvent};

pub fn handle (
    key: KeyEvent,
    pending: &mut Option<char>,
    selected: &mut Option<usize>,
    len: usize,
) -> bool {
    let current = selected.unwrap_or(0);

    match key.code {
        KeyCode::Char('j') => {
            if current + 1 < len {
                *selected = Some(current + 1);
            }
            true
        }

        KeyCode::Char('k') => {
            if current > 0 {
                *selected = Some(current - 1);
            }
            true
        }

        KeyCode::Char('G') => {
            if len > 0 {
                *selected = Some(len - 1);
            }
            true
        }

        KeyCode::Char('g') => {
            if *pending == Some('g') {
                if len > 0 {
                    *selected = Some(0);
                }
                *pending = None;
                true
            } else {
                *pending = Some('g');
                true
            }
        }

        _ => false,
    }
}
