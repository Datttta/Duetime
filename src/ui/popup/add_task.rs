use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, Popup, SelectedInput},
    ui::widgets::input,
};

use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = centered_rect(frame);

    frame.render_widget(Clear, area);

    let block = Block::bordered().title("Add Task");
    let inner = block.inner(area);

    frame.render_widget(block, area);

    let vertical = Layout::vertical([
        Constraint::Length(1), // height of chuck for space
        Constraint::Length(3), // input height
    ])
    .split(inner);

    let inputs = Layout::horizontal([
        Constraint::Length(23), // task name chunk
        Constraint::Length(2),
        Constraint::Length(30), // planned start chunk
    ])
    .flex(Flex::Start)
    .split(vertical[1]);


    input::draw(
        frame,
        inputs[0],
        &app.task_name,
        "Task name",
    );


    input::draw(
        frame,
        inputs[2],
        &app.planned_start,
        "planned start (e.g. 14:00)",
    );
}

fn centered_rect(frame: &Frame) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Length(7), // add_task box height
    ])
    .flex(Flex::Center)
    .split(frame.area());

    let horizontal = Layout::horizontal([
        Constraint::Length(80), // task_box Length
    ])
    .flex(Flex::Center)
    .split(vertical[0]);

    horizontal[0]
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    match key.code {

        KeyCode::Enter => {
            create_task(app);
        }

        KeyCode::Esc => {
            close(app);
        }

        KeyCode::Tab => {
            app.selected_input = match app.selected_input {
                SelectedInput::TaskName => SelectedInput::PlannedStart,
                SelectedInput::PlannedStart => SelectedInput::TaskName
            }
        }

        _ => {
            match app.selected_input {
                SelectedInput::TaskName => app.task_name.handle_key(key),
                SelectedInput::PlannedStart => app.planned_start.handle_key(key),
            }
        }
    }
}


fn create_task(app: &mut App) {
    println!("Task created: {}", app.task_name.text);

    app.task_name.clear();

    app.popup = Popup::None;
}

fn close(app: &mut App) {
    app.task_name.clear();

    app.popup = Popup::None;
}
