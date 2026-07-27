use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, Popup, TaskDestination, SelectedInput},
};
use crate::ui::widgets::input;
use crate::vim_text::InputMode;

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex},
    widgets::{Clear, Block, List, ListItem, Padding},
    text::Line,
    Frame
};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = centered_rect(frame);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("New Preset")
        .padding(Padding::new(1,0,0,0));

    frame.render_widget(&block, area);

    fn centered_rect(frame: &Frame) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(18),
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(48)
        ])
        .flex(Flex::Center)
        .split(vertical[0]);
        
        horizontal[0]
    }

    let chunks = Layout::vertical([
        Constraint::Length(3), // preset name
        Constraint::Min(0),    // task list
    ])
    .split(block.inner(area));

    let tasks: Vec<ListItem> = app
        .preset_tasks
        .iter()
        .map(|task| {
            ListItem::new(Line::from(format!(
                "{} {} - {}",
                task.name,
                task.planned_start.as_deref().unwrap_or(""),
                task.planned_end.as_deref().unwrap_or("")
            )))
        })
        .collect();

    input::draw(
        frame,
        chunks[0],
        &app.preset_name,
        "Preset name",
        true, // or compare against a SelectedPresetInput enum later
        app.mode,
    );

    let list = List::new(tasks);

    frame.render_widget(list, chunks[1])
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    match key.code {

        KeyCode::Char('a') if app.mode == InputMode::Normal => {
            app.popup = Popup::AddTask;

            app.task_destination = TaskDestination::Preset;

            app.task_name.clear();
            app.planned_start.clear();
            app.planned_end.clear();

            app.selected_input = SelectedInput::TaskName;
            app.mode = InputMode::Insert;
        }
        
        KeyCode::Esc if app.mode == InputMode::Normal => {
            app.popup = Popup::None
        }

        _ => {
            app.preset_name.handle_key(key, &mut app.mode, 40);
        }
    }
}

