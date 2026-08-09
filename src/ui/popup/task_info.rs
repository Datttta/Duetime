use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex},
    widgets::{Clear, Block, Paragraph, Padding},
    text::{Line},
    Frame
};

use crate::app::{App, Popup};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame, app);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Task info")
        .padding(Padding::new(1,1,0,0));

    frame.render_widget(&block, area);

    fn centered_rect(frame: &mut Frame, app: &mut App) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(23),
            Constraint::Length(1), // keys help
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
    
    if let Some(index) = app.table_state.selected() {
        let task = &app.tasks[index];

        let in_known_tasks = app.known_tasks
            .iter()
            .any(|known| known.name == task.name);

        let task_preset: Vec<&str> = app.presets
            .iter()
            .filter(|preset| {
                preset.tasks
                    .iter()
                    .any(|preset_task| preset_task.name == task.name)
            })
            .map(|preset| preset.name.as_str())
            .collect();

        let paragraph = Paragraph::new(vec![
            Line::from("Task name:"),
            Line::from(task.name.as_str()), 
            Line::from(" "),
            Line::from(format!("In known tasks: {}", in_known_tasks)),
            Line::from(format!(
                "Preset: {}",
                if task_preset.is_empty() {
                    "None".to_string()
                } else {
                    task_preset.join(", ")
                }
            ))
        ]);
    
        frame.render_widget(paragraph, inner);
    }
}

pub fn handle_keys (app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => {
            app.popup = Popup::None;
        }

        _ => {}
    }
}
