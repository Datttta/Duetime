use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    widgets::{Block, Cell, Padding, Paragraph, Row, Table},
    style::{Color, Style},
    text::Line,
    Frame,
};

use crate::{
    app::{App, Panel, Popup},
    navigation::vim_navigation::NavigationMode,
    ui::{
        theme::{task_selection_color, unfocused_panel},
        widgets::input::ellipsize,
    },
};

use chrono::{NaiveDate, NaiveTime, Local};
use serde::{Deserialize, Serialize};
use super::actions;

#[derive(Default)]
pub struct AgendaEvent {
    pub name: String,
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub repeat: bool,
}

#[derive(Default, Serialize, Deserialize)]
pub struct AgendaEventData {
    pub name: String,
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub repeat: bool,
}

impl AgendaEvent {
    pub fn to_data(&self) -> AgendaEventData {
        AgendaEventData {
            name: self.name.clone(),
            date: self.date,
            time: self.time,
            repeat: self.repeat,
        }
    }

    pub fn from_data(data: AgendaEventData) -> Self {
        AgendaEvent {
            name: data.name,
            date: data.date,
            time: data.time,
            repeat: data.repeat,
        }
    }
}

fn format_countdown(date: NaiveDate) -> String {
    let today = Local::now().date_naive();

    let days = (date - today).num_days();

    match days {
        0 => "Today".to_string(),
        1 => "1 day".to_string(),
        days if days > 1 => format!("{} days", days),
        -1 => "Yesterday".to_string(),
        days => format!("{} days ago", days.abs()),
    }
}

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
        Constraint::Length(10), // event date
        Constraint::Length(2),  // space
        Constraint::Length(5),  // time of the event
        Constraint::Length(2),  // space
        Constraint::Length(7),  // countdown
    ];

    let visual_start = app.n_visual_start;
    let visual_mode = is_visual; 
    let current = app.agenda_tasks_table_state.selected();

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

    for (index, event) in app.events.iter().enumerate() {
        let time = event
            .time
            .map(|time| time.format("%H:%M").to_string())
            .unwrap_or_default();

        let countdown = format_countdown(event.date);

        let mut row = Row::new(vec![
            Cell::from(format!("  {}", ellipsize(&event.name, 22))),
            Cell::from(
                Line::from(event.date.format("%d-%m-%y").to_string())
                    .alignment(Alignment::Center),
            ),
            Cell::from(String::new()),
            Cell::from(time),
            Cell::from(String::new()),
            Cell::from(
                Line::from(countdown)
                    .alignment(Alignment::Center),
            ),
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

    let table = Table::new(rows, columns)
        .highlight_symbol("> ")
        .row_highlight_style(highlight_style);


    frame.render_stateful_widget(
        table,
        area,
        &mut app.agenda_tasks_table_state,
    );
}
