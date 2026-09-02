use crate::{
    ui::{
        theme::{unfocused_panel},
    },
    app::{App, Panel},
};

use ratatui::{
    layout::{Constraint, Rect, Layout},
    widgets::{Paragraph, Padding, Block},
    style::{Style, Color},
    Frame,
};

pub fn draw_agenda_panel(
    frame: &mut Frame,
    area: Rect,
    app: &mut App,
) {
    let border_color = if app.focused_panel == Panel::Agenda {
        Color::White
    } else {
        unfocused_panel()
    };

    let border = Block::bordered()
        .title(" Agenda ")
        .border_style(Style::default().fg(border_color))
        .padding(Padding::new(0, 0, 1, 0));

    let inner = border.inner(area);

    frame.render_widget(border, area);

    let chunks = Layout::vertical([
        Constraint::Length(1), // Today
        Constraint::Min(0),    // Today events
        Constraint::Length(1), // Upcoming
        Constraint::Min(0),    // Upcoming events
    ])
    .split(inner);

    frame.render_widget(Paragraph::new("Today"), chunks[0]);

    frame.render_widget(Paragraph::new("Upcoming"), chunks[2]);
}
