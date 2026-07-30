use ratatui::{
    layout::{Rect, Constraint, Layout, Flex},
    widgets::{Clear, Block, List, ListItem, Padding},
    text::Line,
    Frame
};
use serde::{Deserialize, Serialize};

use crossterm::event::{KeyCode, KeyEvent};

use crate::vim_navigation;
use crate::vim_text::InputMode;
use crate::tasks::TaskInfo;
use crate::app::{App, Popup};
use crate::presets::{Preset, TaskTemplate};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Presets")
        .padding(Padding::new(1,1,1,0));

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
            app.preset_name.clear();
            app.preset_tasks.clear();

            app.mode = InputMode::Insert;
            
            app.popup = Popup::NewPreset;
        }

        KeyCode::Char('e') => {
            // edit preset
            if let Some(index) = app.preset_state.selected() {
                let preset = &app.presets[index];

                app.edit_preset = Some(index);

                app.preset_name.text = preset.name.clone();
                app.preset_name.cursor = app.preset_name.text.len();

                app.preset_tasks = preset.tasks.clone();

                app.popup = Popup::NewPreset;
                app.mode = InputMode::Normal;
            }
        }

        KeyCode::Enter => {
            if let Some(index) = app.preset_state.selected() {
                // Set preset to main task table
                let preset = &app.presets[index];

                app.tasks = preset
                    .tasks
                    .iter()
                    .map(|task| TaskInfo {
                        name: task.name.clone(),
                        status: "PENDING".to_string(),
                        planned_start: task.planned_start.clone().unwrap_or_default(),
                        planned_end: task.planned_end.clone().unwrap_or_default(),
                        ..Default::default()
                    })
                    .collect();

                app.popup = Popup::None;
            }
        }

        KeyCode::Esc => {
            app.popup = Popup::None
        }

        _ => {}
    }
}
