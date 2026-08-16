pub mod header;
pub mod popup;
pub mod widgets;
pub mod theme;

use crate::app::{App, Popup};
use crate::keys_help;
use crate::tasks;

use std::time::Duration;

use ratatui::{
    layout::{Constraint, Layout, Rect, Alignment},
    widgets::{Block, Padding, Paragraph},
    Frame,
};

struct MainLayout {
    header: Rect,
    tasks: Rect,
    footer: Rect,
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn draw_layout(frame: &mut Frame) -> MainLayout {
    let border = Block::bordered()
        .title(" Duetime ")
        .padding(Padding::new(0, 0, 1, 0));

    let inner = border.inner(frame.area());

    frame.render_widget(border, frame.area());

    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(2),
    ])
    .split(inner);

    MainLayout {
        header: chunks[0],
        tasks: chunks[2],
        footer: chunks[3],
    }
}

pub fn draw(frame: &mut Frame, app: &mut App) {
    let layout = draw_layout(frame);

    header::draw(frame, layout.header);
    tasks::draw(frame, layout.tasks, app);
    
    if let Popup::None = app.popup {

        let status = Paragraph::new(format!(
            " Total elapsed: {}",
            format_duration(app.total_elapsed())
        ))
        .block(
            Block::default()
                .padding(Padding::new(2, 0, 0, 0))
        );

        frame.render_widget(status, layout.footer);
    }

    if let Popup::AddTask = app.popup {
        popup::add_task::draw(frame, app);
    }

    if let Popup::EditTask = app.popup {
        popup::add_task::draw(frame, app);
    }

    if let Popup::NewPreset = app.popup {
        popup::new_preset::draw(frame, app);
    }
    
    if let Popup::Presets = app.popup {
        popup::presets::draw(frame, app);
    }

    if let Popup::KnownTasks = app.popup {
        popup::known_tasks::draw(frame, app);
    }

    if let Popup::AddKnownTask = app.popup {
        popup::known_tasks_add::draw(frame, app);
    }

    if let Popup::EditKnownTask(_) = app.popup {
        popup::known_tasks_add::draw(frame, app);
    }

    if let Popup::TaskInfo = app.popup {
        popup::task_info::draw(frame, app);
    }

    if let Popup::Help = app.popup {
        popup::help::draw(frame, app);
    }
}
