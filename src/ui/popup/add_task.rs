use crossterm::event::{KeyCode, KeyEvent};
use crate::events::{handle_escape};
use crate::tasks::TaskInfo;

use crate::{
    app::{App, Popup, SelectedInput},
    ui::widgets::input,
};

use ratatui::{
    layout::{Constraint, Flex, Layout, Rect, Alignment},
    widgets::{Block, Clear, Paragraph, Padding},
    Frame,
};

const TASK_NAME_WIDTH: u16 = 26;
const PLAN_START_WIDTH: u16 = 30;
const PLAN_END_WIDTH: u16 = 28;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = centered_rect(frame);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Add Task")
        .padding(Padding::new(1,0,0,0));

    let inner = block.inner(area);

    frame.render_widget(block, area);

    fn centered_rect(frame: &Frame) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(5), // add_task box height
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(93), // task_box Length
        ])
        .flex(Flex::Center)
        .split(vertical[0]);

        horizontal[0]
    }

    let vertical = Layout::vertical([
        Constraint::Length(3), // input height
    ])
    .split(inner);

    let inputs = Layout::horizontal([
        Constraint::Length(TASK_NAME_WIDTH), // task name chunk
        Constraint::Length(2),
        Constraint::Length(PLAN_START_WIDTH), // planned start chunk
        Constraint::Length(3), 
        Constraint::Length(PLAN_END_WIDTH), // planned end chunk
    ])
    .flex(Flex::Start)
    .split(vertical[0]);

    let separator = Paragraph::new("-")
        .alignment(Alignment::Center)
        .block(Block::default().padding(Padding::top(1)));

    input::draw(
        frame,
        inputs[0],
        &app.task_name,
        "Task name",
        app.selected_input == SelectedInput::TaskName,
        app.mode,
    );

    input::draw(
        frame,
        inputs[2],
        &app.planned_start,
        "planned start (e.g. 14:00)",
        app.selected_input == SelectedInput::PlannedStart,
        app.mode,
    );

    frame.render_widget(separator, inputs[3]);

    input::draw(
        frame,
        inputs[4],
        &app.planned_end,
        "planned end (e.g. 15:00)",
        app.selected_input == SelectedInput::PlannedEnd,
        app.mode,
    );
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    match key.code {

        KeyCode::Enter => {
            create_task(app);
        }

        KeyCode::Esc => {
            if handle_escape(app) {
                close(app);
            }
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

        _ => {
            match app.selected_input {
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
            }
        }
    }
}


fn create_task(app: &mut App) {
    app.tasks.push(TaskInfo {
        name: app.task_name.text.clone(),
        status: "PENDING".into(),
        planned_start: app.planned_start.text.clone(),
        planned_end: app.planned_end.text.clone(),
        actual_start: "--:--".into(),
        actual_end: "--:--".into(),
        elapsed: "--:--:--".into(),
    });

    app.task_name.clear();
    app.planned_start.clear();
    app.planned_end.clear();

    app.popup = Popup::None;
}

fn close(app: &mut App) {
    app.task_name.clear();
    app.planned_start.clear();
    app.planned_end.clear();

    app.popup = Popup::None;
}
