use crossterm::event::{KeyCode, KeyEvent};

use crate::vim_navigation;
use crate::{
    app::{App, Popup},
};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex},
    widgets::{Clear, Block, List, ListItem, Padding},
    text::Line,
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

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Presets")
        .padding(Padding::new(1,1,0,0));

    frame.render_widget(&block, area);

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

    let presets: Vec<ListItem> = app
        .presets
        .iter()
        .map(|preset| {
            ListItem::new(Line::from(format!(
                        "{} ({} tasks)",
                        preset.name,
                        preset.tasks.len(),
            )))
        })
        .collect();

    let list = List::new(presets.clone())
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, block.inner(area), &mut app.preset_state);
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    let mut selected = app.preset_state.selected();

    if vim_navigation::handle(
        key,
        &mut app.pending_command,
        &mut selected,
        app.presets.len(),
    ) {
        app.preset_state.select(selected);
        return;
    }

    match key.code {

        KeyCode::Char('n') => {
            app.popup = Popup::NewPreset;
        }

        KeyCode::Esc => {
            app.popup = Popup::None
        }

        _ => {}
    }
}
