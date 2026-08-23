use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, Padding, Paragraph},
    Frame
};

use crate::{
    ui::widgets::input,
    vim_text::{InputResult},
    app::{App, Popup, InboxPopup, InboxSelectedInput},
    models::InboxItem,
    inbox::InboxItemInfo,
    keys_help,
};

pub const INBOX_ITEM_INPUT_WIDTH: u16 = 34;

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
            Constraint::Length(INBOX_ITEM_INPUT_WIDTH + 2)
        ])
        .flex(Flex::Center)
        .split(vertical[0]);
        
        let keys_help = Paragraph::new(keys_help::keys(app))
            .alignment(Alignment::Center);
        frame.render_widget(keys_help, vertical[1]);
        
        horizontal[0]
    }

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Add Item")
        .padding(Padding::new(1, 1, 0, 0));

    frame.render_widget(&block, area);
    
    let inner = block.inner(area);
    let item_input = Layout::horizontal([
        Constraint::Length(INBOX_ITEM_INPUT_WIDTH),
    ])
    .flex(Flex::Center)
    .split(inner);

    input::draw(
        frame,
        item_input[0],
        &app.inbox_item,
        "add item",
        true,
        app.mode,
    );
}

fn save_inbox_item (app: &mut App) {
    match app.popup {
        Popup::Inbox(InboxPopup::AddInboxItem) => {
            let item = InboxItemInfo {
                    input: app.inbox_item.text.clone(),
                    priority: app.planned_start.text.clone(),
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
                    item.priority = app.planned_start.text.clone();
                }
            }
        }

        _ => {},
    }

    crate::storage_inbox::save_inbox(&app.inbox_items).unwrap();

    app.known_task_name.clear();
    app.popup = Popup::None;
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    let result = match app.inbox_selected_input {
        InboxSelectedInput::InboxItemInput => {
            app.inbox_item.handle_key(key, &mut app.mode, usize::MAX)
        }
    };

    match key.code {
        KeyCode::Enter => {
            save_inbox_item(app);
        }

        KeyCode::Char('q') => {
            app.popup = Popup::None;
        }
 
        _ => {}
    }
}
