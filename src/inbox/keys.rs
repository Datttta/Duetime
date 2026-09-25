use crate::{
    app::App,
    navigation::vim_navigation,
    inbox::keys::vim_navigation::NavigationMode,
    Panel,
};

//use log::info;
use super::actions;

use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    // Search typing mode
    if app.n_mode == NavigationMode::SearchInbox {
        actions::handle_search_input(app, key);
        return;
    }

    // Search navigation mode
    if app.n_mode == NavigationMode::SearchInboxNavigation {
        actions::handle_search_navigation(app, key);
        return;
    }

    // Start search
    if app.n_mode == NavigationMode::Normal
        && app.focused_panel == Panel::Inbox
        && key.code == KeyCode::Char('/')
    {
        app.inbox_search.clear();
        app.inbox_search_matches.clear();
        app.inbox_search_match = 0;
        app.n_mode = NavigationMode::SearchInbox;
        app.pending_command = None;
        return;
    }

    let mut selected = app.inbox_tasks_table_state.selected();

    let handled = vim_navigation::handle(
        key,
        &mut app.pending_command,
        &mut selected,
        app.inbox_items.len(),
        &mut app.n_mode,
        &mut app.n_visual_start,
    );

    app.inbox_tasks_table_state.select(selected);

    if handled {
        return;
    }

    match key.code {
        KeyCode::Char('a') => {
            actions::inbox_item_add_popup(app);
        }

        KeyCode::Char('i') => {
            actions::inbox_item_info(app);
        }

        KeyCode::Char('e') => {
            actions::edit_inbox_item(app);
        }

        KeyCode::Char('y') => {
            app.copy_inbox_input();
        }

        KeyCode::Char('d') => {
            if app.pending_command == Some('d') {
                actions::delete_inbox_item(app);
                app.pending_command = None;
            } else {
                app.pending_command = Some('d');
            }
        }

        _ => {
            app.pending_command = None;
        }
    }
}
