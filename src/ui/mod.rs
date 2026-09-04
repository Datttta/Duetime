pub mod widgets;
pub mod theme;
pub mod help;

//use log::info;

use crate::{
    app::{App, Popup, TasksTablePopup, Panel, InboxPopup, AgendaPopup},
    
    tasks_table::{
        ui::draw_tasks_panel,
        popups::{
            task_add,
            task_info, 
            known_tasks,
            known_tasks_add, 
            presets,
            new_preset,
        }
    },

    inbox::{
        ui::draw_inbox_panel,
        popups::{
            inbox_item_add,
            inbox_item_info,
        },
    },

    agenda::{
        ui::draw_agenda_panel,
        popups::{
            add_event,
        },
    },

    ui::widgets::status_message::draw_status_message,
};

use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};

struct MainLayout {
    tasks: Rect,
    inbox: Rect,
    agenda: Rect,
}

fn draw_layout(frame: &mut Frame) -> MainLayout {
    let chunks = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Length(1),
        Constraint::Percentage(50),
    ])
    .split(frame.area());

    let right = Layout::vertical([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(chunks[2]);

    MainLayout {
        tasks: chunks[0],
        inbox: right[0],
        agenda: right[1],
    }
}

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    let status_area = Rect {
        x: frame.area().x,
        y: frame.area().bottom().saturating_sub(2),
        width: frame.area().width,
        height: 1,
    };

    // Small terminal window: show only tasks
    if area.width < 140 {
        // check the previous focused panel
        if app.is_change {
            app.previous_panel = app.focused_panel;
            app.is_change = false;
        }

        app.focused_panel = Panel::TasksTable;
        draw_tasks_panel(frame, area, app);
    } else {
        //focus on previous panel
        if !app.is_change {
            app.focused_panel = app.previous_panel;
            app.is_change = true;
        }

        // Show all panels if in fullscreen
        let layout = draw_layout(frame);
        
        // draw panels
        draw_tasks_panel(frame, layout.tasks, app);
        draw_inbox_panel(frame, layout.inbox, app);
        draw_agenda_panel(frame, layout.agenda, app);
    }

    //help popup
    if let Popup::Help = app.popup {
        help::draw(frame, app);
    }

    // TasksTable-panel popups
    if let Popup::TasksTable(popup) = &app.popup {
        match popup {
            TasksTablePopup::AddTask => {
                task_add::draw(frame, app);
            }

            TasksTablePopup::EditTask => {
                task_add::draw(frame, app);
            }

            TasksTablePopup::Presets => {
                presets::draw(frame, app);
            }

            TasksTablePopup::NewPreset => {
                new_preset::draw(frame, app);
            }

            TasksTablePopup::KnownTasks => {
                known_tasks::draw(frame, app);
            }

            TasksTablePopup::AddKnownTask => {
                known_tasks_add::draw(frame, app);
            }

            TasksTablePopup::EditKnownTask(_) => {
                known_tasks_add::draw(frame, app);
            }

            TasksTablePopup::TaskInfo => {
                task_info::draw(frame, app);
            }
        }
    }

    // Inbox-panel popups
    if let Popup::Inbox(popup) = &app.popup {
        match popup {
            InboxPopup::AddInboxItem => {
                inbox_item_add::draw(frame, app);
            }
            
            InboxPopup::EditInboxItem => {
                inbox_item_add::draw(frame, app);
            }
            
            InboxPopup::InfoInboxItem => {
                inbox_item_info::draw(frame, app);
            }
        }
    }

    // agenda-panel popups
    if let Popup::Agenda(popup) = &app.popup {
        match popup {
            AgendaPopup::AddEvent => {
                add_event::draw(frame, app);
            }
        }
    }

    draw_status_message(frame, app, status_area);
}

