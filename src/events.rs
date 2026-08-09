use crate::app::{App, Popup, SelectedInput, TaskDestination, NewPresetFocus};
use crate::ui::popup;
use crate::vim_text::InputMode;
use crate::vim_navigation;
use crate::storage_current_tasks;
use crate::vim_navigation::NavigationMode;
use crate::models::TaskTemplate;

use std::time::SystemTime;
use std::thread::current;
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
                app.preset_tasks = app.tasks
                    .iter()
                    .map(|task| TaskTemplate {
                        id: app.next_id,
                        name: task.name.clone(),
                        planned_start: Some(task.planned_start.clone()),
                        planned_end: Some(task.planned_end.clone()),
                    })
                    .collect();

                app.next_id += app.preset_tasks.len() as u64;

                if !app.preset_tasks.is_empty() {
                    app.preset_task_state.select(Some(0));
                }

                app.preset_name.clear();
                app.new_preset_focus = NewPresetFocus::Name;
                app.popup = Popup::NewPreset;

                app.pending_command = None;
            }
        }

        KeyCode::Char('t') => {
            if app.pending_command == Some('a') {
                app.task_destination = TaskDestination::AddTask;

                app.task_name.clear();
                app.planned_start.clear();
                app.planned_end.clear();

                app.popup = Popup::AddTask;
                app.selected_input = SelectedInput::TaskName;
                app.mode = InputMode::Insert;
            
                storage_current_tasks::save_current_tasks(&app.tasks).unwrap();
            }


            app.pending_command = None;
        }

        KeyCode::Char('e') => {
            if let Some(index) = app.table_state.selected() {
                app.popup = Popup::EditTask;

                app.task_destination = TaskDestination::EditTask(index);

                let task = &app.tasks[index];
                
                //load task data into the inputs
                app.task_name.text = task.name.clone();
                app.planned_start.text = task.planned_start.clone();
                app.planned_end.text = task.planned_end.clone();

                app.task_name.cursor = app.task_name.text.len();
                app.planned_start.cursor = app.planned_start.text.len();
                app.planned_end.cursor = app.planned_end.text.len();

                app.mode = InputMode::Normal;
                app.selected_input = SelectedInput::TaskName;

                storage_current_tasks::save_current_tasks(&app.tasks).unwrap();
            }
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

        KeyCode::Char('p') => {
            if let Some(index) = app.table_state.selected(){
                let task = &mut app.tasks[index];

                if task.stopwatch.running() {
                    task.stopwatch.stop();
                    task.status = "STOPPED".into();
                }

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
