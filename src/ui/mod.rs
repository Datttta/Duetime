mod header;
mod popup;
mod tasks;

use crate::app::{App, Popup};

use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Padding},
    Frame,
};

struct MainLayout {
    header: Rect,
    tasks: Rect,
}

fn draw_layout(frame: &mut Frame) -> MainLayout {
    let border = Block::bordered()
        .title(" Duetime ")
        .padding(Padding::new(2, 4, 1, 1));

    let inner = border.inner(frame.area());

    frame.render_widget(border, frame.area());

    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
    ])
    .split(inner);

    MainLayout {
        header: chunks[0],
        tasks: chunks[2],
    }
}

pub fn draw(frame: &mut Frame, app: &App) {
    let layout = draw_layout(frame);

    header::draw(frame, layout.header);
    tasks::draw(frame, layout.tasks, app);

    if let Popup::AddTask = app.popup {
        popup::add_task::draw(frame, app);
    }
}
