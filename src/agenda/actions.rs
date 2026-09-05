use crate::{
    app::{
    App,
    Popup, 
    AgendaPopup, 
    AgendaSelectedInput,
    },

    storage::agenda,
    navigation::vim_navigation::NavigationMode,
    vim_text::InputMode,
    storage,
};

use chrono::Local;

use super::ui::AgendaEvent;

pub fn remove_expired_events(events: &mut Vec<AgendaEvent>) {
    let today = Local::now().date_naive();

    events.retain(|event| {
        event.repeat || event.date >= today
    });
}

pub fn add_event(app: &mut App) {
    app.event.clear();

    app.event_time.cursor = 0;
    app.event_time.value = "--:--".to_string();
    
    app.event_date.cursor = 0;
    app.event_date.value = Local::now().date_naive().format("%d-%m-%y").to_string();

    app.event_repeat = false;

    app.mode = InputMode::Insert;
    app.agenda_selected_input = AgendaSelectedInput::Name;
    app.popup = Popup::Agenda(AgendaPopup::AddEvent);
}

pub fn delete_event(app: &mut App) {
    if let Some(current) = app.agenda_table_state.selected() {
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

        if app.events.is_empty() {
            app.agenda_table_state.select(None);
        } else {
            let new_index = first.min(app.events.len() - 1);
            app.agenda_table_state.select(Some(new_index));
        }

        app.n_mode = NavigationMode::Normal;
        app.n_visual_start = None;

        storage::agenda::save_agenda(&app.events).unwrap();
    }

    app.pending_command = None;
}
