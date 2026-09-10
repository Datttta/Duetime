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

pub fn delete_timer(app: &mut App) {
    if let Some(index) = app.timers_list_state.selected() {
        // 1. Check bounds and remove the timer
        if index < app.timers.len() {
            app.timers.remove(index);
        }

        // 2. Adjust selection index after deletion
        if app.timers.is_empty() {
            app.timers_list_state.select(None);
        } else {
            // Clamp the selected index so it stays within valid bounds
            let new_index = index.min(app.timers.len() - 1);
            app.timers_list_state.select(Some(new_index));
        }

        // 3. Reset navigation state
        app.n_mode = NavigationMode::Normal;
        app.n_visual_start = None;
    }

    app.pending_command = None;
}
