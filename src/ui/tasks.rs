use crate::app::App;

use ratatui::{
    layout::{Constraint, Rect},
    widgets::{Row, Table},
    Frame,
};

pub fn draw(frame: &mut Frame, area: Rect, _app: &App) {
    let widths = [
        Constraint::Length(22),
        Constraint::Length(12),
        Constraint::Length(13),
        Constraint::Length(13),
        Constraint::Length(11),
        Constraint::Length(9),
        Constraint::Length(8),
    ];

    let rows = vec![
        Row::new(vec![
            "My first task",
            "PENDING",
            "14:00",
            "16:00",
            "--:--",
            "--:--",
            "--:--:--",
        ]),
        Row::new(vec![
            "Second task",
            "DONE",
            "09:00",
            "10:00",
            "09:00",
            "09:45",
            "00:45:00",
        ]),
    ];

    let table = Table::new(rows, widths);

    frame.render_widget(table, area);
}
