use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        Block, Clear, Paragraph, Padding,
        Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    Frame,
};

use crate::{
    app::{App, Popup},
    ui::theme::gray_color,
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame, 40, 30);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title(" Help ")
        .padding(Padding::new(2, 2, 1, 1))
        .border_style(Style::default().fg(Color::White));

    frame.render_widget(&block, area);

    let lines = vec![
        Line::from("Navigation"),
        Line::from(""),
        Line::from(vec![
            Span::styled("j -", Style::default().fg(Color::Yellow)),
            Span::styled(" Move down", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("k -", Style::default().fg(Color::Yellow)),
            Span::styled(" Move up", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("gg -", Style::default().fg(Color::Yellow)),
            Span::styled(" Go to first item", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("G -", Style::default().fg(Color::Yellow)),
            Span::styled(" Go to last item", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("v / V -", Style::default().fg(Color::Yellow)),
            Span::styled(" Visual mode", Style::default().fg(gray_color())),
        ]),

        Line::from(""),
        Line::from("Tasks"),
        Line::from(""),

        Line::from(vec![
            Span::styled("t -", Style::default().fg(Color::Yellow)),
            Span::styled(" Add task", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("e -", Style::default().fg(Color::Yellow)),
            Span::styled(" Edit task", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("P -", Style::default().fg(Color::Yellow)),
            Span::styled(" Presets", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("Ctrl+l -", Style::default().fg(Color::Yellow)),
            Span::styled(" Known tasks", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("dd -", Style::default().fg(Color::Yellow)),
            Span::styled(" Delete task", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("x -", Style::default().fg(Color::Yellow)),
            Span::styled(" Move task", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("s -", Style::default().fg(Color::Yellow)),
            Span::styled(" Start/stop task", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("c -", Style::default().fg(Color::Yellow)),
            Span::styled(" Complete task", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("r -", Style::default().fg(Color::Yellow)),
            Span::styled(" Reset task", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("R -", Style::default().fg(Color::Yellow)),
            Span::styled(" Hard reset task", Style::default().fg(gray_color())),
        ]),

        Line::from(""),
        Line::from("Inbox"),
        Line::from(""),

        Line::from(vec![
            Span::styled("ai -", Style::default().fg(Color::Yellow)),
            Span::styled(" Add inbox item", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("e -", Style::default().fg(Color::Yellow)),
            Span::styled(" Edit inbox item", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("i -", Style::default().fg(Color::Yellow)),
            Span::styled(" Inbox item information", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("cc -", Style::default().fg(Color::Yellow)),
            Span::styled(" Copy inbox item", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("dd -", Style::default().fg(Color::Yellow)),
            Span::styled(" Delete inbox item", Style::default().fg(gray_color())),
        ]),

        Line::from(""),
        Line::from("Others"),
        Line::from(""),

        Line::from(vec![
            Span::styled("q -", Style::default().fg(Color::Yellow)),
            Span::styled(" Quit", Style::default().fg(gray_color())),
        ]),
        Line::from(vec![
            Span::styled("Esc -", Style::default().fg(Color::Yellow)),
            Span::styled(" Close / cancel", Style::default().fg(gray_color())),
        ]),
    ];

    // Height available for the text inside the block.
    let inner = block.inner(area);
    let visible_height = inner.height;

    let max_scroll = lines
        .len()
        .saturating_sub(visible_height as usize);

    // Keep the scroll position valid even if the popup size changes.
    let scroll = app.help_scroll.min(max_scroll as u16);

    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Left)
        .scroll((scroll, 0));

    frame.render_widget(paragraph, inner);

    // Scrollbar
    if max_scroll > 0 {
        let mut scrollbar_state = ScrollbarState::new(max_scroll + 1)
            .position(scroll as usize);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .thumb_symbol("▊") // variations: "▉", "▊"(current), "▋"
            .track_symbol(Some(""))
            .begin_symbol(Some(""))
            .end_symbol(Some(""));

        let scrollbar_area = Rect {
            x: area.x + area.width - 3,
            y: area.y + 1,
            width: 1,
            height: area.height - 2,
        };

        frame.render_stateful_widget(
            scrollbar,
            scrollbar_area,
            &mut scrollbar_state,
        );
    }
}

fn centered_rect(frame: &Frame, width: u16, height: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .split(frame.area());

    let horizontal = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .split(vertical[0]);

    horizontal[0]
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('j') => {
            app.help_scroll = app.help_scroll.saturating_add(1);
        }

        KeyCode::Char('k') => {
            app.help_scroll = app.help_scroll.saturating_sub(1);
        }

        KeyCode::Esc
        | KeyCode::Char('q')
        | KeyCode::Char('h')
        | KeyCode::Char('?') => {
            app.help_scroll = 0;
            app.popup = Popup::None;
        }

        _ => {}
    }
}
