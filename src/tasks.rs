use crate::app::App;
use crate::stopwatch::Stopwatch;

use chrono::{DateTime, Local};

use std::time::SystemTime;

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
    pub actual_start: Option<SystemTime>,
    pub actual_end: Option<SystemTime>,
    pub stopwatch: Stopwatch,
}

pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    let columns = [
        Constraint::Length(26), // task name
        Constraint::Length(12), // status
        Constraint::Length(13), // planned start
        Constraint::Length(13), // planned end
        Constraint::Length(11), // actual start
        Constraint::Length(9), // actual end
        Constraint::Length(8), // stopwatch/elapsed
    ];

    let rows = app.tasks.iter().map(|task| {
        Row::new(vec![
            format!("  {}", task.name),
            task.status.clone(),
            task.planned_start.clone(),
            task.planned_end.clone(),
            format_time(task.actual_start),
            format_time(task.actual_end),
            task.stopwatch.formatted()
        ])
    });

    let table = Table::new(rows, columns)
        .row_highlight_style(Style::default().reversed());

    frame.render_stateful_widget(table, area, &mut app.table_state);
}

fn format_time(time: Option<SystemTime>) -> String {
    match time {
        Some(t) => {
            let datetime: DateTime<Local> = t.into();
            datetime.format("%H:%M").to_string()
        }
        None => "".to_string(),
    }
}
