use super::actions;
use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::App,
    navigation::vim_navigation,
    inbox::keys::vim_navigation::NavigationMode,
    Panel, search,
};

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    // --------------------------------------------------
    // Search typing mode
    // --------------------------------------------------

    if app.n_mode == NavigationMode::Search {
        match search::handle_search_input(
            &mut app.inbox_search,
            key,
        ) {
            search::SearchInputResult::Continue => {
                actions::search_inbox(app);
            }

            search::SearchInputResult::Navigate => {
                app.n_mode = NavigationMode::SearchNavigation;
            }

            search::SearchInputResult::Cancel => {
                search::clear(&mut app.inbox_search);
                
                app.search_panel = None;
                app.pending_command = None;
                app.n_mode = NavigationMode::Normal;
            }
        }

        return;
    }

    // --------------------------------------------------
    // Search navigation mode
    // --------------------------------------------------

    if app.n_mode == NavigationMode::SearchNavigation {
        match search::handle_search_navigation(
            &mut app.inbox_search,
            key,
        ) {
            search::SearchNavigationResult::Continue => {
                if let Some(&index) = app
                    .inbox_search
                    .matches
                    .get(app.inbox_search.current_match)
                {
                    app.inbox_tasks_table_state.select(Some(index));
                }
            }

            search::SearchNavigationResult::Cancel => {
                app.pending_command = None;
                app.n_mode = NavigationMode::Normal;
            }
        }

        return;
    }

    // --------------------------------------------------
    // Start search
    // --------------------------------------------------

    if app.n_mode == NavigationMode::Normal
        && app.focused_panel == Panel::Inbox
        && key.code == KeyCode::Char('/')
    {
        search::clear(&mut app.inbox_search);

        app.n_mode = NavigationMode::Search;
        app.search_panel = Some(Panel::Inbox);
        app.pending_command = None;

        return;
    }

    // --------------------------------------------------
    // Normal Vim navigation
    // --------------------------------------------------

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

    // --------------------------------------------------
    // Inbox actions
    // --------------------------------------------------

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
