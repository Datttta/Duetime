use crate::{
    app::{
    App,
    TaskDestination, 
    },

    storage::{current_tasks},
    navigation::{
        vim_navigation::NavigationMode,
        vim_navigation,
        move_items,
    },
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::actions;

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    if app.move_state.is_moving() {
        let was_moving = app.move_state.is_moving();

        move_items::handle_keys(
            &mut app.move_state,
            &mut app.tasks,
            &mut app.table_state,
            &mut app.pending_command,
            key,
        );

        // Enter or Esc ended the move.
        if was_moving && !app.move_state.is_moving() {
            current_tasks::save_current_tasks(&app.tasks).unwrap();

            app.n_mode = NavigationMode::Normal;
            app.n_visual_start = None;
        }

        return;
    }

    let mut selected = app.table_state.selected();

    let handled = vim_navigation::handle(
        key,
        &mut app.pending_command,
        &mut selected,
        app.tasks.len(),
        &mut app.n_mode,
        &mut app.n_visual_start,
    );

    app.table_state.select(selected);

    if handled {
        return;
    }

    match key.code {
        KeyCode::Char('a') => {
            app.pending_command = Some('a');
        }

        KeyCode::Char('p') => {
            if app.pending_command == Some('a') {
                actions::add_tasks_to_preset(app);
            } 
        }

        KeyCode::Char('t') => {
            app.task_add(TaskDestination::AddTask);
        }

        KeyCode::Char('e') => {
            actions::edit_task(app);
        }

        KeyCode::Char('d') => {
            if app.pending_command == Some('d') {
                actions::delete_task(app);
                app.pending_command = None;
            } else {
                app.pending_command = Some('d')
            }
        }

        KeyCode::Char('x') => {
            actions::move_tasks(app);
            return;
        }

        KeyCode::Char('i') => {
            actions::task_info(app);
        }

        KeyCode::Char('s') => {
            actions::start_stop(app);
        }
        
        KeyCode::Char('c') => {
            actions::complete_task(app);
        }

        KeyCode::Char('r') => {
            actions::reset_task(app);
        }
        
        KeyCode::Char('R') => {
            actions::hard_reset_task(app);
        }

        KeyCode::Char('P') => {
           actions::open_presets_popup(app); 
        }

        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
           actions::open_known_tasks(app); 
        }

        _ => {
            app.pending_command = None;
        }
    }
}
