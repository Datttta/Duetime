use crate::{
    app::{App, Popup},
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

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct InboxItemInfo {
    pub item: String,
    pub priority: String,
}

#[derive(Serialize, Deserialize)]
pub struct InboxItemInfoData {
    pub item: String,
    pub priority: String,
}

impl InboxItemInfo {
    pub fn to_data(&self) -> InboxItemInfoData {
        InboxItemInfoData {
            item: self.item.clone(),
            priority: self.priority.clone(),
        }
    }

    pub fn from_data(data: InboxItemInfoData) -> Self {
        InboxItemInfo {
            item: data.item,
            priority: data.priority,
        }
    }
}

pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    let columns = [
        Constraint::Percentage(80), // task name
        Constraint::Percentage(20), // priority (still don't now if i add it)
    ];

    let visual_start = app.n_visual_start;
    let visual_mode = app.n_mode == NavigationMode::Visual;
    let current = app.table_state.selected();

    let popup_open = !matches!(app.popup, Popup::None);

    let highlight_style = if popup_open {
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
            Cell::from(format!("  {}", ellipsize(&item.item, 22))),
            Cell::from(item.priority.clone()),
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
