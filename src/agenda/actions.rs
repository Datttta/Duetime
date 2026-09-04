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

    app.event_date.value = format!("00-00-{}", chrono::Local::now().format("%y"));
    app.event_date.cursor = 0;

    app.event_time.value = "00:00".to_string();
    app.event_time.cursor = 0;

    app.event_repeat = false;

    app.mode = InputMode::Insert;
    app.agenda_selected_input = AgendaSelectedInput::Name;
    app.popup = Popup::Agenda(AgendaPopup::AddEvent);
}
