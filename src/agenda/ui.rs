//use log::info;
use crossterm::event::{KeyCode, KeyEvent}; 
                                           
use ratatui::{
    layout::{Alignment, Constraint, Rect},
    widgets::{
        Block, Cell, Padding, Paragraph, Row, Table,
        Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    style::{Color, Style, Modifier},
    text::{Line, Span},
    Frame,
};

use crate::{
    app::{App, Panel, Popup},
    ui::{
        theme::unfocused_panel,
        widgets::input::ellipsize,
    },
    navigation::vim_navigation::NavigationMode,
};

use chrono::{NaiveDate, NaiveTime, Local, Datelike, Duration};
use serde::{Deserialize, Serialize};

pub const DATE_EDITABLE_POSITIONS: [usize; 6] = [0, 1, 3, 4, 6, 7];
pub const TIME_EDITABLE_POSITIONS: &[usize] = &[0, 1, 3, 4];

const EVENT_NAME_LENGTH: u16 = 55;

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

pub fn format_countdown(date: NaiveDate) -> String {
    let today = Local::now().date_naive();
    let days = (date - today).num_days();

    match days {
        1 => "1 day".to_string(),
        days => format!("{} days", days),
    }
}

pub fn update_repeating_events(events: &mut Vec<AgendaEvent>) {
    let today = Local::now().date_naive();

    for event in events.iter_mut() {
        if !event.repeat {
            continue;
        }

        while event.date < today {
            let next_year = event.date.year() + 1;

            event.date = match event.date.with_year(next_year) {
                Some(date) => date,
                None => {
                    event.date
                        .with_day(28)
                        .unwrap()
                        .with_year(next_year)
                        .unwrap()
                }
            };
        }
    }
}

// Returns true if the table row index is a header or spacer
pub fn is_header_or_spacer(row_idx: usize, today_count: usize, app: &mut App) -> bool {
    if row_idx == 0 {
        let offset = app.agenda_table_state.offset_mut();
        *offset = 0;
        return true; // "Today" header
    }
    if row_idx == today_count + 1 {
        return true; // Spacing row
    }
    if row_idx == today_count + 2 {
        if let Some(current) = app.agenda_table_state.selected() {
            if current == today_count + 2{
                app.agenda_table_state.select(Some(current +1))
            }
        }
        return true; // "Upcoming" header
    }
    false
}

// Snaps the selection to the closest valid event row if it lands on a header/spacer
pub fn snap_agenda_selection(app: &mut App, total_rows: usize, today_count: usize, key: KeyEvent) {
    if let Some(current) = app.agenda_table_state.selected() {
        if !is_header_or_spacer(current, today_count, app) {
            return; // Already on a valid row
        }

        let mut found = None;

        // Check which key/letter was typed to decide search direction
        let search_down_first = match key.code {
            KeyCode::Char('k') | KeyCode::Up => false, // Pressed up -> search upwards first
            KeyCode::Char('j') | KeyCode::Down => true,  // Pressed down -> search downwards first
            KeyCode::Char('G') => false,                 // Jumped to bottom -> search upwards
            _ => true,                                   // Default fallback (e.g., 'gg' or jumps)
        };

        if search_down_first {
            // 1. Search downwards first
            let mut test_idx = current;
            while test_idx < total_rows {
                if !is_header_or_spacer(test_idx, today_count, app) {
                    found = Some(test_idx);
                    break;
                }
                test_idx += 1;
            }
            // 2. Fallback to upwards if nothing found below
            if found.is_none() {
                let mut test_idx = current;
                while test_idx > 0 {
                    test_idx -= 1;
                    if !is_header_or_spacer(test_idx, today_count, app) {
                        found = Some(test_idx);
                        break;
                    }
                }
            }
        } else {
            // 1. Search upwards first
            let mut test_idx = current;
            while test_idx > 0 {
                test_idx -= 1;
                if !is_header_or_spacer(test_idx, today_count, app) {
                    found = Some(test_idx);
                    break;
                }
            }
            // 2. Fallback to downwards if nothing found above
            if found.is_none() {
                let mut test_idx = current;
                while test_idx < total_rows {
                    if !is_header_or_spacer(test_idx, today_count, app) {
                        found = Some(test_idx);
                        break;
                    }
                    test_idx += 1;
                }
            }
        }

        app.agenda_table_state.select(found);
    }
}

// Helper to map the currently selected table row back to the actual event index in app.events
pub fn get_selected_global_index(app: &App) -> Option<usize> {
    let selected_row = app.agenda_table_state.selected()?;
    
    let today = Local::now().date_naive();
    let max_date = today + Duration::days(30);

    let visible_indices: Vec<usize> = (0..app.events.len())
        .filter(|&i| {
            let date = app.events[i].date;
            date >= today && date <= max_date
        })
        .collect();

    let (today_indices, upcoming_indices): (Vec<usize>, Vec<usize>) =
        visible_indices
            .into_iter()
            .partition(|&i| app.events[i].date == today);

    let today_count = today_indices.len();

    // Map table row index back to the correct slice
    if selected_row >= 1 && selected_row <= today_count {
        // Today event slice (Row 1 is the first event, since Row 0 is the "Today" header)
        Some(today_indices[selected_row - 1])
    } else if selected_row > today_count + 2 {
        // Upcoming event slice (Account for Today header [0], today events, spacer, and Upcoming header)
        let upcoming_idx = selected_row - (today_count + 3);
        Some(upcoming_indices.get(upcoming_idx).copied()?)
    } else {
        None // Safely ignored if it somehow lands on a header or spacer row
    }
}

pub fn remove_expired_events(events: &mut Vec<AgendaEvent>) {
    let today = Local::now().date_naive();
    events.retain(|event| event.repeat || event.date >= today);
}

pub fn draw_repeat_input(
    frame: &mut Frame,
    area: Rect,
    repeat: bool,
    selected: bool,
) {
    let checkbox = if repeat { "" } else { "" };

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

    let chunks = ratatui::layout::Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .split(inner);

    let table_area = chunks[0];
    let search_area = chunks[1];

    let today = Local::now().date_naive();
    let max_date = today + Duration::days(30);

    let visible_indices: Vec<usize> = (0..app.events.len())
        .filter(|&i| {
            let date = app.events[i].date;
            date >= today && date <= max_date
        })
        .collect();

    let (today_indices, upcoming_indices): (Vec<usize>, Vec<usize>) =
        visible_indices
            .into_iter()
            .partition(|&i| app.events[i].date == today);

    let is_visual = app.focused_panel == Panel::Agenda
        && app.n_mode == NavigationMode::Visual;
    let popup_open = !matches!(app.popup, Popup::None);
    let current_row = app.agenda_table_state.selected();

    let columns = [
        Constraint::Length(name_length), // event name
        Constraint::Length(3),  // sapce
        Constraint::Length(5),  // time of the event
        Constraint::Length(1), // space
        Constraint::Length(11), // event date
        Constraint::Length(1), // space
        Constraint::Length(8),  // countdown
    ];

    let mut rows = Vec::new();

    // 1. Today Header (Row index 0)
    rows.push(Row::new(vec![Cell::from("Today")]).style(Style::default()));

    // 2. Today Events
    for &global_index in &today_indices {
        let event = &app.events[global_index];
        let row_idx = rows.len();
        rows.push(build_agenda_row(event, row_idx, current_row, is_visual, popup_open, app));
    }

    // 3. Spacing Row
    rows.push(Row::new(vec![Cell::from("")]));

    // 4. Upcoming Header
    rows.push(Row::new(vec![Cell::from("Upcoming")]).style(Style::default()));

    // 5. Upcoming Events
    for &global_index in &upcoming_indices {
        let event = &app.events[global_index];
        let row_idx = rows.len();
        rows.push(build_agenda_row(event, row_idx, current_row, is_visual, popup_open, app));
    }

    let total_rows = rows.len();
    let scroll_offset = app.agenda_table_state.offset();
    
    let visible_height = table_area.height as usize;

    let mut table_area = table_area;
    if total_rows > visible_height {
        table_area.width = table_area.width.saturating_sub(2);
    }

    let table = Table::new(rows, columns);
    frame.render_stateful_widget(table, table_area, &mut app.agenda_table_state);

    // Scrollbar rendering
    if total_rows > visible_height {
        let max_scroll = total_rows.saturating_sub(visible_height);
        
        let mut scrollbar_state = ScrollbarState::new(max_scroll + 1)
            .position(scroll_offset)
            .viewport_content_length(visible_height);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .thumb_symbol("▊")
            .track_symbol(Some(""))
            .begin_symbol(Some(""))
            .end_symbol(Some(""));

        let scrollbar_area = Rect {
            x: area.x + area.width - 2,
            y: table_area.y,
            width: 1,
            height: table_area.height,
        };

        let match_info = if app.agenda_search.matches.is_empty() {
            "0/0".to_string()
        } else {
            format!(
                "{}/{}",
                app.agenda_search.current_match + 1,
                app.agenda_search.matches.len()
            )
        };
        
        if app.n_mode == NavigationMode::SearchInbox 
           || app.n_mode == NavigationMode::SearchInboxNavigation 
        {
            let search_line = Line::from(vec![
                Span::raw(" /"),
                Span::raw(&app.agenda_search.query),
            ]);

            frame.render_widget(search_line, chunks[3]);
            
            frame.render_widget(
                Paragraph::new(match_info)
                    .alignment(Alignment::Right)
                    .block(
                        Block::default()
                            .padding(Padding::right(2))
                    ),
                chunks[3],
            );

        } else {
            let search_line = Paragraph::new(format!(""))
                .style(Style::default().fg(Color::White));

            frame.render_widget(search_line, chunks[3]);
        }


        frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}

// Row builder for the main agenda panel (accounts for headers and spacing rows via row_idx)
fn build_agenda_row(
    event: &AgendaEvent,
    row_idx: usize,
    current_row: Option<usize>,
    is_visual: bool,
    popup_open: bool,
    app: &App,
) -> Row<'static> {
    let time = event
        .time
        .map(|t| t.format("%H:%M").to_string())
        .unwrap_or_default();
    let countdown = format_countdown(event.date);

    let is_selected = current_row == Some(row_idx);
    let prefix = if is_selected { "> " } else { "  " };

    let mut row = Row::new(vec![
        Cell::from(format!("{}{}", prefix, ellipsize(&event.name, (EVENT_NAME_LENGTH - 2).into()))),
        Cell::from(String::new()),
        Cell::from(Line::from(time).alignment(Alignment::Center)),
        Cell::from(String::new()),
        Cell::from(Line::from(event.date.format("%a, %b %-d").to_string()).alignment(Alignment::Center)),
        Cell::from(Cell::from(String::new())),
        Cell::from(Line::from(countdown)),
    ]);

    if !popup_open && is_visual {
        if let (Some(start), Some(end)) = (app.n_visual_start, current_row) {
            let first = start.min(end);
            let last = start.max(end);
            if row_idx >= first && row_idx <= last {
                row = row.style(Style::default().fg(Color::Black).bg(Color::White));
            }
        }
    }

    row
}

// Used exclusively by the All Events popup (flat list, no headers)
pub fn draw_events(
    frame: &mut Frame,
    area: Rect,
    app: &mut App,
    section_indices: &[usize],
    is_visual: bool,
    name_length: u16,
) {
    let columns = [
        Constraint::Length(name_length), // event name
        Constraint::Length(3),  // sapce
        Constraint::Length(5),  // time of the event
        Constraint::Length(1), // space
        Constraint::Length(11), // event date
        Constraint::Length(1), // space
        Constraint::Length(8),  // countdown

    ];

    let current = app.all_events_table_state.selected();

    let mut rows = Vec::new();

    for (popup_row_idx, &global_index) in section_indices.iter().enumerate() {
        let event = &app.events[global_index];
        let is_selected = current == Some(popup_row_idx);
        let prefix = if is_selected { "> " } else { "  " };

        let time = event.time.map(|t| t.format("%H:%M").to_string()).unwrap_or_default();
        let countdown = format_countdown(event.date);

        let mut row = Row::new(vec![
            Cell::from(format!("{}{}", prefix, ellipsize(&event.name, (name_length - 2).into()))),
            Cell::from(String::new()),
            Cell::from(Line::from(time).alignment(Alignment::Center)),
            Cell::from(String::new()),
            Cell::from(Line::from(event.date.format("%a, %b %-d").to_string()).alignment(Alignment::Center)),
            Cell::from(String::new()),
            Cell::from(Line::from(countdown)),
        ]);

        if is_visual {
            if let (Some(start), Some(end)) = (app.n_visual_start, current) {
                let first = start.min(end);
                let last = start.max(end);
                if popup_row_idx >= first && popup_row_idx <= last {
                    row = row.style(Style::default().fg(Color::Black).bg(Color::White));
                }
            }
        }

        rows.push(row);
    }

    let table = Table::new(rows, columns);
    frame.render_stateful_widget(table, area, &mut app.all_events_table_state);
}
