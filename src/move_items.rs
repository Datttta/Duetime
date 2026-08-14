use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::{ListState, TableState};

pub trait Selectable {
    fn select(&mut self, index: Option<usize>);
}

impl Selectable for TableState {
    fn select(&mut self, index: Option<usize>) {
        TableState::select(self, index);
    }
}

impl Selectable for ListState {
    fn select(&mut self, index: Option<usize>) {
        ListState::select(self, index);
    }
}

#[derive(Default)]
pub struct MoveState {
    pub first: Option<usize>,
    pub last: Option<usize>,
    pub position: Option<usize>,
}

impl MoveState {
    pub fn is_moving(&self) -> bool {
        self.position.is_some()
    }
}

pub fn start(
    state: &mut MoveState,
    selected: Option<usize>,
    visual_start: Option<usize>,
    len: usize,
) {
    let Some(current) = selected else {
        return;
    };

    if len == 0 {
        return;
    }

    let (first, last) = if let Some(start) = visual_start {
        (start.min(current), start.max(current))
    } else {
        (current, current)
    };

    state.first = Some(first);
    state.last = Some(last);
    state.position = Some(last + 1);
}

pub fn handle_keys<T, S>(
    state: &mut MoveState,
    items: &mut Vec<T>,
    selection: &mut S,
    key: KeyEvent,
) -> bool
where
    S: Selectable,
{
    match key.code {
        KeyCode::Char('j') => {
            if let Some(position) = state.position {
                state.position = Some((position + 1).min(items.len()));
            }

            true
        }

        KeyCode::Char('k') => {
            if let Some(position) = state.position {
                state.position = Some(position.saturating_sub(1));
            }

            true
        }

        KeyCode::Enter => {
            finish(state, items, selection);
            true
        }

        KeyCode::Esc => {
            *state = MoveState::default();
            true
        }

        _ => false,
    }
}

pub fn move_items<T>(
    items: &mut Vec<T>,
    first: usize,
    last: usize,
    mut position: usize,
) -> usize {
    if first > last || last >= items.len() {
        return first;
    }

    let moved: Vec<T> = items.drain(first..=last).collect();

    if position > last {
        position -= moved.len();
    } else if position >= first {
        position = first;
    }

    position = position.min(items.len());

    for (offset, item) in moved.into_iter().enumerate() {
        items.insert(position + offset, item);
    }

    position
}

fn finish<T, S>(
    state: &mut MoveState,
    items: &mut Vec<T>,
    selection: &mut S,
)
where
    S: Selectable,
{
    let (Some(first), Some(last), Some(position)) =
        (state.first, state.last, state.position)
    else {
        return;
    };

    let new_position = move_items(
        items,
        first,
        last,
        position,
    );

    selection.select(Some(new_position));

    *state = MoveState::default();
}
