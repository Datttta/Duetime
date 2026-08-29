use crate::{
    ui::{
        widgets::input::ellipsize,
        theme::task_selection_color,
    },
    app::{App, Popup, Panel, Priority},
};

use ratatui::{
    layout::{Constraint, Rect, Alignment},
    widgets::{Row, Table, Cell},
    style::{Style, Color},
    text::Line,
    Frame,
};

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct InboxItemInfo {
    pub input: String,
    pub priority: Priority,
}

#[derive(Serialize, Deserialize)]
pub struct InboxItemInfoData {
    pub input: String,
    pub priority: Priority,
}

impl InboxItemInfo {
    pub fn to_data(&self) -> InboxItemInfoData {
        InboxItemInfoData {
            input: self.input.clone(),
            priority: self.priority.clone(),
        }
    }

    pub fn from_data(data: InboxItemInfoData) -> Self {
        InboxItemInfo {
            input: data.input,
            priority: data.priority,
        }
    }
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Low => "LOW",
            Priority::Medium => "MEDIUM",
            Priority::High => "HIGH",
        }
    }
}

pub fn draw(frame: &mut Frame, area: Rect, app: &mut App, is_visual: bool) {
    let columns = [
        Constraint::Percentage(82), // inbox input
        Constraint::Percentage(18), // priority (still don't now if i add it)
    ];

    let visual_start = app.n_visual_start;
    let visual_mode = is_visual;
    let current = app.inbox_table_state.selected();

    let popup_open = !matches!(app.popup, Popup::None);

    let highlight_style = if popup_open || app.focused_panel != Panel::Inbox {
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

    for (index, item) in app.inbox_items.iter().enumerate() {
        // Draw insertion line before this task.

        let mut row = Row::new(vec![
            Cell::from(format!("  {}", ellipsize(&item.input, 75))),
            Cell::from(
                Line::from(item.priority.as_str()),
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
        //.highlight_symbol("> ");
        .row_highlight_style(highlight_style);


    frame.render_stateful_widget(
        table,
        area,
        &mut app.inbox_table_state,
    );
}

