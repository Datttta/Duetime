
use crate::{
    app::App,
    navigation::vim_navigation,
};

use super::actions;

use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    let mut selected = app.agenda_table_state.selected();

    let handled = vim_navigation::handle(
        key,
        &mut app.pending_command,
        &mut selected,
        app.events.len(),
        &mut app.n_mode,
        &mut app.n_visual_start,
    );

    app.agenda_table_state.select(selected);

    if handled {
        return;
    }

    match key.code {
        _ => {}
    }
}
