use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, Popup},
};

use ratatui::{
    layout::{Rect, Alignment, Constraint, Layout, Flex},
    widgets::{Clear, Block},
    Frame
};

pub struct Preset {
    pub id: u64,
    pub name: String,
    pub tasks: Vec<TaskTemplate>,
}

pub struct TaskTemplate {
    pub id: u64,
    pub name: String,
    pub planned_start: Option<String>,
    pub planned_end: Option<String>,
}

pub fn draw(frame: &mut Frame, _app: &App) {
    let area = centered_rect(frame);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Presets");

    frame.render_widget(block, area);

    fn centered_rect(frame: &Frame) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(18),
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(40)
        ])
        .flex(Flex::Center)
        .split(vertical[0]);
        
        horizontal[0]
    }
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    match key.code {

        KeyCode::'n' => {
            // add new presets
        }

        KeyCode::Esc => {
            app.popup = Popup::None
        }

        _ => {}
    }
}
