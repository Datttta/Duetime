use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex},
    widgets::{Clear, Block, Paragraph, Padding, Wrap},
    text::{Line},
    Frame
};

use crate::app::{App, Popup};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Item info")
        .padding(Padding::new(1,1,0,0));

    frame.render_widget(&block, area);

    fn centered_rect(frame: &mut Frame) -> Rect {
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
    
    if let Some(index) = app.inbox_table_state.selected() {
        let item = &app.inbox_items[index];

        let priority = app.priority;
        
        let paragraph = Paragraph::new(vec![
            Line::from("Plan:"),
            Line::from(item.input.as_str()), 
            Line::from(" "),
            Line::from(format!("Priority: {:?}", priority)),
        ])
        .wrap(Wrap { trim: false });
    
        frame.render_widget(paragraph, inner);
    }
}

fn copy_input(app: &mut App) {
    let Some(index) = app.inbox_table_state.selected() else {
        app.set_status_message(
            "No inbox item selected".to_string()
        );

        log::warn!("Could not copy inbox item: no item selected");
        return;
    };

    let text = app.inbox_items[index].input.clone();

    let Some(clipboard) = app.clipboard.as_mut() else {
        app.set_status_message(
            "Clipboard unavailable".to_string()
        );

        log::error!("Could not copy inbox item: clipboard unavailable");
        return;
    };

    match clipboard.set_text(text.clone()) {
        Ok(()) => {
            log::debug!("Copied inbox item to clipboard: {:?}", text);

            match clipboard.get_text() {
                Ok(copied) => {
                    log::debug!("Clipboard read-back: {:?}", copied);

                    app.set_status_message(
                        "Copied to clipboard".to_string()
                    );
                }

                Err(error) => {
                    log::error!(
                        "Clipboard write succeeded, but read-back failed: {}",
                        error
                    );

                    app.set_status_message(
                        "Copied, but clipboard could not be verified".to_string()
                    );
                }
            }
        }

        Err(error) => {
            log::error!("Failed to copy inbox item: {}", error);

            app.set_status_message(
                format!("Copy failed: {}", error)
            );
        }
    }
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    log::debug!("inbox_item_info received key: {:?}", key);

    match key.code {
        KeyCode::Char('c') => {
            log::debug!("C received");

            if app.pending_command == Some('c') {
                log::debug!("CC received, copying");
                copy_input(app);
                app.pending_command = None;
            } else {
                app.pending_command = Some('c');
            }
        }

        KeyCode::Char('q') => {
            app.popup = Popup::None;
        }

        _ => {}
    }
}
