use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Default)]
pub struct SearchState {
    pub query: String,
    pub matches: Vec<usize>,
    pub current_match: usize,
}

#[derive(Debug)]
pub enum SearchNavigationResult {
    Continue,
    Cancel,
}

#[derive(Debug)]
pub enum SearchInputResult {
    Continue,
    Navigate,
    Cancel,
}

pub fn handle_search_navigation(
    search: &mut SearchState,
    key: KeyEvent,
) -> SearchNavigationResult {
    match key.code {
        KeyCode::Char('n') => {
            next_match(search);
            SearchNavigationResult::Continue
        }

        KeyCode::Char('N') => {
            previous_match(search);
            SearchNavigationResult::Continue
        }

        KeyCode::Esc | KeyCode::Char('q') => {
            clear(search);
            SearchNavigationResult::Cancel
        }

        _ => SearchNavigationResult::Continue,
    }
}

pub fn search_items<T, F>(
    search: &mut SearchState,
    items: &[T],
    matches: F,
) where
    F: Fn(&T, &str) -> bool,
{
    let query = search.query.trim().to_lowercase();

    search.matches.clear();
    search.current_match = 0;

    if query.is_empty() {
        return;
    }

    search.matches = items
        .iter()
        .enumerate()
        .filter(|(_, item)| matches(item, &query))
        .map(|(index, _)| index)
        .collect();
}

pub fn handle_search_input(
    search: &mut SearchState,
    key: KeyEvent,
) -> SearchInputResult {
    match key.code {
        KeyCode::Char(c) => {
            search.query.push(c);
            SearchInputResult::Continue
        }

        KeyCode::Backspace => {
            search.query.pop();
            SearchInputResult::Continue
        }

        KeyCode::Enter => {
            if search.matches.is_empty() {
                SearchInputResult::Continue
            } else {
                SearchInputResult::Navigate
            }
        }

        KeyCode::Esc => SearchInputResult::Cancel,

        _ => SearchInputResult::Continue,
    }
}

pub fn next_match(search: &mut SearchState) {
    if search.matches.is_empty() {
        return;
    }

    search.current_match =
        (search.current_match + 1) % search.matches.len();
}

pub fn previous_match(search: &mut SearchState) {
    if search.matches.is_empty() {
        return;
    }

    if search.current_match == 0 {
        search.current_match = search.matches.len() - 1;
    } else {
        search.current_match -= 1;
    }
}

pub fn clear(search: &mut SearchState) {
    search.query.clear();
    search.matches.clear();
    search.current_match = 0;
}
