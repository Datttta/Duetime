use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, Popup},
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

    let chunks = Layout::vertical([
        Constraint::Length(0),
        Constraint::Length(3),
    ])
    .split(inner);

    input::draw(
        frame,
        chunks[1],
        &app.task_name,
        "",
    );
}

fn centered_rect(frame: &Frame) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Length(10),
    ])
    .flex(Flex::Center)
    .split(frame.area());

    let horizontal = Layout::horizontal([
        Constraint::Length(70),
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

        _ => {
            app.task_name.handle_key(key);
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
