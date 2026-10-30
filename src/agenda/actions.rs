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
            event.date <= max_date
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
        .filter(|(_, event)| event.date <= today)
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

pub use crate::agenda::ui::get_global_index_for_row;

pub fn delete_event(app: &mut App) {
    if app.popup == Popup::None {
        let Some(current_row) = app.agenda_table_state.selected() else {
            return reset_and_clear(app);
        };

        let (min_row, max_row) = get_selection_range(app, current_row);

        // Collect valid global indices, sort descending, and remove
        let mut indices: Vec<usize> = (min_row..=max_row)
            .filter_map(|r| get_global_index_for_row(app, r))
            .collect();
        
        indices.sort_unstable();
        indices.dedup();
        indices.iter().rev().for_each(|&idx| {
            app.events.remove(idx);
        });

        // Recalculate total table rows and snap cursor to nearest valid row
        let total_rows = calculate_agenda_total_rows(app);
        let target = min_row.min(total_rows.saturating_sub(1));

        let new_selection = (target..total_rows)
            .find(|&r| get_global_index_for_row(app, r).is_some())
            .or_else(|| (0..target).rev().find(|&r| get_global_index_for_row(app, r).is_some()));

        app.agenda_table_state.select(new_selection);
    } else {
        // Popup deletion logic (All Events popup)
        if let Some(current_row) = app.all_events_table_state.selected() {
            let (min_row, max_row) = get_selection_range(app, current_row);
            let end_drain = (max_row + 1).min(app.events.len());
            
            if min_row < end_drain {
                app.events.drain(min_row..end_drain);
            }

            let new_sel = if app.events.is_empty() {
                None
            } else {
                Some(min_row.min(app.events.len() - 1))
            };
            app.all_events_table_state.select(new_sel);
        }
    }

    reset_and_clear(app);
    let _ = storage::agenda::save_agenda(&app.events);
}

// Helper: Gets min/max range for normal or Visual mode
fn get_selection_range(app: &App, current: usize) -> (usize, usize) {
    if app.n_mode == NavigationMode::Visual {
        let start = app.n_visual_start.unwrap_or(current);
        (start.min(current), start.max(current))
    } else {
        (current, current)
    }
}

// Helper: Computes total rows in the main agenda table
fn calculate_agenda_total_rows(app: &App) -> usize {
    let today = Local::now().date_naive();
    let max_date = today + Duration::days(30);

    let visible_count = app.events.iter()
        .filter(|e| e.date <= max_date)
        .count();
    
    let today_count = app.events.iter()
        .filter(|e| e.date <= today && e.date <= max_date)
        .count();

    let upcoming_count = visible_count - today_count;
    
    // Today header + today events + spacer + upcoming header + upcoming events
    today_count + upcoming_count + 3
}

// Helper: Resets navigation mode and pending command
fn reset_and_clear(app: &mut App) {
    app.n_mode = NavigationMode::Normal;
    app.n_visual_start = None;
    app.pending_command = None;
}
