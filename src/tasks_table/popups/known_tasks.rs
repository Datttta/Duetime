use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, List, ListItem, Padding, Paragraph},
    text::Line,
    Frame
};

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    input::vim_navigation,
    ui::widgets::keys_help,
    app::{App, Popup},
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame, app);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Known Tasks")
        .padding(Padding::new(1,1,0,0));

    frame.render_widget(&block, area);

    fn centered_rect(frame: &mut Frame, app: &mut App) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(21),
            Constraint::Length(1), // keys_help
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal_box = Layout::horizontal([
            Constraint::Length(43)
        ])
        .flex(Flex::Center)
        .split(vertical[0]);
        
        let horizontal_help = Layout::horizontal([
            Constraint::Length(70)
        ])
        .flex(Flex::Center)
        .split(vertical[1]);
    
        frame.render_widget(Clear, horizontal_help[0]);

        let keys_help = Paragraph::new(keys_help::keys(app))
            .alignment(Alignment::Center);
            
        frame.render_widget(keys_help, horizontal_help[0]);
        
        horizontal_box[0]
    }

    let known_tasks: Vec<ListItem> = app.known_tasks.iter()
        .map(|task| {
            ListItem::new(Line::from(format!("{}", task.name)))
        })
        .collect();

    let list = List::new(known_tasks.clone())
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, block.inner(area), &mut app.known_tasks_state);
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    let mut selected = app.known_tasks_state.selected();

    if vim_navigation::handle(
        key,
        &mut app.pending_command,
        &mut selected,
        app.known_tasks.len(),
        &mut app.n_mode,
        &mut app.n_visual_start,
    ) {
        app.known_tasks_state.select(selected);
        return;
    }

    match key.code {
        KeyCode::Char('a') => {
            app.add_known_task();
        }

        KeyCode::Char('e') => {
            app.edit_known_task();
            return;
        }

        KeyCode::Char('d') => {
            if app.pending_command == Some('d') {
                app.delete_known_task();
            } else {
                app.pending_command = Some('d');
            }
        }

        KeyCode::Char('q') => {
            app.popup = Popup::None
        }

        _ => {}
    }
}
