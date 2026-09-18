use crate::{
    app::App,
    navigation::vim_navigation,
};

use super::actions;
use super::ui::snap_agenda_selection; // Import your snapper helper from ui.rs

use crossterm::event::{KeyCode, KeyEvent};
use chrono::Duration;

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    let mut selected = app.agenda_table_state.selected();

    let today = chrono::Local::now().date_naive();
    let max_date = today + Duration::days(30);

    // Filter visible events just like ui.rs does
    let visible_indices: Vec<usize> = (0..app.events.len())
        .filter(|&i| {
            let date = app.events[i].date;
            date >= today && date <= max_date
        })
        .collect();

    let (today_indices, upcoming_indices): (Vec<usize>, Vec<usize>) =
        visible_indices
            .into_iter()
            .partition(|&i| app.events[i].date == today);

    let today_count = today_indices.len();
    let upcoming_count = upcoming_indices.len();

    // Total table rows = Today header (1) + today events + spacer (1) + Upcoming header (1) + upcoming events
    let total_rows = today_count + upcoming_count + 3;

    let handled = vim_navigation::handle(
        key,
        &mut app.pending_command,
        &mut selected,
        total_rows, // Pass total table rows, not just event count!
        &mut app.n_mode,
        &mut app.n_visual_start,
    );

    app.agenda_table_state.select(selected);

    if handled {
        snap_agenda_selection(app, total_rows, today_count, key);
    }

    match key.code {
        KeyCode::Char('a') => {
            actions::add_event(app);
        }

        KeyCode::Char('e') => {
            actions::edit_event(app);
        }
        
        KeyCode::Char('i') => {
            app.last_popup = app.popup.clone();
            actions::event_info(app);
        }
        
        KeyCode::Char('l') => {
            actions::all_events(app);
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
