use crate::{
    app::App,
};

use super::actions;
use crossterm::event::{KeyCode, KeyEvent};
//use log::info;

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    let selected = if app.timers.is_empty() {
        None
    } else {
        app.timers_list_state.selected()
    };

    app.timers_list_state.select(selected);

    match key.code {
        KeyCode::Char('a') => {
            app.previous_panel = app.focused_panel;
            actions::add_timer(app);
        }

        KeyCode::Char('i') => {
            app.previous_panel = app.focused_panel;
            actions::timer_info(app);
        }

        KeyCode::Char('e') => {
            app.previous_panel = app.focused_panel;
            actions::edit_timer(app);
        }
        
        KeyCode::Char('r') => {
            app.previous_panel = app.focused_panel;
            actions::reset_timer(app);
        }
        
        // timer
        KeyCode::Char('s') => {
            app.previous_panel = app.focused_panel;
            actions::start_stop(app);
        }

        KeyCode::Char('l') => {
            app.previous_panel = app.focused_panel;
            actions::move_right(app)
        }
        
        KeyCode::Char('h') => {
            app.previous_panel = app.focused_panel;
            actions::move_left(app)
        }

        KeyCode::Char('k') => {
            app.previous_panel = app.focused_panel;
            actions::move_up(app)
        }
        
        KeyCode::Char('j') => {
            app.previous_panel = app.focused_panel;
            actions::move_down(app)
        }

        KeyCode::Char('d') => {
            if app.pending_command == Some('d') {
                app.previous_panel = app.focused_panel;
                actions::delete_timer(app);
                app.pending_command = None;
            } else {
                app.pending_command = Some('d');
            }
        }

        _ => {
            app.pending_command = None;
        }
    }
}

