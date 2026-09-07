use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, Padding, Paragraph},
    Frame,
};

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    navigation::{
        vim_navigation::NavigationMode,
        vim_navigation,
    },

    app::{App, Popup},
    agenda::actions,
    keys_help,
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame, app);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("All Events")
        .padding(Padding::new(1,1,1,0));

    let inner = block.inner(area);

    frame.render_widget(&block, area);

    let event_indices: Vec<usize> = (0..app.events.len()).collect();

    let is_visual = app.n_mode == NavigationMode::Visual;

    crate::agenda::ui::draw_events(
        frame,
        inner,
        app,
        &event_indices,
        is_visual,
    );

    fn centered_rect(frame: &mut Frame, app: &mut App) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(21),
            Constraint::Length(1), // keys_help
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(85)
        ])
        .flex(Flex::Center)
        .split(vertical[0]);
        
        let keys_help = Paragraph::new(keys_help::keys(app)) 
            .alignment(Alignment::Center);
        frame.render_widget(keys_help, vertical[1]);
        
        horizontal[0]
    }
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    let mut selected = app.agenda_table_state.selected();

    let handled = vim_navigation::handle(
        key,
        &mut app.pending_command,
        &mut selected,
        app.events.len(),
        &mut app.n_mode,
        &mut app.n_visual_start,
    );

    app.agenda_table_state.select(selected);

    if handled {
        return;
    }

    match key.code {
        KeyCode::Char('a') => {
            app.last_popup = app.popup.clone();
            actions::add_event(app);
        }

        KeyCode::Char('e') => {
            app.last_popup = app.popup.clone();
            actions::edit_event(app);
        }
        
        KeyCode::Char('i') => {
            app.last_popup = app.popup.clone();
            actions::event_info(app);
        }
        
        KeyCode::Char('l') => {
            actions::all_events(app);
        }

        KeyCode::Char('d') => {
            if app.pending_command == Some('d') {
                actions::delete_event(app);
                app.pending_command = None;
            } else {
                app.pending_command = Some('d')
            }
        }
        
        KeyCode::Char('q') => {
            app.popup = Popup::None
        }

        _ => {}
    }
}

