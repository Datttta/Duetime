use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex},
    widgets::{Clear, Block, Paragraph, Padding, Wrap},
    text::{Line},
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
            Constraint::Length(12),
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

    let inner = block.inner(area);
    frame.render_widget(block, area);
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

