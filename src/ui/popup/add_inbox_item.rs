use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, Padding, Paragraph},
    text::{Line, Span},
    style::{Style, Color, Modifier},
    Frame
};

use crate::{
    ui::widgets::input,
    vim_text::{InputResult},
    app::{App, Popup, InboxPopup, InboxSelectedInput, Priority},
    models::InboxItem,
    inbox::InboxItemInfo,
    keys_help,
};

pub const INBOX_ITEM_INPUT_WIDTH: u16 = 34;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame, app);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Add Item")
        .padding(Padding::new(2, 2, 0, 0));

    let inner = block.inner(area);

    frame.render_widget(block, area);

    fn centered_rect(frame: &mut Frame, app: &mut App) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(8),
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(50),
        ])
        .flex(Flex::Center)
        .split(vertical[0]);
        
        horizontal[0]
    }

    let vertical = Layout::vertical([
        Constraint::Length(10), // input height
        Constraint::Length(1), // keys_help
    ])
    .split(inner);

    let item_features = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(1), // spacing
        Constraint::Length(1), // priority
    ])
    .flex(Flex::Center)
    .split(vertical[0]);

    let keys_help = Paragraph::new(keys_help::keys(app))
            .alignment(Alignment::Center);
    frame.render_widget(keys_help, vertical[1]);

    input::draw(
        frame,
        item_features[0],
        &app.inbox_item,
        "plan...",
        true,
        app.mode,
    );

    let priority_line = Line::from(vec![
        Span::raw("Priority: "),

        if app.priority == Priority::Low {
            Span::styled(
                " Low ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::raw(" Low ")
        },

        if app.priority == Priority::Medium {
            Span::styled(
                " Medium ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::raw(" Medium ")
        },

        if app.priority == Priority::High {
            Span::styled(
                " High ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::raw(" High ")
        },
    ]);

    frame.render_widget(
        Paragraph::new(priority_line),
        item_features[2],
    );
}

fn save_inbox_item (app: &mut App) {
    match app.popup {
        Popup::Inbox(InboxPopup::AddInboxItem) => {
            let item = InboxItemInfo {
                    input: app.inbox_item.text.clone(),
                    ..Default::default()
                };

                let position = match app.table_state.selected() {
                    Some(index) => index + 1,
                    None => 0
                };
                
                app.inbox_items.insert(position.min(app.inbox_items.len()), item);

                app.inbox_table_state.select(Some(position.min(app.inbox_items.len() - 1)));
        }
        
        Popup::Inbox(InboxPopup::EditInboxItem) => {
            if let Some(index) = app.inbox_table_state.selected() {
                if let Some(item) = app.inbox_items.get_mut(index) {
                    item.input = app.inbox_item.text.clone();
                }
            }
        }

        _ => {},
    }

    crate::storage_inbox::save_inbox(&app.inbox_items).unwrap();

    app.known_task_name.clear();
    app.popup = Popup::None;
}

fn next_priority(priority: Priority) -> Priority {
    match priority {
        Priority::Low => Priority::Medium,
        Priority::Medium => Priority::High,
        Priority::High => Priority::Low,
    }
}

fn previous_priority(priority: Priority) -> Priority {
    match priority {
        Priority::Low => Priority::High,
        Priority::Medium => Priority::Low,
        Priority::High => Priority::Medium,
    }
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    match app.inbox_selected_input {
        InboxSelectedInput::InboxItemInput => {
            let result = app.inbox_item.handle_key(
                key,
                &mut app.mode,
                usize::MAX,
            );

            match result {
                InputResult::Consumed => return,
                InputResult::Ignored => {}
                InputResult::TextChanged => {}
            }
        }

        InboxSelectedInput::Priority => {
            match key.code {
                KeyCode::Char('h') | KeyCode::Left => {
                    app.priority = previous_priority(app.priority);
                    return;
                }

                KeyCode::Char('l') | KeyCode::Right => {
                    app.priority = next_priority(app.priority);
                    return;
                }

                _ => {}
            }
        }
    }

    match key.code {
        KeyCode::Tab => {
            app.inbox_selected_input = match app.inbox_selected_input {
                InboxSelectedInput::InboxItemInput => {
                    InboxSelectedInput::Priority
                }

                InboxSelectedInput::Priority => {
                    InboxSelectedInput::InboxItemInput
                }
            };
        }

        KeyCode::Enter => {
            save_inbox_item(app);
        }

        KeyCode::Char('q') => {
            app.popup = Popup::None;
        }
 
        _ => {}
    }
}
