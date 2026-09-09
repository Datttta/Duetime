use crate::{
    ui::{
        widgets::input::ellipsize,
        theme::{task_selection_color, unfocused_panel},
    },
    app::{App, Popup, Panel, Priority},
    navigation::vim_navigation::NavigationMode,
};

use ratatui::{
    layout::{Alignment, Rect, Flex, Constraint, Layout},
    widgets::{Block, Paragraph, Padding},
    style::{Color, Style},
    text::{Line, Span},
    Frame,
};

use serde::{Deserialize, Serialize};

pub fn draw_timers_panel(
    frame: &mut Frame,
    area: Rect,
    app: &mut App,
) {
    let border_color = if app.focused_panel == Panel::Timers {
        Color::White
    } else {
        unfocused_panel()
    };

    let border = Block::bordered()
        .title(" Timers ")
        .border_style(Style::default().fg(border_color))
        .padding(Padding::new(0, 0, 1, 0));

    let inner = border.inner(area);

    frame.render_widget(border, area);

    draw_timer_placeholder(frame, inner);
}

fn draw_timer_placeholder(
    frame: &mut Frame,
    area: Rect,
) {
    // Create a smaller area for the timer
    let vertical = Layout::vertical([
        Constraint::Length(7),
    ])
    .flex(Flex::Center)
    .split(area);

    let horizontal = Layout::horizontal([
        Constraint::Length(24),
    ])
    .flex(Flex::Center)
    .split(vertical[0]);

    let timer_area = horizontal[0];

    let timer_block = Block::bordered()
        .border_style(Style::default().fg(Color::White))
        .padding(Padding::new(0, 0, 1, 0));

    let inner = timer_block.inner(timer_area);

    frame.render_widget(timer_block, timer_area);

    let text = vec![
        Line::from(
            Span::styled(
                "00:00:00",
                Style::default().fg(Color::White),
            )
        ),
        Line::from(""),
        Line::from(
            Span::styled(
                "ADD TIMER",
                Style::default().fg(Color::Gray),
            )
        ),
    ];

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, inner);
}
