use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, List, ListItem, Padding, Paragraph},
    text::Line,
    Frame
};

use crate::{
    app::{App, Popup},
    keys_help,
    vim_navigation,
};

use crossterm::event::{KeyCode, KeyEvent};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame, app);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Presets")
        .padding(Padding::new(1,1,1,0));

    frame.render_widget(&block, area);

    fn centered_rect(frame: &mut Frame, app: &mut App) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(18),
            Constraint::Length(1), // keys_help
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(40)
        ])
        .flex(Flex::Center)
        .split(vertical[0]);
        
        let keys_help = Paragraph::new(keys_help::keys(app))
            .alignment(Alignment::Center);
        frame.render_widget(keys_help, vertical[1]);
        
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
        &mut app.tasks_navigation_mode,
        &mut app.tasks_visual_start,
    ) {
        app.preset_state.select(selected);
        return;
    }

    match key.code {

        KeyCode::Char('a') => {
            app.create_preset();
        }

        KeyCode::Char('e') => {
            app.edit_preset();
        }

        KeyCode::Char('d') => {
            if app.pending_command == Some('d') {
                app.delete_preset();
            } else {
                app.pending_command = Some('d')
            }
        }

        KeyCode::Enter => {
            app.apply_preset();
        }

        KeyCode::Char('q') => {
            app.popup = Popup::None
        }

        _ => {}
    }
}
