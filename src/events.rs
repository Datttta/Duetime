use crate::{
    app::{App, Popup, TaskDestination, NewPresetFocus, SelectedInput, Panel, TasksPopup, InboxPopup, InboxSelectedFeature},
    ui::popup,
    vim_navigation::NavigationMode,
    vim_text::InputMode,
    move_items::MoveTarget,
    models::TaskTemplate,
    vim_navigation,storage_current_tasks, storage_inbox, move_items,
};

use std::io;
use std::time::SystemTime;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

pub fn handle_events(app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {

            // Panel switching only when no popup is open
            if matches!(app.popup, Popup::None) {
                match key.code {
                    KeyCode::Char('L') => {
                        app.focused_panel = match app.focused_panel {
                            Panel::Tasks => Panel::Inbox,
                            Panel::Inbox => Panel::Inbox,
                        };
                        return Ok(());
                    }

                    KeyCode::Char('H') => {
                        app.focused_panel = match app.focused_panel {
                            Panel::Inbox => Panel::Tasks,
                            Panel::Tasks => Panel::Tasks,
                        };
                        return Ok(());
                    }

                    _ => {}
                }
            }

            // Popup gets priority over focused panel
            match &app.popup {
                Popup::None => {
                    match app.focused_panel {
                        Panel::Tasks => handle_tasks_keys(app, key),
                        Panel::Inbox => handle_inbox_keys(app, key),
                    }
                }

                // TASKS POPUPS

                Popup::Tasks(TasksPopup::AddTask) => {
                    popup::add_task::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::EditTask) => {
                    popup::add_task::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::Presets) => {
                    popup::presets::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::NewPreset) => {
                    popup::new_preset::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::KnownTasks) => {
                    popup::known_tasks::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::AddKnownTask) => {
                    popup::known_tasks_add::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::EditKnownTask(_)) => {
                    popup::known_tasks_add::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::TaskInfo) => {
                    popup::task_info::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::Help) => {
                    popup::help::handle_keys(app, key);
                }

                // INBOX POPUPS

                Popup::Inbox(InboxPopup::AddInboxItem) => {
                    popup::add_inbox_item::handle_keys(app, key);
                }

                Popup::Inbox(InboxPopup::EditInboxItem) => {
                    popup::add_inbox_item::handle_keys(app, key);
                }
            }
        }
    }

    Ok(())
}

// =======================================================
// #####################    TASKS   ######################      
// =======================================================

// ========================== popups ====================

fn edit_task(app: &mut App) {
    if let Some(index) = app.table_state.selected() {
        let task = &app.tasks[index];

        app.task_destination = TaskDestination::EditTask(index);

        // Load task data into inputs
        app.task_name.text = task.name.clone();
        app.planned_start.text = task.planned_start.clone();
        app.planned_end.text = task.planned_end.clone();

        app.task_name.cursor = app.task_name.text.len();
        app.planned_start.cursor = app.planned_start.text.len();
        app.planned_end.cursor = app.planned_end.text.len();

        app.mode = InputMode::Normal;
        app.popup = Popup::Tasks(TasksPopup::EditTask);
        app.selected_input = SelectedInput::TaskName;

        app.pending_command = None;
    }
}

fn edit_inbox_item(app: &mut App) {
    if let Some(index) = app.inbox_table_state.selected() {
        let item = &app.inbox_items[index];

        // Load task data into inputs
        app.inbox_item.text = item.input.clone();
        app.inbox_item.cursor = app.inbox_item.text.len();

        app.mode = InputMode::Normal;
        app.popup = Popup::Inbox(InboxPopup::EditInboxItem);
        app.inbox_selected_feature = InboxSelectedFeature::InboxItemInput;

        app.pending_command = None;
    }
}

fn add_tasks_to_preset(app: &mut App) {
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
    app.popup = Popup::Tasks(TasksPopup::NewPreset);

    app.pending_command = None;
}

fn open_presets_popup(app: &mut App) {
    app.popup = Popup::Tasks(TasksPopup::Presets);
}

fn task_info(app: &mut App) {
    if app.table_state.selected().is_some() {
        app.popup = Popup::Tasks(TasksPopup::TaskInfo);
    }
}

fn open_known_tasks(app: &mut App) {
    app.popup = Popup::Tasks(TasksPopup::KnownTasks);
}

fn open_help_popup(app: &mut App) {
    app.popup = Popup::Tasks(TasksPopup::Help);
}

// =======================================================
// ######################   INBOX   ######################      
// =======================================================

fn add_inbox_item_popup(app: &mut App) {
    app.inbox_item.clear();
    app.inbox_selected_feature = InboxSelectedFeature::InboxItemInput;
    app.popup = Popup::Inbox(InboxPopup::AddInboxItem);
}

// ============================ actions =======================
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

    app.pending_command = None;
}

