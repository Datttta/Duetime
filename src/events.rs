use std::io;

use crossterm::event::{self, Event, KeyCode};

use crate::app::{App, Popup};

pub fn handle_events(app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
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

                KeyCode::Esc => {
                    app.popup = Popup::None;
                }

                KeyCode::Char('q') => {
                    app.running = false;
                }

                _ => {
                    app.waiting_for_t = false;
                }
            }
        }
    }

    Ok(())
}
