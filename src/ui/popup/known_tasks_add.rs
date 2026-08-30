use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, Padding, Paragraph},
    Frame
};

use crate::{
    ui::widgets::input,
    vim_text::{InputMode, InputResult},
    app::{App, Popup, TasksPopup},
    models::KnownTask,
    keys_help,
};

const KNOWN_TASK_NAME_WIDTH: u16 = 43;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame, app);

    frame.render_widget(Clear, area);

    fn centered_rect(frame: &mut Frame, app: &mut App) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(5),
            Constraint::Length(1), // keys_help
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(45)
        ])
        .flex(Flex::Center)
        .split(vertical[0]);
        
        let keys_help = Paragraph::new(keys_help::keys(app))
            .alignment(Alignment::Center);
        frame.render_widget(keys_help, vertical[1]);
        
        horizontal[0]
    }

    let area = centered_rect(frame, app);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Add task name")
        .padding(Padding::new(1, 1, 0, 0));

    frame.render_widget(&block, area);
    
    let inner = block.inner(area);
    let name_input = Layout::horizontal([
        Constraint::Length(KNOWN_TASK_NAME_WIDTH),
    ])
    .flex(Flex::Center)
    .split(inner);

    input::draw(
        frame,
        name_input[0],
        &app.known_task_name,
        "Task name",
        true,
        app.mode,
    );
}

pub fn save_known_task(app: &mut App) {
    match app.popup {
        Popup::Tasks(TasksPopup::AddKnownTask) => {
            let id = app.next_id;
            app.next_id += 1;

            app.known_tasks.push(KnownTask {
                id, name: app.known_task_name.text.clone(),
            });
            
            if app.known_tasks_state.selected().is_none() && !app.known_tasks.is_empty() {
                app.known_tasks_state.select(Some(0));
            }
        }

        Popup::Tasks(TasksPopup::EditKnownTask(index)) => {
            if let Some(task) = app.known_tasks.get_mut(index) {
                task.name = app.known_task_name.text.clone();
            }
        }

        _ => return,
    }

    crate::storage::known_tasks::save_known_tasks(&app.known_tasks).unwrap();

    app.known_task_name.clear();
    app.popup = Popup::Tasks(TasksPopup::KnownTasks)
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    if app.known_task_name.handle_key(key, &mut app.mode, (KNOWN_TASK_NAME_WIDTH - 6).into()) != InputResult::Ignored {
        return;
    }

    match key.code {
        KeyCode::Enter => {
            save_known_task(app);
        }

        KeyCode::Esc => {
            if app.mode == InputMode::Normal {
                app.popup = Popup::Tasks(TasksPopup::KnownTasks)
            }
        }

        _ => {}
    }
}
