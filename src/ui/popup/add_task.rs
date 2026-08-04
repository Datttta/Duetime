use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, Popup, SelectedInput, TaskDestination},
    ui::widgets::input,
};
use crate::tasks::TaskInfo;
use crate::keys_help;
use crate::models::TaskTemplate;
use crate::vim_text::InputResult;
use crate::suggestions;

use ratatui::{
    layout::{Constraint, Flex, Layout, Rect, Alignment},
    widgets::{Block, Clear, Paragraph, Padding},
    Frame,
};

const TASK_NAME_WIDTH: u16 = 27;
const PLAN_START_WIDTH: u16 = 30;
const PLAN_END_WIDTH: u16 = 28;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = centered_rect(frame);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Add Task")
        .padding(Padding::new(1,1,0,0));

    let inner = block.inner(area);

    frame.render_widget(block, area);

    fn centered_rect(frame: &Frame) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(6), // add_task box height
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(94), // task_box Length
        ])
        .flex(Flex::Center)
        .split(vertical[0]);

        horizontal[0]
    }

    let vertical = Layout::vertical([
        Constraint::Length(3), // input height
        Constraint::Length(1), // keys_help
    ])
    .split(inner);

    let tasks_colums = Layout::horizontal([
        Constraint::Length(TASK_NAME_WIDTH), // task name chunk
        Constraint::Length(2),
        Constraint::Length(PLAN_START_WIDTH), // planned start chunk
        Constraint::Length(3), 
        Constraint::Length(PLAN_END_WIDTH), // planned end chunk
    ])
    .flex(Flex::Start)
    .split(vertical[0]);

    let keys_help = Paragraph::new(keys_help::keys(app))
            .alignment(Alignment::Center);
    frame.render_widget(keys_help, vertical[1]);

    let separator = Paragraph::new("-")
        .alignment(Alignment::Center)
        .block(Block::default().padding(Padding::top(1)));

    let suggestions_area = Rect {
        x: tasks_colums[0].x,
        y: tasks_colums[0].bottom(),
        width: TASK_NAME_WIDTH,
        height: app.suggestions.len().min(5) as u16,
    };

    let task_name_suggestions = suggestions::task_name_list(
        &app.known_tasks,
        &app.presets,
        &app.task_name.text,
        app.selected_suggestion,
    );

    frame.render_widget(task_name_suggestions, suggestions_area);

    input::draw(
        frame,
        tasks_colums[0],
        &app.task_name,
        "Task name",
        app.selected_input == SelectedInput::TaskName,
        app.mode,
    );

    input::draw(
        frame,
        tasks_colums[2],
        &app.planned_start,
        "planned start (e.g. 14:00)",
        app.selected_input == SelectedInput::PlannedStart,
        app.mode,
    );

    frame.render_widget(separator, tasks_colums[3]);

    input::draw(
        frame,
        tasks_colums[4],
        &app.planned_end,
        "planned end (e.g. 15:00)",
        app.selected_input == SelectedInput::PlannedEnd,
        app.mode,
    );
}

fn update_suggestions(app: &mut App) {
    app.suggestions = suggestions::task_name_suggestions(
        &app.known_tasks,
        &app.presets,
        &app.task_name.text,
    );

    app.selected_suggestion = 0;
}

