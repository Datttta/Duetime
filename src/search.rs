#[derive(Debug, Default)]
pub struct SearchState {
    pub query: String,
    pub matches: Vec<usize>,
    pub current_match: usize,
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
