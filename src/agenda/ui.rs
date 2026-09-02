use crate::{
    ui::{
        theme::{unfocused_panel, task_selection_color},
    },
    app::{App, Panel, Popup},
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
        .padding(Padding::new(2, 0, 1, 0));

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

pub fn draw_events (
    frame: &mut Frame,
    area: Rect, 
    app: &mut App, 
    is_visual: bool
    ) {
    let columns = [
        Constraint::Length(20), // event name
        Constraint::Length(10), // event day
        Constraint::Length(3), // space
        Constraint::Length(7), // countdown
    ];

    let visual_start = app.n_visual_start;
    let visual_mode = is_visual; 
    let current = app.agenda_table_state.selected();

    let popup_open = !matches!(app.popup, Popup::None);

    let highlight_style = if popup_open || app.focused_panel != Panel::Agenda {
        Style::default()
    } else if app.move_state.is_moving() {
        Style::default()
    } else if visual_mode {
        Style::default()
            .fg(Color::Black)
            .bg(Color::White)
    } else {
        Style::default()
            .bg(task_selection_color())
            .fg(Color::Black)
    };

}

