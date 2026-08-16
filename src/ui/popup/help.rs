use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Padding},
    Frame,
};

use crate::app::{App, Popup};

pub fn draw(frame: &mut Frame, _app: &mut App) {
    let area = centered_rect(frame, 35, 22);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title(" Help ")
        .padding(Padding::new(2,2,1,1))
        .border_style(Style::default().fg(Color::White));

    let lines = vec![
        Line::from("Navigation"),
        Line::from(""),
        Line::from(vec![
            Span::styled("j -", Style::default().fg(Color::Yellow)),
            Span::raw(" Move down"),
        ]),
        Line::from(vec![
            Span::styled("k -", Style::default().fg(Color::Yellow)),
            Span::raw(" Move up"),
        ]),
        Line::from(vec![
            Span::styled("gg -", Style::default().fg(Color::Yellow)),
            Span::raw(" Go to first task"),
        ]),
        Line::from(vec![
            Span::styled("G -", Style::default().fg(Color::Yellow)),
            Span::raw(" Go to last task"),
        ]),
        Line::from(vec![
            Span::styled("v / V -", Style::default().fg(Color::Yellow)),
            Span::raw(" Visual mode"),
        ]),
        Line::from(""),
        Line::from("Tasks"),
        Line::from(""),
        Line::from(vec![
            Span::styled("t -", Style::default().fg(Color::Yellow)),
            Span::raw(" Add task"),
        ]),
        Line::from(vec![
            Span::styled("e -", Style::default().fg(Color::Yellow)),
            Span::raw(" Edit task"),
        ]),
        Line::from(vec![
            Span::styled("dd -", Style::default().fg(Color::Yellow)),
            Span::raw(" Delete task"),
        ]),
        Line::from(vec![
            Span::styled("x -", Style::default().fg(Color::Yellow)),
            Span::raw(" Move task"),
        ]),
        Line::from(vec![
            Span::styled("s -", Style::default().fg(Color::Yellow)),
            Span::raw(" Start/stop task"),
        ]),
        Line::from(vec![
            Span::styled("c -", Style::default().fg(Color::Yellow)),
            Span::raw(" Complete task"),
        ]),
        Line::from(vec![
            Span::styled("r -", Style::default().fg(Color::Yellow)),
            Span::raw(" Reset task"),
        ]),
        Line::from(""),
        Line::from("Other"),
        Line::from(""),
        Line::from(vec![
            Span::styled("P", Style::default().fg(Color::Yellow)),
            Span::raw("         Presets"),
        ]),
        Line::from(vec![
            Span::styled("Ctrl+l", Style::default().fg(Color::Yellow)),
            Span::raw("     Known tasks"),
        ]),
        Line::from(vec![
            Span::styled("i", Style::default().fg(Color::Yellow)),
            Span::raw("         Task information"),
        ]),
        Line::from(vec![
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw("         Quit"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw("       Close / cancel"),
        ]),
    ];

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
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
            //mode down
        }

        KeyCode::Char('k') => {
            //move up
        }

        KeyCode::Esc
        | KeyCode::Char('q')
        | KeyCode::Char('h')
        | KeyCode::Char('?') => {
            app.popup = Popup::None;
        }

        _ => {}
    }
}
