use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, Padding, Paragraph},
    Frame
};

use crate::{
    ui::widgets::input,
    vim_text::{InputMode, InputResult},
    app::{App, Popup, InboxPopup},
    models::Plan,
    keys_help,
};

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
            Constraint::Length(36)
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
        .title("Add Plan")
        .padding(Padding::new(1, 1, 0, 0));

    frame.render_widget(&block, area);
    
    let inner = block.inner(area);
    let name_input = Layout::horizontal([
        Constraint::Length(27),
    ])
    .flex(Flex::Center)
    .split(inner);

    input::draw(
        frame,
        name_input[0],
        &app.plan_name,
        "Task name",
        true,
        app.mode,
    );
}

fn save_plan (app: &mut App) {
    match app.popup {
        Popup::Inbox(InboxPopup::AddPlan) => {
            let id = app.next_id;
            app.next_id += 1;

            app.inbox.push(Plan {
                id, name: app.plan_name.text.clone(),
            });
            
            if app.plans_state.selected().is_none() && !app.inbox.is_empty() {
                app.plans_state.select(Some(0));
            }
        }

        //Popup::Inbox(InboxPopup::EditKnownTask(index)) => {
        //    if let Some(task) = app.known_tasks.get_mut(index) {
        //        task.name = app.known_task_name.text.clone();
        //    }
        //}

        _ => return,
    }

    crate::storage_inbox::save_inbox(&app.inbox).unwrap();

    app.known_task_name.clear();
    app.popup = Popup::None;
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    if app.plan_name.handle_key(key, &mut app.mode, 22) != InputResult::Ignored {
        return;
    }

    match key.code {
        KeyCode::Enter => {
            save_plan(app);
        }

        KeyCode::Char('q') => {
            app.popup = Popup::None;
        }
 
        _ => {}
    }
}
