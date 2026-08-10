use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

pub fn move_tasks(app: &mut App) {
    if let Some(index) = app.table_state.selected() {
        app.moving_task = Some(index);
        app.move_position = Some(index);
    }
}

pub fn handle_keys(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('j') => {
            if let Some(position) = app.move_position {
                if position < app.tasks.len() {
                    app.move_position = Some(position + 1);
                }
            }
            true
        }

        KeyCode::Char('k') => {
            if let Some(position) = app.move_position {
                if position > 0 {
                    app.move_position = Some(position - 1);
                }
            }
            true
        }

        KeyCode::Enter => {
            app.finish_move();
            true
        }

        KeyCode::Esc => {
            app.cancel_move();
            true
        }

        _ => true,
    }
}
