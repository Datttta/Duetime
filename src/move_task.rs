use crossterm::event::{KeyCode, KeyEvent};
use log::info;
use crate::app::App;

pub fn move_tasks(app: &mut App) {
    if let Some(index) = app.table_state.selected() {
        app.moving_task = Some(index);
        app.move_position = Some(index);
    }
}

fn next_position(app: &App, position: usize) -> usize {
    let mut next = position + 1;

    if let Some(current) = app.table_state.selected() {
        let (beginning_selected_row, end_selected_row) =  
        if let Some(start) = app.n_visual_start {
            (start.min(current), start.max(current))
        } else {
            (current, current)
        };

        let moving = (end_selected_row - beginning_selected_row) + 1;

        info!("moving: {:?}", moving);

        if Some(next - 1) == Some(beginning_selected_row) {
            next += (moving - 1);
        }
    };


    next.min(app.tasks.len())
}

fn previous_position(app: &App, position: usize) -> usize {
    let Some(moving) = app.moving_task else {
        return position;
    };

    if position == 0 {
        return 0;
    }

    let mut previous = position - 1;

    if previous == moving {
        previous = previous.saturating_sub(1);
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

        KeyCode::Esc => {
            cancel_move(app);
            true
        }

        _ => true,
    }
}
