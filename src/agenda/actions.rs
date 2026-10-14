use log::info;
use chrono::{Local, Duration};

use crate::{
    app::{
    App,
    Popup, 
    AgendaPopup, 
    AgendaSelectedInput,
    },

    input::{
        vim_navigation::NavigationMode,
        vim_text::InputMode,
    },
    agenda::ui::get_selected_global_index,
    storage, search,
};

pub fn add_event(app: &mut App) {
    app.event_name.clear();

    app.event_time.cursor = 0;
    app.event_time.value = "--:--".to_string();
    
    app.event_date.cursor = 0;
    app.event_date.value = Local::now().date_naive().format("%d-%m-%y").to_string();

    app.event_repeat = false;

    app.event_name.cursor = app.event_name.text.len();

    app.mode = InputMode::Insert;
    app.agenda_selected_input = AgendaSelectedInput::Name;
    app.popup = Popup::Agenda(AgendaPopup::AddEvent);
}

pub fn edit_event(app: &mut App) {
    let selected = if app.popup == Popup::None {
        get_selected_global_index(app)
    } else {
        app.all_events_table_state.selected()
    };

    info!("Selected edit: {:?}", selected);

    if let Some(index) = selected {
        let event = &app.events[index];

        info!("selected event name: {:?}", event.name.clone());
        // Load event data into inputs
        app.event_name.text = event.name.clone();

        app.event_date.value =
            event.date.format("%d-%m-%y").to_string();

        app.event_time.value = event
            .time
            .map(|time| time.format("%H:%M").to_string())
            .unwrap_or_else(|| "--:--".to_string());

        app.event_repeat = event.repeat;

        // Reset cursors
        app.event_name.cursor = 0;
        app.event_date.cursor = 0;
        app.event_time.cursor = 0;

        app.mode = InputMode::Normal;
        app.popup = Popup::Agenda(AgendaPopup::EditEvent);
        app.agenda_selected_input = AgendaSelectedInput::Name;

        app.pending_command = None;
    }
}

pub fn all_events(app: &mut App) {
    app.popup = Popup::Agenda(AgendaPopup::AllEvents);
}

pub fn event_info(app: &mut App) {
    app.popup = Popup::Agenda(AgendaPopup::EventInfo);
}

pub fn search_agenda(app: &mut App) {
    let today = chrono::Local::now().date_naive();
    let max_date = today + chrono::Duration::days(30);

    let visible_indices: Vec<usize> = app
        .events
        .iter()
        .enumerate()
        .filter(|(_, event)| {
            event.date >= today && event.date <= max_date
        })
        .map(|(index, _)| index)
        .collect();

    let query = app.agenda_search.query.trim().to_lowercase();

    app.agenda_search.matches.clear();
    app.agenda_search.current_match = 0;

    if query.is_empty() {
        return;
    }

    app.agenda_search.matches = visible_indices
        .into_iter()
        .filter(|&event_index| {
            app.events[event_index]
                .name
                .to_lowercase()
                .contains(&query)
        })
        .collect();

    if let Some(&event_index) = app.agenda_search.matches.first() {
        select_agenda_event(app, event_index);
    }
}

pub fn select_agenda_event(app: &mut App, event_index: usize) {
    let today = chrono::Local::now().date_naive();

    let today_indices: Vec<usize> = app
        .events
        .iter()
        .enumerate()
        .filter(|(_, event)| event.date == today)
        .map(|(index, _)| index)
        .collect();

    let upcoming_indices: Vec<usize> = app
        .events
        .iter()
        .enumerate()
        .filter(|(_, event)| {
            event.date > today
                && event.date <= today + chrono::Duration::days(30)
        })
        .map(|(index, _)| index)
        .collect();

    if let Some(position) = today_indices
        .iter()
        .position(|&index| index == event_index)
    {
        // Row 0 is "Today"
        app.agenda_table_state.select(Some(position + 1));
    } else if let Some(position) = upcoming_indices
        .iter()
        .position(|&index| index == event_index)
    {
        // Today events + Today header + spacer + Upcoming header
        let row = today_indices.len() + 3 + position;

        app.agenda_table_state.select(Some(row));
    }
}

pub fn search_all_events(app: &mut App) {
    search::search_items(
        &mut app.agenda_search,
        &app.events,
        |event, query| {
            event.name.to_lowercase().contains(query)
        },
    );

    if let Some(&index) = app.agenda_search.matches.first() {
        app.all_events_table_state.select(Some(index));
    }
}

