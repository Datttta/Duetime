use crossterm::event::{KeyCode, KeyEvent};
use log::info;
use crate::{
    app::App,
};

pub fn move_tasks(app: &mut App) {
    if let Some(index) = app.table_state.selected() {
        app.moving_task = Some(index);
        let Some(current) = app.table_state.selected() else {
            return;
        };

        let (beginning_selected_row, end_selected_row) =  
            if let Some(start) = app.n_visual_start {
                (start.min(current), start.max(current))
            } else {
                (current, current)
            };

        if index == beginning_selected_row {
            app.move_position = Some(index);
        } else {
            app.move_position = Some(index + 1);
        }
    }
}

fn next_position(app: &App, position: usize) -> usize {
    let Some(current) = app.table_state.selected() else {
        return position.saturating_add(1);
    };

    let (beginning_selected_row, end_selected_row) =  
        if let Some(start) = app.n_visual_start {
            (start.min(current), start.max(current))
        } else {
            (current, current)
        };

    let moving = (end_selected_row - beginning_selected_row) + 1;

    let mut next = position.saturating_add(1);

    if Some(next - 1) == Some(beginning_selected_row) {
        next += moving - 1;
    }


    next.min(app.tasks.len())
}

fn previous_position(app: &App, position: usize) -> usize {
    let Some(current) = app.table_state.selected() else {
        return position.saturating_sub(1);
    };

    let (beginning_selected_row, end_selected_row) =
        if let Some(start) = app.n_visual_start {
            (start.min(current), start.max(current))
        } else {
            (current, current)
        };

    let moving = end_selected_row - beginning_selected_row + 1;

    let mut previous = position.saturating_sub(1);

    if previous == end_selected_row {
        if beginning_selected_row == 0 {
            previous = previous.saturating_add(1)
        } else {
            previous = previous.saturating_sub(moving - 1);
        }
    }

    previous
}

fn finish_move(app: &mut App) {
    let Some(from) = app.moving_task.take() else {
        return;
    };

    let Some(mut position) = app.move_position.take() else {
        return;
    };

    if app.tasks.is_empty() {
        return;
    }

    let task = app.tasks.remove(from);

    if position > from {
        position -= 1;
    }

    position = position.min(app.tasks.len());

    app.tasks.insert(position, task);

    app.table_state.select(Some(position));
}

fn cancel_move(app: &mut App) {
    app.moving_task = None;
    app.move_position = None;
    app.n_visual_start = None;
}

pub fn handle_keys(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('j') => {
            if let Some(position) = app.move_position {
                app.move_position = Some(next_position(app, position));
            }

            true
        }

        KeyCode::Char('k') => {
            if let Some(position) = app.move_position {
                app.move_position = Some(previous_position(app, position));
            }

            true
        }

        KeyCode::Enter => {
            finish_move(app);
            true
        }

        KeyCode::Char('q') => {
            cancel_move(app);
            true
        }

        _ => true,
    }
}
