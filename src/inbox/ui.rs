use crate::{
    ui::{
        widgets::input::ellipsize,
        theme::{task_selection_color, unfocused_panel},
    },
    app::{App, Popup, Panel, Priority},
    navigation::vim_navigation::NavigationMode,
};

use ratatui::{
    layout::{Constraint, Rect, Layout, Flex, Alignment},
    widgets::{
        Row, Table, Cell, Paragraph, Padding, Block,
        Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    style::{Style, Color},
    text::{Line, Span},
    Frame,
};

use serde::{Deserialize, Serialize};

const ITEM_NAME_LENGHT: u16 = 84;

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
        Constraint::Min(0),    // plans
        Constraint::Length(1), // search 
    ])
    .split(inner);

    let is_visual = app.focused_panel == Panel::Inbox
        && app.n_mode == NavigationMode::Visual;

    // header
    let columns = Layout::horizontal([
        Constraint::Length(2), // extra
        Constraint::Length(86), // Item    
        Constraint::Length(10), // Priority
    ])
    .flex(Flex::Start)
    .split(chunks[0]);

    frame.render_widget(Paragraph::new("Item"), columns[1]);
    frame.render_widget(Paragraph::new("Priority"), columns[2]);
    
    // Scrollbar
    let visible_height = chunks[2].height as usize;
    let item_count = app.inbox_items.len();
    let scroll_offset = app.inbox_tasks_table_state.offset();

    let mut table_area = chunks[2];
    if item_count > visible_height {
        // Leave a 2-column margin on the right so highlights don't bleed into the scrollbar
        table_area.width = table_area.width.saturating_sub(2);
    }

    if item_count > visible_height {
        let max_scroll = item_count.saturating_sub(visible_height);
        
        let mut scrollbar_state = ScrollbarState::new(max_scroll + 2)
            .position(scroll_offset);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .thumb_symbol("▊")
            .track_symbol(Some(""))
            .begin_symbol(Some(""))
            .end_symbol(Some(""));

        let scrollbar_area = Rect {
            x: area.x + area.width - 2,
            y: chunks[1].y,
            width: 1,
            height: chunks[2].height + 1,
        };

        frame.render_stateful_widget(
            scrollbar,
            scrollbar_area,
            &mut scrollbar_state,
        );

        let match_info = if app.inbox_search.matches.is_empty() {
            "0/0".to_string()
        } else {
            format!(
                "{}/{}",
                app.inbox_search.current_match + 1,
                app.inbox_search.matches.len()
            )
        };
        
        if app.n_mode == NavigationMode::SearchInbox 
           || app.n_mode == NavigationMode::SearchInboxNavigation 
        {
            let search_line = Line::from(vec![
                Span::raw(" /"),
                Span::raw(&app.inbox_search.query),
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
    }
    
    draw_items(frame, table_area, app, is_visual);
}

pub fn draw_items (
    frame: &mut Frame,
    area: Rect,
    app: &mut App,
    is_visual: bool,
    ) {
    let columns = [
        Constraint::Length(ITEM_NAME_LENGHT), // inbox input
        Constraint::Length(3), // space
        Constraint::Length(6), // priority 
    ];

    let visual_start = app.n_visual_start;
    let visual_mode = is_visual;
    let current = app.inbox_tasks_table_state.selected();

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
            Cell::from(format!("  {}", ellipsize(&item.input, (ITEM_NAME_LENGHT - 3).into()))),
            Cell::from(String::new()),
            Cell::from(
                Line::from(item.priority.as_str())
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
        //.highlight_symbol("> ");
        .row_highlight_style(highlight_style);


    frame.render_stateful_widget(
        table,
        area,
        &mut app.inbox_tasks_table_state,
    );
}
