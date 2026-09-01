use crate::{
    ui::{
        widgets::input::ellipsize,
        theme::{task_selection_color, unfocused_panel},
    },
    app::{App, Popup, Panel, Priority},
    navigation::vim_navigation::NavigationMode,
};

use ratatui::{
    layout::{Constraint, Rect, Layout, Flex},
    widgets::{Row, Table, Cell, Paragraph, Padding, Block},
    style::{Style, Color},
    text::Line,
    Frame,
};

use serde::{Deserialize, Serialize};

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

    let is_visual = app.focused_panel == Panel::Agenda
        && app.n_mode == NavigationMode::Visual;

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
