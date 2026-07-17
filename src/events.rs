use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::app::{App, Popup};
use crate::ui::popup;


pub fn handle_events(app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {

            match app.popup {
                Popup::AddTask => {
                    popup::add_task::handle_keys(app, key);
                }

                Popup::None => {
                    handle_normal_keys(app, key);
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

        KeyCode::Char('t') => {
            if app.waiting_for_t {
                app.popup = Popup::AddTask;
            }

            app.waiting_for_t = false;
        }

        KeyCode::Char('q') => {
            app.running = false;
        }

        _ => {
            app.waiting_for_t = false;
        }
    }
}
