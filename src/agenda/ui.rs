use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    widgets::{Block, Cell, Padding, Paragraph, Row, Table},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    Frame,
};

use crate::{
    app::{App, Panel, Popup},
    
    ui::{
        theme::{task_selection_color, unfocused_panel},
        widgets::input::ellipsize,
    },
    
    navigation::vim_navigation::NavigationMode,
};

use chrono::{NaiveDate, NaiveTime, Local};
use serde::{Deserialize, Serialize};
use super::actions;

pub const DATE_EDITABLE_POSITIONS: [usize; 6] = [0, 1, 3, 4, 6, 7];
pub const TIME_EDITABLE_POSITIONS: &[usize] = &[0, 1, 3, 4];

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

pub struct DateTimeInput {
    pub value: String,
    pub cursor: usize,
    pub editable_positions: &'static [usize],
}

impl DateTimeInput {
    pub fn move_left(&mut self) {
        if let Some(position) = self
            .editable_positions
            .iter()
            .position(|&pos| pos == self.cursor)
        {
            if position > 0 {
                self.cursor = self.editable_positions[position - 1];
            }
        }
    }

    pub fn move_right(&mut self) {
        if let Some(position) = self
            .editable_positions
            .iter()
            .position(|&pos| pos == self.cursor)
        {
            if position + 1 < self.editable_positions.len() {
                self.cursor = self.editable_positions[position + 1];
            }
        }
    }

    pub fn insert_digit(&mut self, digit: char) {
        if !digit.is_ascii_digit() {
            return;
        }

        self.value
            .replace_range(self.cursor..self.cursor + 1, &digit.to_string());

        self.move_right();
    }
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

pub fn draw_date_time_input(
    frame: &mut Frame,
    area: Rect,
    input: &DateTimeInput,
    selected: bool,
) {
    let spans = input
        .value
        .chars()
        .enumerate()
        .map(|(index, character)| {
            let style = if selected && index == input.cursor {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            Span::styled(character.to_string(), style)
        })
        .collect::<Vec<_>>();

    let paragraph = Paragraph::new(Line::from(spans));

    frame.render_widget(paragraph, area);
}

pub fn draw_repeat_input(
    frame: &mut Frame,
    area: Rect,
    repeat: bool,
    selected: bool,
) {
    let checkbox = if repeat { "☑" } else { "☐" };

    let style = if selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let line = Line::from(vec![
        Span::raw("Repeat "),
        Span::styled(checkbox, style),
    ]);

    frame.render_widget(Paragraph::new(line), area);
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

    let today = Local::now().date_naive();

    // Partition index maps for events array
    let (today_indices, upcoming_indices): (Vec<usize>, Vec<usize>) =
        (0..app.events.len()).partition(|&i| app.events[i].date <= today);

    // Dynamic height constraints based on section item counts
    let today_height = (today_indices.len() as u16).max(1);
    let upcoming_height = (upcoming_indices.len() as u16).max(1);

    let chunks = Layout::vertical([
        Constraint::Length(1),              // "Today" Header
        Constraint::Length(today_height),   // Today events list
        Constraint::Length(1),              // "space
        Constraint::Length(1),              // "Upcoming" Header
        Constraint::Length(upcoming_height),// Upcoming events list
    ])
    .split(inner);

    let is_visual = app.focused_panel == Panel::Agenda
        && app.n_mode == NavigationMode::Visual;

    frame.render_widget(Paragraph::new("Today"), chunks[0]);
    draw_events(frame, chunks[1], app, &today_indices, is_visual);

    frame.render_widget(Paragraph::new("Upcoming"), chunks[3]);
    draw_events(frame, chunks[4], app, &upcoming_indices, is_visual);
}

pub fn draw_events(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    section_indices: &[usize],
    is_visual: bool,
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
    let current = app.agenda_table_state.selected();
    let popup_open = !matches!(app.popup, Popup::None);

    let base_highlight_style = if popup_open || app.focused_panel != Panel::Agenda {
        Style::default()
    } else if app.move_state.is_moving() {
        Style::default()
    } else if visual_mode {
        Style::default().fg(Color::Black).bg(Color::White)
    } else {
        Style::default().bg(task_selection_color()).fg(Color::Black)
    };

    let mut rows = Vec::new();

    for &global_index in section_indices {
        let event = &app.events[global_index];

        let time = event
            .time
            .map(|time| time.format("%H:%M").to_string())
            .unwrap_or_default();

        let countdown = format_countdown(event.date);

        let is_selected = current == Some(global_index);
        let prefix = if is_selected { "> " } else { "  " };

        let mut row = Row::new(vec![
            Cell::from(format!("{}{}", prefix, ellipsize(&event.name, 20))),
            Cell::from(
                Line::from(event.date.format("%a, %b %-d").to_string())
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


        // Apply visual selection range highlighting across global indices
        if !popup_open && visual_mode {
            if let (Some(start), Some(end)) = (visual_start, current) {
                let first = start.min(end);
                let last = start.max(end);

                if global_index >= first && global_index <= last {
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

    let table = Table::new(rows, columns);

    frame.render_widget(table, area);
}
