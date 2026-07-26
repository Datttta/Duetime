use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, Popup},
};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex},
    widgets::{Clear, Block},
    Frame
};

pub fn draw(frame: &mut Frame, _app: &App) {
    let area = centered_rect(frame);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("New Preset");

    frame.render_widget(block, area);

    fn centered_rect(frame: &Frame) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(18),
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(50)
        ])
        .flex(Flex::Center)
        .split(vertical[0]);
        
        horizontal[0]
    }
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        
        KeyCode::Esc => {
            app.popup = Popup::None
        }

        _ => {}
    }
}

