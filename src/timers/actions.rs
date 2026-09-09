use crate::{
    app::{
    App,
    Popup, 
    TimersPopup, 
    TimerSelectedInput,
    },

    navigation::vim_navigation::NavigationMode,
    vim_text::InputMode,
    storage,
};

use chrono::Local;

pub fn add_timer(app: &mut App) {
    app.timer_name.clear();

    app.timer_duration.cursor = 0;
    app.timer_duration.value = "00:00:00".to_string();
    
    app.timer_name.cursor = app.timer_name.text.len();

    app.mode = InputMode::Insert;
    app.timer_selected_input = TimerSelectedInput::Name;
    app.popup = Popup::Timers(TimersPopup::AddTimer);
}

//pub fn delete_event(app: &mut App) {
//    if let Some(current) = app.timer_list_state.selected() {
//        let (first, last) = if app.n_mode == NavigationMode::Visual {
//            if let Some(start) = app.n_visual_start {
//                (start.min(current), start.max(current))
//            } else {
//                (current, current)
//            }
//        } else {
//            (current, current)
//        };
//
//        app.events.drain(first..=last);
//
//        if app.events.is_empty() {
//            app.timer_list_state.select(None);
//        } else {
//            let new_index = first.min(app.events.len() - 1);
//            app.timer_list_state.select(Some(new_index));
//        }
//
//        app.n_mode = NavigationMode::Normal;
//        app.n_visual_start = None;
//    }
//
//    app.pending_command = None;
//}

