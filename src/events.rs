use crate::{
    app::{
    App,
    Popup, 
    Panel, 
    TasksTablePopup, 
    InboxPopup, 
    },

    tasks_table::popups::{
        task_add,
        task_info,
        known_tasks,
        known_tasks_add,
        presets,
        new_preset,
    },

    inbox::popups::{
        inbox_item_add,
        inbox_item_info,
    },

    storage::current_tasks,
    ui::help,
    tasks_table, inbox, agenda
};

use std::io;

use crossterm::event::{self, Event, KeyCode};

pub fn handle_events(app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {

            // Panel switching only when no popup is open
            if matches!(app.popup, Popup::None) {
                if key.code == KeyCode::Char('?') {
                    app.popup = Popup::Help;
                    return Ok(());
                }

                match key.code {
                    KeyCode::Char('L') => {
                        app.focused_panel = match app.focused_panel {
                            Panel::TasksTable => Panel::Inbox,
                            Panel::Inbox => Panel::Inbox,
                            Panel::Agenda => Panel::Agenda,
                        };
                        return Ok(());
                    }

                    KeyCode::Char('H') => {
                        app.focused_panel = match app.focused_panel {
                            Panel::Inbox => Panel::TasksTable,
                            Panel::TasksTable => Panel::TasksTable,
                            Panel::Agenda => Panel::TasksTable,
                        };
                        return Ok(());
                    }
                    
                    KeyCode::Char('J') => {
                        app.focused_panel = match app.focused_panel {
                            Panel::Inbox => Panel::Agenda,
                            Panel::Agenda => Panel::Agenda,
                            Panel::TasksTable => Panel::TasksTable,
                        };
                        return Ok(());
                    }
                    
                    KeyCode::Char('K') => {
                        app.focused_panel = match app.focused_panel {
                            Panel::Agenda => Panel::Inbox,
                            Panel::Inbox => Panel::Inbox,
                            Panel::TasksTable => Panel::TasksTable,
                        };
                        return Ok(());
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
                        Panel::TasksTable => tasks_table::keys::handle_keys(app, key),
                        Panel::Inbox => inbox::keys::handle_keys(app, key),
                        Panel::Agenda => {},
                    }
                }

                // TASKS POPUPS

                Popup::TasksTable(TasksTablePopup::AddTask) => {
                    task_add::handle_keys(app, key);
                }

                Popup::TasksTable(TasksTablePopup::EditTask) => {
                    task_add::handle_keys(app, key);
                }

                Popup::TasksTable(TasksTablePopup::Presets) => {
                    presets::handle_keys(app, key);
                }

                Popup::TasksTable(TasksTablePopup::NewPreset) => {
                    new_preset::handle_keys(app, key);
                }

                Popup::TasksTable(TasksTablePopup::KnownTasks) => {
                    known_tasks::handle_keys(app, key);
                }

                Popup::TasksTable(TasksTablePopup::AddKnownTask) => {
                    known_tasks_add::handle_keys(app, key);
                }

                Popup::TasksTable(TasksTablePopup::EditKnownTask(_)) => {
                    known_tasks_add::handle_keys(app, key);
                }

                Popup::TasksTable(TasksTablePopup::TaskInfo) => {
                    task_info::handle_keys(app, key);
                }

                Popup::Help => {
                    help::handle_keys(app, key);
                }

                // INBOX POPUPS

                Popup::Inbox(InboxPopup::AddInboxItem) => {
                    inbox_item_add::handle_keys(app, key);
                }

                Popup::Inbox(InboxPopup::EditInboxItem) => {
                    inbox_item_add::handle_keys(app, key);
                }
                
                Popup::Inbox(InboxPopup::InfoInboxItem) => {
                    inbox_item_info::handle_keys(app, key);
                }
            }
        }
    }

    Ok(())
}
