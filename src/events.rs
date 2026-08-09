use crate::{
    app::{App, Popup, TaskDestination, NewPresetFocus},
    ui::popup,
    vim_navigation::NavigationMode,
    models::TaskTemplate,
    vim_navigation,
    storage_current_tasks,
};

use std::time::SystemTime;
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

pub fn handle_events(app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {

            match app.popup {
                Popup::None => handle_normal_keys(app, key),

                Popup::AddTask => popup::add_task::handle_keys(app, key),

                Popup::EditTask => popup::add_task::handle_keys(app, key),
                
                Popup::Presets => popup::presets::handle_keys(app, key),
                
                Popup::NewPreset => popup::new_preset::handle_keys(app, key),

                Popup::KnownTasks => popup::known_tasks::handle_keys(app, key),

                Popup::AddKnownTask => popup::known_tasks_add::handle_keys(app, key),

                Popup::EditKnownTask(_) => popup::known_tasks_add::handle_keys(app, key),

                Popup::TaskInfo => popup::task_info::handle_keys(app, key),
            }
        }
    }

    Ok(())
}


fn handle_normal_keys(app: &mut App, key: KeyEvent) {
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
                app.add_tasks_to_preset();
            } else { 
                app.pause_task();
            }
        }

        KeyCode::Char('t') => {
            app.add_task_popup(TaskDestination::AddTask);
        }

        KeyCode::Char('e') => {
            app.edit_task_popup();
        }

        KeyCode::Char('d') => {
            if app.pending_command == Some('d') {
                delete_task(app);
                app.pending_command = None;
            } else {
                app.pending_command = Some('d')
            }
        }

        KeyCode::Char('i') => {
            if app.table_state.selected().is_some() {
                app.popup = Popup::TaskInfo;
            }
        }

        KeyCode::Char('s') => {
            if let Some(index) = app.table_state.selected() {
                let task = &mut app.tasks[index];

                if task.stopwatch.running() {
                    task.stopwatch.stop();
                    task.actual_end = Some(SystemTime::now());
                    task.status = "COMPLETED".into();
                } else {
                    task.stopwatch.start();
                    task.actual_start = Some(SystemTime::now());
                    task.status = "IN PROGRESS".into();
                }

                storage_current_tasks::save_current_tasks(&app.tasks).unwrap();
            }
        }

        KeyCode::Char('r') => {
            if let Some(index) = app.table_state.selected() {
                let task = &mut app.tasks[index];

                task.stopwatch.reset();
                task.actual_start = None;
                task.actual_end = None;
                task.status = "PENDING".into();
                
                storage_current_tasks::save_current_tasks(&app.tasks).unwrap();
            }
        }

        KeyCode::Char('P') => {
            app.popup = Popup::Presets;
        }

        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.popup = Popup::KnownTasks;
        }

        KeyCode::Char('q') => {
            storage_current_tasks::save_current_tasks(&app.tasks).unwrap();

            app.running = false;
        }

        _ => {
            app.pending_command = None;
        }
    }
}

fn delete_task(app: &mut App) {
    if let Some(current) = app.table_state.selected() {
        let (first, last) = if app.n_mode == NavigationMode::Visual {
            if let Some(start) = app.n_visual_start {
                (start.min(current), start.max(current))
            } else {
                (current, current)
            }
        } else {
            (current, current)
        };

        app.tasks.drain(first..=last);

        if app.tasks.is_empty() {
            app.table_state.select(None);
        } else {
            let new_index = first.min(app.tasks.len() - 1);
            app.table_state.select(Some(new_index));
        }

        app.n_mode = NavigationMode::Normal;
        app.n_visual_start = None;

        storage_current_tasks::save_current_tasks(&app.tasks).unwrap();
    }
}