// Maps any agenda table row index back to its global event index in app.events
pub fn get_global_index_for_row(app: &App, row_idx: usize) -> Option<usize> {
    let today = Local::now().date_naive();
    let max_date = today + Duration::days(30);

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

    if row_idx >= 1 && row_idx <= today_count {
        Some(today_indices[row_idx - 1])
    } else if row_idx > today_count + 2 {
        let upcoming_idx = row_idx - (today_count + 3);
        Some(upcoming_indices.get(upcoming_idx).copied()?)
    } else {
        None
    }
}

pub fn delete_event(app: &mut App) {
    if app.popup == Popup::None {
        let current_row = match app.agenda_table_state.selected() {
            Some(r) => r,
            None => {
                app.pending_command = None;
                return;
            }
        };

        // Get table row range (works for both single selection and visual mode)
        let (min_row, max_row) = if app.n_mode == NavigationMode::Visual {
            let start = app.n_visual_start.unwrap_or(current_row);
            (start.min(current_row), start.max(current_row))
        } else {
            (current_row, current_row)
        };

        // Collect exact global event indices for the selected table rows
        let mut global_indices_to_delete = Vec::new();
        for r in min_row..=max_row {
            if let Some(g_idx) = get_global_index_for_row(app, r) {
                global_indices_to_delete.push(g_idx);
            }
        }

        // Sort descending so removing higher indices doesn't shift lower indices
        global_indices_to_delete.sort_unstable();
        global_indices_to_delete.dedup();

        for &g_idx in global_indices_to_delete.iter().rev() {
            if g_idx < app.events.len() {
                app.events.remove(g_idx);
            }
        }

        // Recalculate table layout and snap cursor to a valid visible row
        let today = Local::now().date_naive();
        let max_date = today + Duration::days(30);

        let visible_indices: Vec<usize> = (0..app.events.len())
            .filter(|&i| {
                let date = app.events[i].date;
                date >= today && date <= max_date
            })
            .collect();

        if visible_indices.is_empty() {
            app.agenda_table_state.select(None);
        } else {
            let (today_indices, upcoming_indices): (Vec<usize>, Vec<usize>) =
                visible_indices
                    .into_iter()
                    .partition(|&i| app.events[i].date == today);

            let today_count = today_indices.len();
            let total_rows = 1 + today_count + 1 + 1 + upcoming_indices.len();

            let target = min_row.min(total_rows.saturating_sub(1));
            let mut selected_row = None;

            // Search downwards from target for a valid selectable event row
            for r in target..total_rows {
                if get_global_index_for_row(app, r).is_some() {
                    selected_row = Some(r);
                    break;
                }
            }
            // Search upwards from target if nothing found below
            if selected_row.is_none() {
                let mut r = target;
                while r > 0 {
                    r -= 1;
                    if get_global_index_for_row(app, r).is_some() {
                        selected_row = Some(r);
                        break;
                    }
                }
            }
            // Fallback to first valid row
            if selected_row.is_none() {
                for r in 0..total_rows {
                    if get_global_index_for_row(app, r).is_some() {
                        selected_row = Some(r);
                        break;
                    }
                }
            }

            app.agenda_table_state.select(selected_row);
        }

        app.n_mode = NavigationMode::Normal;
        app.n_visual_start = None;

        let _ = storage::agenda::save_agenda(&app.events);
    } else {
        // Popup deletion logic (All Events popup)
        if let Some(current_row) = app.all_events_table_state.selected() {
            let (min_idx, max_idx) = if app.n_mode == NavigationMode::Visual {
                let start = app.n_visual_start.unwrap_or(current_row);
                (start.min(current_row), start.max(current_row))
            } else {
                (current_row, current_row)
            };

            let start_drain = min_idx.min(app.events.len());
            let end_drain = (max_idx + 1).min(app.events.len());
            
            if start_drain < end_drain {
                app.events.drain(start_drain..end_drain);
            }

            if app.events.is_empty() {
                app.all_events_table_state.select(None);
            } else {
                let new_selected = min_idx.min(app.events.len() - 1);
                app.all_events_table_state.select(Some(new_selected));
            }

            app.n_mode = NavigationMode::Normal;
            app.n_visual_start = None;

            let _ = storage::agenda::save_agenda(&app.events);
        }
    }

    app.pending_command = None;
}
