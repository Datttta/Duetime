use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

use crate::{
    app::{App},
};

pub fn draw(
    frame: &mut Frame,
    area: Rect,
    app: &mut App,
    is_visual: bool,
    ) {
    let chunks = Layout::vertical([
        Constraint::Length(1), // Today
        Constraint::Min(0),    // Today events
        Constraint::Length(1), // Upcoming
        Constraint::Min(0),    // Upcoming events
    ])
    .split(area);

    frame.render_widget(Paragraph::new("Today"), chunks[0]);
    frame.render_widget(Paragraph::new("Upcoming"), chunks[2]);
}
