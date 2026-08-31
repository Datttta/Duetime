use crate::{
    app::{
    App,
    Popup, 
    Panel, 
    TasksPopup, 
    InboxPopup, 
    },

    storage::{current_tasks},
    tasks_table, inbox, popup
};

use std::io;

use crossterm::event::{self, Event, KeyCode};

pub fn handle_events(app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {

            // Panel switching only when no popup is open
            if matches!(app.popup, Popup::None) {
                match key.code {
                    KeyCode::Char('L') => {
                        app.focused_panel = match app.focused_panel {
                            Panel::Tasks => Panel::Inbox,
                            Panel::Inbox => Panel::Inbox,
                        };
                        return Ok(());
                    }

                    KeyCode::Char('H') => {
                        app.focused_panel = match app.focused_panel {
                            Panel::Inbox => Panel::Tasks,
                            Panel::Tasks => Panel::Tasks,
                        };
                        return Ok(());
                    }

                    KeyCode::Char('?') => {
                        app.popup = Popup::Tasks(TasksPopup::Help);
                    }
                    
                    KeyCode::Char('q') => {
                        current_tasks::save_current_tasks(&app.tasks).unwrap();
                        app.running = false;
                    }

                    _ => {}
                }
            }

            // Popup gets priority over focused panel
            match &app.popup {
                Popup::None => {
                    match app.focused_panel {
                        Panel::Tasks => tasks_table::keys::handle_keys(app, key),
                        Panel::Inbox => inbox::keys::handle_keys(app, key),
                    }
                }

                // TASKS POPUPS

                Popup::Tasks(TasksPopup::AddTask) => {
                    popup::task_add::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::EditTask) => {
                    popup::task_add::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::Presets) => {
                    popup::presets::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::NewPreset) => {
                    popup::new_preset::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::KnownTasks) => {
                    popup::known_tasks::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::AddKnownTask) => {
                    popup::known_tasks_add::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::EditKnownTask(_)) => {
                    popup::known_tasks_add::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::TaskInfo) => {
                    popup::task_info::handle_keys(app, key);
                }

                Popup::Tasks(TasksPopup::Help) => {
                    popup::help::handle_keys(app, key);
                }

                // INBOX POPUPS

                Popup::Inbox(InboxPopup::AddInboxItem) => {
                    popup::inbox_item_add::handle_keys(app, key);
                }

                Popup::Inbox(InboxPopup::EditInboxItem) => {
                    popup::inbox_item_add::handle_keys(app, key);
                }
                
                Popup::Inbox(InboxPopup::InfoInboxItem) => {
                    popup::inbox_item_info::handle_keys(app, key);
                }
            }
        }
    }

    Ok(())
}
