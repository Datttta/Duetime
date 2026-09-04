use crate::{
    app::{App, Popup, Panel},
    stopwatch::{Stopwatch, StopwatchData},
    ui::{
        theme::{task_selection_color, unfocused_panel},
        widgets::{
            input::ellipsize,
            duration::format_duration,
        },
    },
    navigation::{
        move_items::MoveTarget,
        vim_navigation::NavigationMode,
    },
};

use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    widgets::{Block, Cell, Padding, Paragraph, Row, Table},
    style::{Color, Style},
    text::Line,
    Frame,
};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime};

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

pub fn format_time(time: Option<SystemTime>) -> String {
    match time {
        Some(t) => {
            let datetime: DateTime<Local> = t.into();
            datetime.format("%H:%M").to_string()
        }
        None => "".to_string(),
    }
}

pub fn draw_tasks_panel(
    frame: &mut Frame,
    area: Rect, 
    app: &mut App,
) {
    let border_color = if app.focused_panel == Panel::TasksTable {
        Color::White
    } else {
        unfocused_panel()
    };

    let border = Block::bordered()
        .title(" Tasks ")
        .border_style(Style::default().fg(border_color))
        .padding(Padding::new(0, 0, 1, 0));

    let inner = border.inner(area);

    frame.render_widget(border, area);

    let chunks = Layout::vertical ([
        Constraint::Length(1), // header
        Constraint::Length(1), // spacing
        Constraint::Min(0),    // tasks
        Constraint::Length(2), // footer
    ])
    .split(inner);
    
    // header
    let columns = Layout::horizontal([
        Constraint::Length(11), // extra
        Constraint::Length(16), // TO-DO: 5    
        Constraint::Length(11), // Status: 6 
        Constraint::Length(15), // Plan start: 10 
        Constraint::Length(13), // Plan end: 8 
        Constraint::Length(13), // Act start: 8 
        Constraint::Length(11), // Act end: 6 
        Constraint::Length(7),  // Elapsed: 7 
    ])
    .flex(Flex::Start)
    .split(chunks[0]);

    frame.render_widget(Paragraph::new("TO-DO"), columns[1]);
    frame.render_widget(Paragraph::new("Status"), columns[2]);
    frame.render_widget(Paragraph::new("Plan start"), columns[3]);
    frame.render_widget(Paragraph::new("Plan end"), columns[4]);
    frame.render_widget(Paragraph::new("Act start"), columns[5]);
    frame.render_widget(Paragraph::new("Act end"), columns[6]);
    frame.render_widget(Paragraph::new("Elapsed"), columns[7]);
    ////

    let is_visual = app.focused_panel == Panel::TasksTable
        && app.n_mode == NavigationMode::Visual;

    draw_tasks(frame, chunks[2], app, is_visual);

    if let Popup::None = app.popup {
        let status = Paragraph::new(format!(
                " Total elapsed {}",
                format_duration(app.total_elapsed())
        ))
        .block(
            Block::default()
                .padding(Padding::new(2, 0, 0, 0))
        );

        frame.render_widget(status, chunks[3]);
    }
}

pub fn draw_tasks(
    frame: &mut Frame,
    area: Rect, 
    app: &mut App, 
    is_visual: bool,
    ) {
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
    let current = app.tasks_table_state.selected();

    let popup_open = !matches!(app.popup, Popup::None);

    let highlight_style = if popup_open || app.focused_panel != Panel::TasksTable {
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
        &mut app.tasks_table_state,
    );
}
