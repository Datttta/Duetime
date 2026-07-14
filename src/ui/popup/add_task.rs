use crate::app::App;

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

    let text = Paragraph::new("Task name:");
    frame.render_widget(text, inner);
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
