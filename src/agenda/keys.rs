
use crate::{
    app::App,
    navigation::vim_navigation,
};

use super::actions;

use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    let mut selected = app.agenda_table_state.selected();

    // Build the same list of events that are displayed in the agenda.
    let today = chrono::Local::now().date_naive();

    let table_events: Vec<usize> = app
        .events
        .iter()
        .enumerate()
        .filter(|(_, event)| {
            let days = (event.date - today).num_days();

            days <= 30
        })
        .map(|(index, _)| index)
        .collect();

    let len = table_events.len();

    let handled = vim_navigation::handle(
        key,
        &mut app.pending_command,
        &mut selected,
        len,
        &mut app.n_mode,
        &mut app.n_visual_start,
    );

    app.agenda_table_state.select(selected);

    if handled {
        return;
    }

    match key.code {
        KeyCode::Char('a') => {
            actions::add_event(app);
        }

        KeyCode::Char('e') => {
            actions::edit_event(app);
        }

        KeyCode::Char('d') => {
            if app.pending_command == Some('d') {
                actions::delete_event(app);
                app.pending_command = None;
            } else {
                app.pending_command = Some('d')
            }
        }

        _ => {}
    }
}
