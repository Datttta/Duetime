use crate::{
    ui::{
        widgets::input::ellipsize,
        theme::{task_selection_color, unfocused_panel},
    },
    app::{App, Popup, Panel, Priority},
    navigation::vim_navigation::NavigationMode,
};

use ratatui::{
    layout::{Constraint, Rect, Layout, Flex},
    widgets::{Row, Table, Cell, Paragraph, Padding, Block},
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

pub fn header_draw (frame: &mut Frame, area: Rect) {
}

pub fn draw_inbox_panel (
    frame: &mut Frame,
    area: Rect,
    app: &mut App
) {
    let border_color = if app.focused_panel == Panel::Inbox {
        Color::White
    } else {
        unfocused_panel()
    };

    let border = Block::bordered()
        .title(" Inbox ")
        .border_style(Style::default().fg(border_color))
        .padding(Padding::new(0, 0, 1, 0));
    
    let inner = border.inner(area);

    frame.render_widget(border, area);

    let chunks = Layout::vertical ([
        Constraint::Length(1), // header
        Constraint::Length(1), // spacing
        Constraint::Min(0),    // tasks
    ])
    .split(inner);

    let is_visual = app.focused_panel == Panel::Inbox
        && app.n_mode == NavigationMode::Visual;

    // header
    let columns = Layout::horizontal([
        Constraint::Percentage(2), // extra
        Constraint::Percentage(78), // Item    
        Constraint::Percentage(20), // Priority
    ])
    .flex(Flex::Start)
    .split(chunks[0]);

    frame.render_widget(Paragraph::new("Item"), columns[1]);
    frame.render_widget(Paragraph::new("Priority"), columns[2]);
    
    // draw
    draw(frame, chunks[2], app, is_visual);
}

pub fn draw(
    frame: &mut Frame,
    area: Rect,
    app: &mut App,
    is_visual: bool,
    ) {
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
            Cell::from(format!("  {}", ellipsize(&item.input, 77))),
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