fn start_stop(app: &mut App) {
    if let Some(index) = app.table_state.selected() {
        let task = &mut app.tasks[index];

        if task.stopwatch.running() {
            task.stopwatch.stop();
            task.status = "STOPPED".into();
        } else {
            task.stopwatch.start();
            if task.actual_start.is_none() {
                task.actual_start = Some(SystemTime::now());
            }

            task.status = "IN PROGRESS".into();
        }
    }
}

fn complete_task(app: &mut App) {
    if let Some(index) = app.table_state.selected() {
        let task = &mut app.tasks[index];

        task.stopwatch.stop();
        task.actual_end = Some(SystemTime::now());
        task.status = "COMPLETED".into();

        storage_current_tasks::save_current_tasks(&app.tasks).unwrap();
    }
}

fn reset_task(app: &mut App) {
    if let Some(index) = app.table_state.selected() {
        let task = &mut app.tasks[index];

        task.stopwatch.reset();
        task.actual_start = None;
        task.actual_end = None;
        task.status = "PENDING".into();
        
        storage_current_tasks::save_current_tasks(&app.tasks).unwrap();
    }
}

fn hard_reset_task(app: &mut App) {
    if let Some(index) = app.table_state.selected() {
        let task = &mut app.tasks[index];

        task.stopwatch.reset();
        task.actual_start = None;
        task.actual_end = None;
        task.planned_start = "".to_string();
        task.planned_end = "".to_string();
        task.status = "PENDING".into();
        
        storage_current_tasks::save_current_tasks(&app.tasks).unwrap();
    }
}

fn move_tasks(app: &mut App) {
    if app.n_mode == NavigationMode::Visual {
        move_items::start(
            &mut app.move_state,
            app.table_state.selected(),
            app.n_visual_start,
            app.tasks.len(),
            MoveTarget::Tasks,
        );
    }
}

fn quit(app: &mut App) {
    storage_current_tasks::save_current_tasks(&app.tasks).unwrap();
    app.running = false;
}


// ========= INBOX ================
fn delete_inbox_item(app: &mut App) {
    if let Some(current) = app.inbox_table_state.selected() {
        let (first, last) = if app.n_mode == NavigationMode::Visual {
            if let Some(start) = app.n_visual_start {
                (start.min(current), start.max(current))
            } else {
                (current, current)
            }
        } else {
            (current, current)
        };

        app.inbox_items.drain(first..=last);

        if app.inbox_items.is_empty() {
            app.inbox_table_state.select(None);
        } else {
            let new_index = first.min(app.inbox_items.len() - 1);
            app.inbox_table_state.select(Some(new_index));
        }

        app.n_mode = NavigationMode::Normal;
        app.n_visual_start = None;

        storage_inbox::save_inbox(&app.inbox_items).unwrap();
    }
}

//  ====================== handle keys =================================
fn handle_tasks_keys(app: &mut App, key: KeyEvent) {
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
            storage_current_tasks::save_current_tasks(&app.tasks).unwrap();

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
                add_tasks_to_preset(app);
            } 
        }

        KeyCode::Char('t') => {
            app.add_task(TaskDestination::AddTask);
        }

        KeyCode::Char('e') => {
            edit_task(app);
        }

        KeyCode::Char('d') => {
            if app.pending_command == Some('d') {
                delete_task(app);
                app.pending_command = None;
            } else {
                app.pending_command = Some('d')
            }
        }

        KeyCode::Char('x') => {
            move_tasks(app);
            return;
        }

        KeyCode::Char('i') => {
            task_info(app);
        }

        KeyCode::Char('s') => {
            start_stop(app);
        }
        
        KeyCode::Char('c') => {
            complete_task(app);
        }

        KeyCode::Char('r') => {
            reset_task(app);
        }
        
        KeyCode::Char('R') => {
            hard_reset_task(app);
        }

        KeyCode::Char('P') => {
           open_presets_popup(app); 
        }

        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
           open_known_tasks(app); 
        }

        KeyCode::Char('?') => {
            open_help_popup(app);
        }

        KeyCode::Char('q') => {
            quit(app);
        }

        _ => {
            app.pending_command = None;
        }
    }
}

fn handle_inbox_keys(app: &mut App, key: KeyEvent) {
    let mut selected = app.inbox_table_state.selected();

    let handled = vim_navigation::handle(
        key,
        &mut app.pending_command,
        &mut selected,
        app.inbox_items.len(),
        &mut app.n_mode,
        &mut app.n_visual_start,
    );

    app.inbox_table_state.select(selected);

    if handled {
        return;
    }

    match key.code {
        KeyCode::Char('a') => {
            app.pending_command = Some('a');
        }

        KeyCode::Char('p') => {
            if app.pending_command == Some('a') {
                add_inbox_item_popup(app);
            }
            
            app.pending_command = None;
        }

        KeyCode::Char('e') => {
            edit_inbox_item(app);
        }

        KeyCode::Char('d') => {
            if app.pending_command == Some('d') {
                delete_inbox_item(app);
                app.pending_command = None;
            } else {
                app.pending_command = Some('d');
            }
        }
        
        KeyCode::Char('q') => {
            quit(app);
        }

        _ => {
            app.pending_command = None;
        }
    }
}
