use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::{ListState, TableState};
use crate::App;

use log::info;

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
    pub target: Option<MoveTarget>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MoveTarget {
    Tasks,
    PresetTasks,
    KnownTasks,
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
    target: MoveTarget,
) {
    let Some(index) = selected else {
        return;
    };

    if len == 0 {
        return;
    }

    let (beginning_selected_row, end_selected_row) =
        if let Some(start) = visual_start {
            (start.min(index), start.max(index))
        } else {
            (index, index)
        };

    state.first = Some(beginning_selected_row);
    state.last = Some(end_selected_row);
    state.target = Some(target);

    if index == beginning_selected_row {
        if beginning_selected_row == 0 {
            state.position = Some(end_selected_row + 1);
        } else {
            state.position = Some(index);
        }
    } else {
        if end_selected_row == len - 1 {
            state.position = Some(beginning_selected_row);
        } else {
            state.position = Some(index + 1);
        }
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

fn move_down<T>(state: &mut MoveState, items: &[T]) -> bool {
    let (Some(first), Some(last), Some(position)) =
            (state.first, state.last, state.position)
        else {
            return false;
        };

        let last_item = items.len() - 1;

        let moving_len = last - first + 1;

        let mut next = position.saturating_add(1);

        if next > first && next <= last {
            if last_item == last {
                next -= 1;
                return false;
            }

            next = last + 1;
        }

        state.position = Some(next.min(items.len()));

        true
}

fn move_up<T>(state: &mut MoveState, items: &[T]) -> bool {
    let (Some(first), Some(last), Some(position)) =
            (state.first, state.last, state.position)
        else {
            return false;
        };

        let mut previous = position.saturating_sub(1);

        if previous >= first && previous <= last {
            if first == 0 {
                previous += 1;
                return false;
            }

            previous = first;
        }

        state.position = Some(previous);
        true
}

pub fn handle_keys<T, S>(
    state: &mut MoveState,
    items: &mut Vec<T>,
    selection: &mut S,
    pending_command: &mut Option<char>,
    key: KeyEvent,
) -> bool
where
    S: Selectable,
{
    match key.code {
        KeyCode::Char('j') => {
            move_down(state, items);
            true
        }

        KeyCode::Char('k') => {
            move_up(state, items);
            true
        }

        KeyCode::Char('G') => {
            state.position = Some(items.len());
            true 
        }

        KeyCode::Char('g') => {
            if *pending_command == Some('g') {
                state.position = Some(0);
                *pending_command = None;
            } else {
                *pending_command = Some('g')
            }

            true
        }

        KeyCode::Enter => {
            finish(state, items, selection);
            true
        }

        KeyCode::Char('q') => {
            *state = MoveState::default();
            true
        }

        _ => false,
    }
}
