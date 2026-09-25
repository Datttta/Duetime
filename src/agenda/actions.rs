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
    search::SearchNavigationResult,
    vim_text::InputMode,
    storage, search,
};

use crossterm::event::{KeyEvent};

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

pub fn search_inbox(app: &mut App) {
    search::search_items(
        &mut app.agenda_search,
        &app.inbox_items,
        |item, query| {
            item.input.to_lowercase().contains(query)
        },
    );

    if let Some(&index) = app.agenda_search.matches.first() {
        app.agenda_table_state.select(Some(index));
    }
}

pub fn handle_agenda_search_navigation(
    app: &mut App,
    key: KeyEvent,
) {
    match search::handle_search_navigation(
        &mut app.agenda_search,
        key,
    ) {
        SearchNavigationResult::Continue => {
            if let Some(&index) = app
                .agenda_search
                .matches
                .get(app.agenda_search.current_match)
            {
                app.agenda_table_state.select(Some(index));
            }
        }

        SearchNavigationResult::Cancel => {
            app.n_mode = NavigationMode::Normal;
            app.pending_command = None;
        }
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
