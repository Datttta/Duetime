//use log::info;
use chrono::{Local, Duration};

use crate::{
    app::{
    App,
    Popup, 
    AgendaPopup, 
    AgendaSelectedInput,
    },

    navigation::vim_navigation::NavigationMode,
    agenda::ui::get_selected_global_index,
    vim_text::InputMode,
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
    if let Some(index) = get_selected_global_index(app) {
        let event = &app.events[index];

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

pub fn delete_event(app: &mut App) {
    if let Some(current) = get_selected_global_index(app) {
        let (first, last) = if app.n_mode == NavigationMode::Visual {
            if let Some(start) = app.n_visual_start {
                (start.min(current), start.max(current))
            } else {
                (current, current)
            }
        } else {
            (current, current)
        };

        app.events.drain(first..=last);

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
            // Target the global event index that took the place of the deleted block, clamped to the new bounds
            let target_global = first.min(app.events.len().saturating_sub(1));

            let (today_indices, upcoming_indices): (Vec<usize>, Vec<usize>) =
                visible_indices
                    .into_iter()
                    .partition(|&i| app.events[i].date == today);

            let today_count = today_indices.len();

            // Map target global index back to its proper table row index
            if let Some(pos) = today_indices.iter().position(|&g| g == target_global) {
                // Today event row (Header is row 0, so event is pos + 1)
                app.agenda_table_state.select(Some(pos + 1));
            } else if let Some(pos) = upcoming_indices.iter().position(|&g| g == target_global) {
                // Upcoming event row (Account for Today header [1] + today events + spacer [1] + Upcoming header [1])
                app.agenda_table_state.select(Some(today_count + 3 + pos));
            } else {
                // Fallback to the first available selectable row
                app.agenda_table_state.select(Some(1));
            }
        }

        app.n_mode = NavigationMode::Normal;
        app.n_visual_start = None;

        storage::agenda::save_agenda(&app.events).unwrap();
    }

    app.pending_command = None;
}
