pub mod header;
pub mod widgets;
pub mod theme;
pub mod inbox_header;

//use log::info;
use std::time::{Duration, Instant};

use crate::{
    app::{App, Popup, TasksPopup, Panel, InboxPopup},
    popup::{
        task_add, 
        presets,
        new_preset,
        known_tasks,
        known_tasks_add,
        task_info,
        help,
        inbox_item_add,
        inbox_item_info
    },
    ui::theme::unfocused_panel,
    navigation::vim_navigation::NavigationMode,
    tasks, inbox,
};

use ratatui::{
    layout::{Constraint, Layout, Rect, Alignment},
    widgets::{Block, Padding, Paragraph},
    style::{Color, Style},
    Frame,
};

struct MainLayout {
    tasks: Rect,
    inbox: Rect,
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn draw_layout(frame: &mut Frame) -> MainLayout {
    let chunks = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Length(1),
        Constraint::Percentage(50),
    ])
    .split(frame.area());

    MainLayout {
        tasks: chunks[0],
        inbox: chunks[2],
    }
}

fn draw_tasks_panel(
    frame: &mut Frame,
    area: Rect, 
    app: &mut App,
) {
    let border_color = if app.focused_panel == Panel::Tasks {
        Color::White
    } else {
        unfocused_panel()
    };

    let border = Block::bordered()
        .title(" Tasks ")
        .border_style(Style::default().fg(border_color))
        .padding(Padding::new(0, 0, 1, 0));

    let inner = border.inner(area);

    frame.render_widget(border, area);

    let chunks = Layout::vertical ([
        Constraint::Length(1), // header
        Constraint::Length(1), // spacing
        Constraint::Min(0),    // tasks
        Constraint::Length(2), // footer
    ])
    .split(inner);

    let is_visual = app.focused_panel == Panel::Tasks
        && app.n_mode == NavigationMode::Visual;

    header::draw(frame, chunks[0]);
    tasks::ui::draw(frame, chunks[2], app, is_visual);

    if let Popup::None = app.popup {
        let status = Paragraph::new(format!(
                " Total elapsed {}",
                format_duration(app.total_elapsed())
        ))
        .block(
            Block::default()
                .padding(Padding::new(2, 0, 0, 0))
        );

        frame.render_widget(status, chunks[3]);
    }
}

fn draw_inbox_panel (
    frame: &mut Frame,
    area: Rect,
    app: &mut App
) {
    let border_color = if app.focused_panel == Panel::Inbox {
        Color::White
    } else {
        unfocused_panel()
    };

    let border = Block::bordered()
        .title(" Inbox ")
        .border_style(Style::default().fg(border_color))
        .padding(Padding::new(0, 0, 1, 0));
    
    let inner = border.inner(area);

    frame.render_widget(border, area);

    let chunks = Layout::vertical ([
        Constraint::Length(1), // header
        Constraint::Length(1), // spacing
        Constraint::Min(0),    // tasks
    ])
    .split(inner);

    let is_visual = app.focused_panel == Panel::Inbox
        && app.n_mode == NavigationMode::Visual;

    inbox_header::draw(frame, chunks[0]);
    inbox::ui::draw(frame, chunks[2], app, is_visual);
}

fn draw_status_message(
    frame: &mut Frame,
    app: &mut App,
    area: Rect,
) {
    let Some(message) = &app.status_message else {
        return;
    };

    let Some(expires) = app.status_message_until else {
        return;
    };

    if Instant::now() >= expires {
        app.status_message = None;
        app.status_message_until = None;
        return;
    }

    let paragraph = Paragraph::new(message.as_str())
        .alignment(Alignment::Right)
        .block(
            Block::default()
                .padding(Padding::new(0, 2, 0, 0))
        );

    frame.render_widget(paragraph, area);
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

        app.focused_panel = Panel::Tasks;
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
    }

    // Tasks-panel popups
    if let Popup::Tasks(popup) = &app.popup {
        match popup {
            TasksPopup::AddTask => {
                task_add::draw(frame, app);
            }

            TasksPopup::EditTask => {
                task_add::draw(frame, app);
            }

            TasksPopup::Presets => {
                presets::draw(frame, app);
            }

            TasksPopup::NewPreset => {
                new_preset::draw(frame, app);
            }

            TasksPopup::KnownTasks => {
                known_tasks::draw(frame, app);
            }

            TasksPopup::AddKnownTask => {
                known_tasks_add::draw(frame, app);
            }

            TasksPopup::EditKnownTask(_) => {
                known_tasks_add::draw(frame, app);
            }

            TasksPopup::TaskInfo => {
                task_info::draw(frame, app);
            }

            TasksPopup::Help => {
                help::draw(frame, app);
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

    draw_status_message(frame, app, status_area);
}

