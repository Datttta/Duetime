use crate::{
    app::App,
    vim_navigation,
};

use super::actions;

use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    let mut selected = app.inbox_table_state.selected();

    let handled = vim_navigation::handle(
        key,
        &mut app.pending_command,
        &mut selected,
        app.inbox_items.len(),
        &mut app.n_mode,
        &mut app.n_visual_start,
    );

    app.inbox_table_state.select(selected);

    if handled {
        return;
    }

    match key.code {
        KeyCode::Char('a') => {
            app.pending_command = Some('a');
        }

        KeyCode::Char('i') => {
            if app.pending_command == Some('a') {
                actions::inbox_item_add_popup(app);
                app.pending_command = None;
            } else {
                actions::inbox_item_info(app);
            }
        }

        KeyCode::Char('e') => {
            actions::edit_inbox_item(app);
        }
        
        KeyCode::Char('c') => {
            if app.pending_command == Some('c') {
                app.copy_inbox_input();
                app.pending_command = None;
            } else {
                app.pending_command = Some('c');
            }
        }

        KeyCode::Char('d') => {
            if app.pending_command == Some('d') {
                actions::delete_inbox_item(app);
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
