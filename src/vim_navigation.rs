use ratatui::widgets::TableState;

use crossterm::event::{KeyCode, KeyEvent};

pub enum Navigation {
    None,
    Consumed,
}

pub fn handle (
    Key: KeyEvent,
    pending: &mut Option<char>,
    state: &mut TableState,
    len: usize,
) -> bool {
    match Key.code {
        KeyCode::Char('k') => {
            let selected = state.selected().unwrap_or(0);
                if selected > 0 {
                    state.select(Some(selected - 1));
                }
                true
        }

        KeyCode::Char('j') => {
            let selected = state.selected().unwrap_or(0);
                if selected + 1 < len {
                    state.select(Some(selected + 1));
                }
                true
        }

        KeyCode::Char('G') => {
            if len > 0 {
                state.select(Some(len - 1));
            }
            true
        }

        KeyCode::Char('g') => {
            if *pending == Some('g') {
                state.select(Some(0));
                *pending = None;
            } else {
                *pending = Some('g');
            }
            true
        }

        _ => false
    }
}
