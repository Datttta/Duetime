use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, Paragraph, Padding},
    text::{Line, Span},
    style::{Style},
    Frame
};

use crate::app::{App, Popup};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .padding(Padding::new(1,1,0,0));

    frame.render_widget(&block, area);

    fn centered_rect(frame: &mut Frame) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(8),
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(55),
        ])
        .flex(Flex::Center)
        .split(vertical[0]);
        
        horizontal[0]
    }

    let old_v_str = match &app.popup { Popup::Updated(v) => v.as_str(), _ => "unknown" };
    let new_v_str = self_update::cargo_crate_version!();

    let text_content = vec![
        Line::from(""),
        Line::from(""),
        Line::from("Duetime has been updated!"),
        Line::from(vec![
            Span::styled(" v", Style::default()),
            Span::styled(old_v_str, Style::default()),
            Span::styled(" -> ", Style::default()),
            Span::styled(" v", Style::default()),
            Span::styled(new_v_str, Style::default()),
            Span::styled(" ", Style::default()), // Subtle spacer
        ]),
    ];

    let text_paragraph = Paragraph::new(text_content)
        .alignment(Alignment::Center);

    let inner = block.inner(area);

    let vertical_center = Layout::vertical([
        Constraint::Length(6), // Height of our text block
    ])
    .flex(Flex::Center)
    .split(inner);

    frame.render_widget(text_paragraph, vertical_center[0]);
}

pub fn handle_keys (app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q')
        | KeyCode::Char(' ')
        | KeyCode::Enter
        | KeyCode::Esc  => {
            app.popup = Popup::None;
        }

        _ => {}
    }
}

