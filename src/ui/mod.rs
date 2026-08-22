pub mod header;
pub mod popup;
pub mod widgets;
pub mod theme;
pub mod inbox_header;

use crate::{
    app::{App, Popup, TasksPopup, Panel, InboxPopup},
    ui::theme::unfocused_panel,
    tasks, inbox,
};

use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Padding, Paragraph},
    style::{Color, Style},
    Frame,
};

use std::time::Duration;

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

    header::draw(frame, chunks[0]);
    tasks::draw(frame, chunks[2], app);

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

    let chunks = Layout::vertical ([
        Constraint::Length(1), // header
        Constraint::Length(1), // spacing
        Constraint::Min(0),    // tasks
    ])
    .split(inner);

    inbox_header::draw(frame, chunks[0]);
    inbox::draw(frame, chunks[2], app);

    frame.render_widget(border, area);
}

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    // Small terminal window: show only tasks
    if area.width < 100 || area.height < 30 {
        app.focused_panel = Panel::Tasks;
        draw_tasks_panel(frame, area, app);
    } else {
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
                popup::add_task::draw(frame, app);
            }

            TasksPopup::EditTask => {
                popup::add_task::draw(frame, app);
            }

            TasksPopup::Presets => {
                popup::presets::draw(frame, app);
            }

            TasksPopup::NewPreset => {
                popup::new_preset::draw(frame, app);
            }

            TasksPopup::KnownTasks => {
                popup::known_tasks::draw(frame, app);
            }

            TasksPopup::AddKnownTask => {
                popup::known_tasks_add::draw(frame, app);
            }

            TasksPopup::EditKnownTask(_) => {
                popup::known_tasks_add::draw(frame, app);
            }

            TasksPopup::TaskInfo => {
                popup::task_info::draw(frame, app);
            }

            TasksPopup::Help => {
                popup::help::draw(frame, app);
            }
        }
    }

    // Inbox-panel popups
    if let Popup::Inbox(popup) = &app.popup {
        match popup {
            InboxPopup::AddInboxItem => {
                popup::add_inbox_item::draw(frame, app);
            }
        }
    }
}
