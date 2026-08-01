use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, List, ListItem, Padding, Paragraph},
    text::Line,
    Frame
};

use crossterm::event::{KeyCode, KeyEvent};

use crate::ui::widgets::input;
use crate::vim_text::InputMode;
use crate::vim_navigation;
use crate::keys_help;
use crate::app::{App, Popup};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame, app);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Add task name")
        .padding(Padding::new(1,1,0,0));

    frame.render_widget(&block, area);

    fn centered_rect(frame: &mut Frame, app: &mut App) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(10),
            Constraint::Length(1), // keys_help
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(27)
        ])
        .flex(Flex::Center)
        .split(vertical[0]);
        
        let keys_help = Paragraph::new(keys_help::keys(AddTask)) // AddTask has the needed keys
            .alignment(Alignment::Center);
        frame.render_widget(keys_help, vertical[1]);
        
        horizontal[0]
    }
    
    input::draw(
        frame,
        vertical[0],
        &app.task_name,
        "Task name",
        true,
        app.mode,
    );
}

fn save_known_task(app: &mut App) {
    match app.popup {
        Popup::AddKnownTask => {
            // somehting
        }

        Popup::EditKnownTask => {
            // somehting
        }

        Popup::None => {}
    }

    app.popup = Popup::None;
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
        KeyCode::Enter => {
            save_known_task(app);
        }

        KeyCode::Esc => {
            app.popup = Popup::KnownTasks
        }

        _ => {}
    }
}
