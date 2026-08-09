use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationMode {
    Normal,
    Visual,
}

pub fn handle (
    key: KeyEvent,
    pending: &mut Option<char>,
    selected: &mut Option<usize>,
    len: usize,
    mode: &mut NavigationMode,
    n_visual_start: &mut Option<usize>,
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

        KeyCode::Char('v') => {
            match *mode {
                NavigationMode::Normal => {
                    *mode = NavigationMode::Visual;
                    *n_visual_start = Some(current);
                }

                NavigationMode::Visual => {
                    *mode = NavigationMode::Normal;
                    *n_visual_start = None;
                }
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

        KeyCode::Esc => {
            *mode = NavigationMode::Normal;
            *n_visual_start = None;
            true
        }

        _ => false,
    }
}
