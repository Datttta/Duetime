use crate::{
    app::{App, Popup, Panel},
    stopwatch::{Stopwatch, StopwatchData},
    ui::widgets::input::ellipsize,
    ui::theme::task_selection_color,
    vim_navigation::NavigationMode,
    move_items::MoveTarget
};

use ratatui::{
    layout::{Constraint, Rect, Alignment},
    widgets::{Row, Table, Cell},
    style::{Style, Color},
    text::Line,
    Frame,
};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;


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

#[derive(Serialize, Deserialize)]
pub struct TaskInfoData {
    pub name: String,
    pub status: String,
    pub planned_start: String,
    pub planned_end: String,
    pub actual_start: Option<SystemTime>,
    pub actual_end: Option<SystemTime>,
    pub stopwatch: StopwatchData,
}

impl TaskInfo {
    pub fn to_data(&self) -> TaskInfoData {
        TaskInfoData {
            name: self.name.clone(),
            status: self.status.clone(),
            planned_start: self.planned_start.clone(),
            planned_end: self.planned_end.clone(),
            actual_start: self.actual_start.clone(),
            actual_end: self.actual_end.clone(),
            stopwatch: self.stopwatch.to_data(),
        }
    }

    pub fn from_data(data: TaskInfoData) -> Self {
        TaskInfo {
            name: data.name,
            status: data.status,
            planned_start: data.planned_start,
            planned_end: data.planned_end,
            actual_start: data.actual_start,
            actual_end: data.actual_end,
            stopwatch: Stopwatch::from_data(data.stopwatch),
            ..Default::default()
        }
    }
}

pub fn draw(frame: &mut Frame, area: Rect, app: &mut App, is_visual: bool) {
    let columns = [
        Constraint::Length(24), // task name
        Constraint::Length(11), // status
        Constraint::Length(3), // gap
        Constraint::Length(13), // planned start
        Constraint::Length(12), // planned end
        Constraint::Length(11), // actual start
        Constraint::Length(9), // actual end
        Constraint::Length(8), // stopwatch/elapsed
    ];

    let visual_start = app.n_visual_start;
    let visual_mode = is_visual; 
    let current = app.table_state.selected();

    let popup_open = !matches!(app.popup, Popup::None);

    let highlight_style = if popup_open || app.focused_panel != Panel::Tasks {
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

    let mut rows = Vec::new();

    for (index, task) in app.tasks.iter().enumerate() {
        // Draw insertion line before this task.
        if app.move_state.is_moving()
            && app.move_state.target == Some(MoveTarget::Tasks)
            && app.move_state.position == Some(index)
        {
            rows.push(Row::new(vec![
                Cell::from("────────────────────"),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ]));
        }

        let mut row = Row::new(vec![
            Cell::from(format!("  {}", ellipsize(&task.name, 22))),
            Cell::from(
                Line::from(task.status.as_str())
                    .alignment(Alignment::Center),
            ),
            Cell::from(String::new()),
            Cell::from(task.planned_start.clone()),
            Cell::from(task.planned_end.clone()),
            Cell::from(format_time(task.actual_start)),
            Cell::from(format_time(task.actual_end)),
            Cell::from(task.stopwatch.formatted()),
        ]);

        if !popup_open && visual_mode {
            if let (Some(start), Some(end)) = (visual_start, current) {
                let first = start.min(end);
                let last = start.max(end);

                if index >= first && index <= last {
                    row = row.style(
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::White),
                    );
                }
            }
        }

        rows.push(row);
    }

    // Insertion line after the final task.
    if app.move_state.is_moving()
        && app.move_state.position == Some(app.tasks.len())
    {
        rows.push(Row::new(vec![
            Cell::from("────────────────────"),
            Cell::from(""),
            Cell::from(""),
            Cell::from(""),
            Cell::from(""),
            Cell::from(""),
            Cell::from(""),
            Cell::from(""),
        ]));
    }

    let table = Table::new(rows, columns)
        //.highlight_symbol("> ");
        .row_highlight_style(highlight_style);


    frame.render_stateful_widget(
        table,
        area,
        &mut app.table_state,
    );
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
