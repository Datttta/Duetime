use crate::{
    app::App,
    navigation::vim_navigation,
};

use super::actions;
use crossterm::event::{KeyCode, KeyEvent};
use log::info;

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    let mut selected = if app.timers.is_empty() {
        None
    } else {
        app.timers_list_state.selected()
    };

    app.timers_list_state.select(selected);

    match key.code {
        KeyCode::Char('a') => {
            actions::add_timer(app);
        }

        //KeyCode::Char('i') => {
        //    actions::inbox_item_info(app);
        //}

        KeyCode::Char('e') => {
            actions::edit_timer(app);
        }
        
        KeyCode::Char('r') => {
            actions::reset_timer(app);
        }
        
        // timer
        KeyCode::Char('s') => {
            actions::start_stop(app);
        }

        KeyCode::Char('l') => {
            actions::move_right(app)
        }
        
        KeyCode::Char('h') => {
            actions::move_left(app)
        }

        KeyCode::Char('k') => {
            actions::move_up(app)
        }
        
        KeyCode::Char('j') => {
            actions::move_down(app)
        }

        KeyCode::Char('d') => {
            if app.pending_command == Some('d') {
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