fn handle_suggestion_keys(app: &mut App, key: KeyEvent) -> bool {
    if app.selected_input != SelectedInput::TaskName {
        return false;
    }

    if app.suggestions.is_empty() {
        return false;
    }

    match key.code {
        KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if app.selected_suggestion + 1 < app.suggestions.len() {
                app.selected_suggestion += 1;
            }
            true
        }

        KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if app.selected_suggestion > 0 {
                app.selected_suggestion -= 1;
            }
            true
        }

        KeyCode::Tab => {
            if let Some(name) = app.suggestions.get(app.selected_suggestion) {
                app.task_name.text = name.clone();
                app.task_name.cursor = app.task_name.text.chars().count();
                app.suggestions.clear();
            }
            true
        }

        _ => false,
    }    
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    if handle_suggestion_keys(app, key) {
        return;
    }

    let result = match app.selected_input {
        SelectedInput::TaskName => {
            let max_chars = (TASK_NAME_WIDTH - 5) as usize;
            app.task_name.handle_key(key, &mut app.mode, max_chars)
        }

        SelectedInput::PlannedStart => {
            let max_chars = (PLAN_START_WIDTH - 5) as usize;
            app.planned_start.handle_key(key, &mut app.mode, max_chars)
        }

        SelectedInput::PlannedEnd => {
            let max_chars = (PLAN_END_WIDTH - 5) as usize;
            app.planned_end.handle_key(key, &mut app.mode, max_chars)
        }
    };

    match result {
        InputResult::TextChanged => {
            if app.selected_input == SelectedInput::TaskName {
                update_suggestions(app);
            }
            return;
        }

        InputResult::Consumed => return,
        InputResult::Ignored => {}
    }

    match key.code {
        KeyCode::Enter => {
            save_task(app);
        }

        KeyCode::Esc => {
            close_popup(app);
        }

        KeyCode::Tab => {
            app.selected_input = match app.selected_input {
                SelectedInput::TaskName => SelectedInput::PlannedStart,
                SelectedInput::PlannedStart => SelectedInput::PlannedEnd,
                SelectedInput::PlannedEnd => SelectedInput::TaskName
            }
        }

        KeyCode::BackTab => {
            app.selected_input = match app.selected_input {
                SelectedInput::TaskName => SelectedInput::PlannedEnd,
                SelectedInput::PlannedEnd => SelectedInput::PlannedStart,
                SelectedInput::PlannedStart => SelectedInput::TaskName
            }
        }

        _ => {}
    }
}

fn close_popup(app: &mut App) {
    match app.task_destination {
        TaskDestination::AddTask | TaskDestination::EditTask(_) => {
            app.popup = Popup::None;
        }

        TaskDestination::Preset | TaskDestination::EditPresetTask(_) => {
            app.popup = Popup::NewPreset;
        }
    }
}

fn save_task(app: &mut App) {
    match app.task_destination {

        TaskDestination::Preset => {
            let id = app.next_id;
            app.next_id += 1;

            app.preset_tasks.push(TaskTemplate {
                id,
                name: app.task_name.text.clone(),
                planned_start: Some(app.planned_start.text.clone()),
                planned_end: Some(app.planned_end.text.clone()),
            });

            if app.preset_task_state.selected().is_none() && !app.preset_tasks.is_empty() {
                app.preset_task_state.select(Some(0));
            }

            app.popup = Popup::NewPreset;
        }

        TaskDestination::AddTask => {
            app.tasks.push(TaskInfo {
                name: app.task_name.text.clone(),
                status: "PENDING".into(),
                planned_start: app.planned_start.text.clone(),
                planned_end: app.planned_end.text.clone(),
                ..Default::default()
            });

            if app.table_state.selected().is_none() && !app.tasks.is_empty() {
                app.table_state.select(Some(0));
            }

        }

        TaskDestination::EditTask(index) => {
            if let Some(task) = app.tasks.get_mut(index) {
                task.name = app.task_name.text.clone();
                task.planned_start = app.planned_start.text.clone();
                task.planned_end = app.planned_end.text.clone();
                app.popup = Popup::None;
            }
        }

        TaskDestination::EditPresetTask(index) => {
            if let Some(task) = app.preset_tasks.get_mut(index) {
                task.name = app.task_name.text.clone();
                task.planned_start = Some(app.planned_start.text.clone());
                task.planned_end = Some(app.planned_end.text.clone());
                app.popup = Popup::NewPreset;
            }
        }
    }

    app.task_name.clear();
    app.planned_start.clear();
    app.planned_end.clear();
    
    app.suggestions.clear();
    app.selected_suggestion = 0;
}
