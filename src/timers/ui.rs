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
    widgets::{Paragraph, Padding, Block},
    style::{Style, Color},
    text::{Line, Text, Span},
    widgets::canvas::{Canvas, Circle, Points},
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
    // 1. Split the area into top (circle canvas) and bottom (text) layout
    let chunks = Layout::vertical([
        Constraint::Min(0),      // Canvas area
        Constraint::Length(1),   // 00:00:00
        Constraint::Length(1),   // ADD TIMER
    ])
    .flex(Flex::Center)
    .split(area);

    // 2. Draw a visually perfect circle using Canvas
    // Terminal pixels are ~2:1 tall-to-wide. Scaling x_bounds by 2.0 corrects the stretch.
    let canvas = Canvas::default()
        .x_bounds([-20.0, 20.0])
        .y_bounds([-10.0, 10.0])
        .paint(|ctx| {
            ctx.draw(&Circle {
                x: 0.0,
                y: 0.0,
                radius: 8.0,
                color: Color::White,
            });
        });

    frame.render_widget(canvas, chunks[0]);

    // 3. Render "00:00:00"
    let time_text = Paragraph::new("00:00:00")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));
    frame.render_widget(time_text, chunks[1]);

    // 4. Render "ADD TIMER"
    let add_text = Paragraph::new("ADD TIMER")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(add_text, chunks[2]);
}
