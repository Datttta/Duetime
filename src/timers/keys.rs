use crate::{
    app::App,
    navigation::vim_navigation,
};

use super::actions;
use crossterm::event::{KeyCode, KeyEvent};
use log::info;

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    let mut selected = app.timers_list_state.selected();

    let handled = vim_navigation::handle(
        key,
        &mut app.pending_command,
        &mut selected,
        app.timers.len(),
        &mut app.n_mode,
        &mut app.n_visual_start,
    );

    app.timers_list_state.select(selected);

    if handled {
        return;
    }

    match key.code {
        KeyCode::Char('a') => {
            actions::add_timer(app);
            info!("Hit a!!")
        }

        //KeyCode::Char('i') => {
        //    actions::inbox_item_info(app);
        //}

        //KeyCode::Char('e') => {
        //    actions::edit_inbox_item(app);
        //}

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

