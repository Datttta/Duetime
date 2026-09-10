use crate::{
    ui::{
        widgets::{
            input::ellipsize,
            duration::format_duration,
        },
        theme::{task_selection_color, unfocused_panel},
    },
    app::{App, Popup, Panel, Priority},
    navigation::vim_navigation::NavigationMode,
};

use ratatui::{
    layout::{Alignment, Rect, Flex, Constraint, Layout},
    widgets::{Block, Paragraph, Cell, Padding},
    style::{Color, Style},
    text::{Line, Span},
    Frame,
};

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Default)]
pub struct TimerInfo {
    pub name: String,
    pub duration: Duration,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct TimerInfoData {
    pub name: String,
    pub duration: u64,
    pub status: String,
}

pub struct DurationInput {
    pub value: String,
    pub cursor: usize,
}

impl TimerInfo {
    pub fn to_data(&self) -> TimerInfoData {
        TimerInfoData {
            name: self.name.clone(),
            duration: self.duration.as_secs(),
            status: self.status.clone(),
        }
    }

    pub fn from_data(data: TimerInfoData) -> Self {
        TimerInfo {
            name: data.name,
            duration: Duration::from_secs(data.duration),
            status: data.status,
        }
    }
}

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

    if app.timers.is_empty() {
        draw_timer_placeholder(frame, inner);
    } else {
        draw_timers(frame, inner, app);
    }

}

pub fn draw_timers(
    frame: &mut Frame,
    area: Rect,
    app: &mut App,
) {
    let timer_width = 34; // Updated from 24 to match timer_placeholder width

    // Center the timer card vertically using the exact same constraints as timer_placeholder
    let vertical = Layout::vertical([
        Constraint::Length(9),
    ])
    .flex(Flex::Center)
    .split(area);

    let constraints = app
        .timers
        .iter()
        .map(|_| Constraint::Length(timer_width))
        .collect::<Vec<_>>();

    let timer_areas = Layout::horizontal(constraints)
        .flex(Flex::Center)
        .split(vertical[0]);

    for (timer, timer_area) in app.timers.iter().zip(timer_areas.iter()) {
        draw_timer(frame, *timer_area, timer);
    }
}

fn draw_timer(
    frame: &mut Frame,
    timer_area: Rect,
    timer: &TimerInfo,
) {
    let timer_block = Block::bordered()
        .border_style(Style::default().fg(Color::White))
        .padding(Padding::new(0, 0, 1, 0));

    let inner = timer_block.inner(timer_area);

    frame.render_widget(timer_block, timer_area);

    let time_str = format_duration(timer.duration);
    let (top_line, mid_line, bot_line) = render_time_display(&time_str);

    // Identical layout structure to draw_timer_placeholder
    let text = vec![
        Line::from(Span::styled(timer.name.as_str(), Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(Span::styled(top_line, Style::default().fg(Color::White))),
        Line::from(Span::styled(mid_line, Style::default().fg(Color::White))),
        Line::from(Span::styled(bot_line, Style::default().fg(Color::White))),
        Line::from(Span::styled(timer.status.as_str(), Style::default().fg(Color::Gray))),
    ];

    let paragraph = Paragraph::new(text).alignment(Alignment::Center);

    frame.render_widget(paragraph, inner);
}

fn get_big_glyph(c: char) -> (&'static str, &'static str, &'static str) {
    match c {
        '0' => ("█▀█", "█ █", "▀▀▀"),

        '1' => ("█", "█", "▀"),

        '2' => ("▀▀█", "█▀▀", "▀▀▀"),

        '3' => ("▀▀█", "▀▀█", "▀▀▀"),

        '4' => ("█ █", "▀▀█", "  ▀"),

        '5' => ("█▀▀", "▀▀█", "▀▀▀"),

        '6' => ("█▀▀", "█▀█", "▀▀▀"),

        '7' => ("▀▀█", "  █", "  ▀"),

        '8' => ("█▀█", "█▀█", "▀▀▀"),

        '9' => ("█▀█", "▀▀█", "▀▀▀"),

        ':' => ("▄", "▄", " "),

        _   => ("   ", "   ", "   "),
    }
}

fn render_time_display(time_str: &str) -> (String, String, String) {
    let mut top = String::new();
    let mut mid = String::new();
    let mut bot = String::new();

    for (i, c) in time_str.chars().enumerate() {
        if i > 0 {
            top.push(' ');
            mid.push(' ');
            bot.push(' ');
        }
        let (t, m, b) = get_big_glyph(c);
        top.push_str(t);
        mid.push_str(m);
        bot.push_str(b);
    }

    (top, mid, bot)
}

fn draw_timer_placeholder(
    frame: &mut Frame,
    area: Rect,
) {
    let vertical = Layout::vertical([
        Constraint::Length(9), // Bumped to 9 to give ample vertical padding for 3 lines
    ])
    .flex(Flex::Center)
    .split(area);

    let horizontal = Layout::horizontal([
        Constraint::Length(34),
    ])
    .flex(Flex::Center)
    .split(vertical[0]);

    let timer_area = horizontal[0];

    let timer_block = Block::bordered()
        .border_style(Style::default().fg(Color::White))
        .padding(Padding::new(0, 0, 1, 0));

    let inner = timer_block.inner(timer_area);

    frame.render_widget(timer_block, timer_area);

    let time_display = "00:00:00"; 
    let (top_line, mid_line, bot_line) = render_time_display(time_display);

    let text = vec![
        Line::from(Span::styled("Add timer", Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(Span::styled(top_line, Style::default().fg(Color::White))),
        Line::from(Span::styled(mid_line, Style::default().fg(Color::White))),
        Line::from(Span::styled(bot_line, Style::default().fg(Color::White))),
        Line::from(Span::styled("", Style::default().fg(Color::White))),
    ];

    let paragraph = Paragraph::new(text).alignment(Alignment::Center);

    frame.render_widget(paragraph, inner);
}
