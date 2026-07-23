use crate::app::App;

use ratatui::{
    layout::{Constraint, Rect},
    widgets::{Row, Table},
    style::Style,
    Frame,
};
#[derive(Default)]
pub struct TaskInfo {
    pub name: String,
    pub status: String,
    pub planned_start: String,
    pub planned_end: String,
    pub actual_start: String,
    pub actual_end: String,
    pub elapsed: String,
}

pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    let columns = [
        Constraint::Length(26), // task name
        Constraint::Length(12), // status
        Constraint::Length(13), // planned start
        Constraint::Length(13), // planned end
        Constraint::Length(11), // actual start
        Constraint::Length(9), // actual end
        Constraint::Length(8), // elapsed
    ];

    let example_task = vec![
        Row::new(vec![
            "  My first task",
            "PENDING",
            "14:00",
            "16:00",
            "",
            "",
            "",
        ]),
    ];

    let rows = app.tasks.iter().map(|task| {
        Row::new(vec![
            format!("  {}", task.name),
            task.status.clone(),
            task.planned_start.clone(),
            task.planned_end.clone(),
            task.actual_start.clone(),
            task.actual_end.clone(),
            task.elapsed.clone(),
        ])
    });


    let table = Table::new(rows, columns)
        .row_highlight_style(Style::default().reversed());

    frame.render_stateful_widget(table, area, &mut app.table_state);
}
