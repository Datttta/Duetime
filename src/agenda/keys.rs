use crate::{
    agenda::keys::vim_navigation::NavigationMode,
    app::App,
    navigation::vim_navigation,
    search,
    Panel,
};

use super::actions;
use super::ui::snap_agenda_selection;

use chrono::Duration;
use crossterm::event::{KeyCode, KeyEvent};
//use log::info;

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    // --------------------------------------------------
    // Search typing mode
    // --------------------------------------------------

    if app.n_mode == NavigationMode::Search {
        match search::handle_search_input(
            &mut app.agenda_search,
            key,
        ) {
            search::SearchInputResult::Continue => {
                actions::search_agenda(app);
            }

            search::SearchInputResult::Navigate => {
                app.n_mode = NavigationMode::SearchNavigation;
            }

            search::SearchInputResult::Cancel => {
                search::clear(&mut app.agenda_search);

                app.n_mode = NavigationMode::Normal;
                app.pending_command = None;
            }
        }

        return;
    }

    // --------------------------------------------------
    // Search navigation mode
    // --------------------------------------------------

    if app.n_mode == NavigationMode::SearchNavigation {
        match search::handle_search_navigation(
            &mut app.agenda_search,
            key,
        ) {
            search::SearchNavigationResult::Continue => {
                if let Some(&index) = app
                    .agenda_search
                    .matches
                    .get(app.agenda_search.current_match)
                {
                    app.agenda_table_state.select(Some(index));
                }
            }

            search::SearchNavigationResult::Cancel => {
                app.n_mode = NavigationMode::Normal;
                app.pending_command = None;
            }
        }

        return;
    }

    // --------------------------------------------------
    // Start search
    // --------------------------------------------------

    if app.n_mode == NavigationMode::Normal
        && app.focused_panel == Panel::Agenda
        && key.code == KeyCode::Char('/')
    {
        search::clear(&mut app.agenda_search);

        app.n_mode = NavigationMode::Search;
        app.pending_command = None;

        return;
    }

    // --------------------------------------------------
    // Normal Agenda navigation
    // --------------------------------------------------

    let mut selected = app.agenda_table_state.selected();

    let today = chrono::Local::now().date_naive();
    let max_date = today + Duration::days(30);

    let visible_indices: Vec<usize> = (0..app.events.len())
        .filter(|&i| {
            let date = app.events[i].date;
            date >= today && date <= max_date
        })
        .collect();

    let (today_indices, upcoming_indices): (Vec<usize>, Vec<usize>) =
        visible_indices
            .into_iter()
            .partition(|&i| app.events[i].date == today);

    let today_count = today_indices.len();
    let upcoming_count = upcoming_indices.len();

    // Today header + today events + spacer + upcoming header + upcoming events
    let total_rows = today_count + upcoming_count + 3;

    let handled = vim_navigation::handle(
        key,
        &mut app.pending_command,
        &mut selected,
        total_rows,
        &mut app.n_mode,
        &mut app.n_visual_start,
    );

    app.agenda_table_state.select(selected);

    if handled {
        snap_agenda_selection(
            app,
            total_rows,
            today_count,
            key,
        );
    }

    match key.code {
        KeyCode::Char('a') => {
            actions::add_event(app);
        }

        KeyCode::Char('e') => {
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
                app.pending_command = Some('d');
            }
        }

        _ => {}
    }
}
