use crate::{
    app::{
    App,
    Popup, 
    TaskDestination, 
    NewPresetFocus, 
    TaskSelectedInput, 
    TasksTablePopup, 
    },

    storage::current_tasks,
    vim_text::InputMode,
    models::TaskTemplate,
    navigation::{
        move_items::MoveTarget,
        vim_navigation::NavigationMode,
        move_items,
    },
};

use std::time::SystemTime;

pub fn edit_task(app: &mut App) {
    if let Some(index) = app.tasks_table_state.selected() {
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
        app.popup = Popup::TasksTable(TasksTablePopup::EditTask);
        app.tasks_selected_input = TaskSelectedInput::TaskName;

        app.pending_command = None;
    }
}

pub fn add_tasks_to_preset(app: &mut App) {
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
    app.popup = Popup::TasksTable(TasksTablePopup::NewPreset);

    app.pending_command = None;
}

pub fn open_presets_popup(app: &mut App) {
    app.popup = Popup::TasksTable(TasksTablePopup::Presets);
}

pub fn task_info(app: &mut App) {
    if app.tasks_table_state.selected().is_some() {
        app.popup = Popup::TasksTable(TasksTablePopup::TaskInfo);
    }
}

pub fn open_known_tasks(app: &mut App) {
    app.popup = Popup::TasksTable(TasksTablePopup::KnownTasks);
}

pub fn delete_task(app: &mut App) {
    if let Some(current) = app.tasks_table_state.selected() {
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
            app.tasks_table_state.select(None);
        } else {
            let new_index = first.min(app.tasks.len() - 1);
            app.tasks_table_state.select(Some(new_index));
        }

        app.n_mode = NavigationMode::Normal;
        app.n_visual_start = None;

        current_tasks::save_current_tasks(&app.tasks).unwrap();
    }

    app.pending_command = None;
}

pub fn start_stop(app: &mut App) {
    if let Some(index) = app.tasks_table_state.selected() {
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

pub fn complete_task(app: &mut App) {
    if let Some(index) = app.tasks_table_state.selected() {
        let task = &mut app.tasks[index];

        task.stopwatch.stop();
        task.actual_end = Some(SystemTime::now());
        task.status = "COMPLETED".into();

        current_tasks::save_current_tasks(&app.tasks).unwrap();
    }
}

pub fn reset_task(app: &mut App) {
    if let Some(index) = app.tasks_table_state.selected() {
        let task = &mut app.tasks[index];

        task.stopwatch.reset();
        task.actual_start = None;
        task.actual_end = None;
        task.status = "PENDING".into();
        
        current_tasks::save_current_tasks(&app.tasks).unwrap();
    }
}

pub fn hard_reset_task(app: &mut App) {
    if let Some(index) = app.tasks_table_state.selected() {
        let task = &mut app.tasks[index];

        task.stopwatch.reset();
        task.actual_start = None;
        task.actual_end = None;
        task.planned_start = "".to_string();
        task.planned_end = "".to_string();
        task.status = "PENDING".into();
        
        current_tasks::save_current_tasks(&app.tasks).unwrap();
    }
}

pub fn move_tasks(app: &mut App) {
    if app.n_mode == NavigationMode::Visual {
        move_items::start(
            &mut app.move_state,
            app.tasks_table_state.selected(),
            app.n_visual_start,
            app.tasks.len(),
            MoveTarget::Tasks,
        );
    }
}


