use crate::app::App;

use ratatui::{
    layout::{Constraint, Rect},
    widgets::{Row, Table},
    style::Style,
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

    let example_task = vec![
        Row::new(vec![
            "My first task",
            "PENDING",
            "14:00",
            "16:00",
            "--:--",
            "--:--",
            "--:--:--",
        ]),
    ];

    let task_rows = app.tasks.iter().enumerate().map(|(index, task)| {
        let row = Row::new(vec![
            task.name.clone(),
            task.status.clone(),
            task.planned_start.clone(),
            task.planned_end.clone(),
            task.actual_start.clone(),
            task.actual_end.clone(),
            task.elapsed.clone(),
        ]);

        if index == app.selected_task {
            row.style(Style::default().reversed())
        } else {
            row
        }
    });

    let rows = example_task.into_iter().chain(task_rows);

    let table = Table::new(rows, widths);

    frame.render_widget(table, area);
}
