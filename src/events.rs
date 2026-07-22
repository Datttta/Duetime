use crate::app::{App, Popup, SelectedInput};
use crate::ui::popup;
use crate::vim::InputMode;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};

pub fn handle_events(app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {

            match app.popup {
                Popup::None => {
                    handle_normal_keys(app, key);
                }

                Popup::AddTask => {
                    popup::add_task::handle_keys(app, key);
                }

                Popup::EditTask(_) => {
                    popup::add_task::handle_keys(app, key);
                }
            }
        }
    }

    Ok(())
}


fn handle_normal_keys(app: &mut App, key: KeyEvent) {
    match key.code {

        KeyCode::Char('a') => {
            app.waiting_for_t = true;
        }

        KeyCode::Char('d') => {
            if app.waiting_for_d {
                delete_task(app);
                app.waiting_for_d = false
            } else {
                app.waiting_for_d = true
            }
        }

        KeyCode::Char('e') => {
            if !app.tasks.is_empty() {
                app.popup = Popup::EditTask(app.selected_task);

                let task = &app.tasks[app.selected_task];
                
                //load task data into the inputs
                app.task_name.text = task.name.clone();
                app.planned_start.text = task.planned_start.clone();
                app.planned_end.text = task.planned_end.clone();

                app.task_name.cursor = app.task_name.text.len();
                app.planned_start.cursor = app.planned_start.text.len();
                app.planned_end.cursor = app.planned_end.text.len();

                app.mode = InputMode::Insert;
                app.selected_input = SelectedInput::TaskName;
            }
        }

        KeyCode::Char('t') => {
            if app.waiting_for_t {
                app.popup = Popup::AddTask;
                app.selected_input = SelectedInput::TaskName;
                app.mode = InputMode::Insert;
            }

            app.waiting_for_t = false;
        }

        KeyCode::Char('q') => {
            app.running = false;
        }

        KeyCode::Char('k') => {
            let selected = app.table_state.selected().unwrap_or(0);

            if selected > 0 {
                app.table_state.select(Some(selected - 1));
            }
        }

        KeyCode::Char('j') => {
            let selected = app.table_state.selected().unwrap_or(0);

            if selected + 1 <= app.tasks.len() {
                app.table_state.select(Some(selected + 1));
            }
        }

        _ => {
            app.waiting_for_t = false;
        }
    }
}

pub fn handle_escape(app: &mut App) -> bool {
    match app.mode {
        InputMode::Normal => true, // close popup
                                   
        _ => {
            app.mode = InputMode::Normal;
            false // do not close
        }
    }
}

fn delete_task(app: &mut App) {
    if app.tasks.is_empty() {
        return;
    }

    if app.selected_task < app.tasks.len() {
        app.tasks.remove(app.selected_task);

        // keep selection valid
        if app.selected_task >= app.tasks.len() && app.selected_task > 0 {
            app.selected_task -= 1;
        }
    }
}
