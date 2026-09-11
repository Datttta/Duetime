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
use log::info;

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


pub fn start_stop(app: &mut App) {
    if let Some(index) = app.timers_list_state.selected() {
        let timer = &mut app.timers[index];
        info!(" is running? {:?}", timer.countdown.running());
        info!("status: {:?}", timer.status);

        if timer.countdown.running() {
            timer.countdown.pause();
            info!(" is running2? {:?}", timer.countdown.running());
            timer.status = "PAUSED".into();
            info!("status2: {:?}", timer.status);
        } else {
            timer.countdown.start();
            timer.status = "".into();
        }
    }
}

// nagivation
const COLUMNS: usize = 3;

pub fn move_left(app: &mut App) {
    if let Some(current) = app.timers_list_state.selected() {
        let prev = current.saturating_sub(1);
        app.timers_list_state.select(Some(prev));
    }
}

pub fn move_right(app: &mut App) {
    if let Some(current) = app.timers_list_state.selected() {
        if !app.timers.is_empty() {
            let next = (current + 1).min(app.timers.len() - 1);
            app.timers_list_state.select(Some(next));
        }
    }
}

pub fn move_down(app: &mut App) {
    if let Some(current) = app.timers_list_state.selected() {
        if app.timers.len() == 4 && current == 1 {
            app.timers_list_state.select(Some(3));
            return
        }
        
        if app.timers.len() == 5 && current == 2 {
            app.timers_list_state.select(Some(4));
            return
        }

        if app.timers.len() == 5 && current == 1 {
            app.timers_list_state.select(Some(3));
            return
        }

        if current > 2 {
            return
        }

        if !app.timers.is_empty() {
            let target = current + COLUMNS;
            
            // If moving down stays within valid array bounds, select it
            if target < app.timers.len() {
                app.timers_list_state.select(Some(target));
            } else {
                // Optional: Clamp to the last item if target exceeds length
                app.timers_list_state.select(Some(app.timers.len() - 1));
            }
        }
    }
}

pub fn move_up(app: &mut App) {
    if let Some(current) = app.timers_list_state.selected() {
        if app.timers.len() == 4 && current == 3 {
            app.timers_list_state.select(Some(1));
            return
        }

        if app.timers.len() == 5 && current == 4 {
            app.timers_list_state.select(Some(2));
            return
        }
        // Subtract 3 if possible; otherwise clamp to row 0 column position
        if current >= COLUMNS {
            app.timers_list_state.select(Some(current - COLUMNS));
        }
    }
}
