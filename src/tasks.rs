use crate::app::App;

use ratatui::{
    layout::{Constraint, Rect},
    widgets::{Row, Table},
    Frame,
};

pub struct TaskInfo {
    pub name: String,
    pub status: String,
    pub planned_start: String,
    pub planned_end: String,
    pub actual_start: String,
    pub actual_end: String,
    pub elapsed: String,
}

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
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
            name.clone(),
            status.clone(),
            planned_start.clone(),
            planned_end.clone(),
            actual_start.clone(),
            actual_end.clone(),
            elapsed.clone(),
        ]),
    ];

    let table = Table::new(rows, widths);

    frame.render_widget(table, area);
}
