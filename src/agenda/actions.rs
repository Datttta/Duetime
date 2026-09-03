use chrono::Local;

use super::ui::AgendaEvent;

pub fn remove_expired_events(events: &mut Vec<AgendaEvent>) {
    let today = Local::now().date_naive();

    events.retain(|event| {
        event.repeat || event.date >= today
    });
}
