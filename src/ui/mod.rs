pub mod header;
pub mod popup;
pub mod widgets;
pub mod theme;

use crate::app::{App, Popup};
use crate::keys_help;
use crate::tasks;

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

fn draw_layout(frame: &mut Frame) -> MainLayout {
    let border = Block::bordered()
        .title(" Duetime ")
        .padding(Padding::new(0, 0, 1, 1));

    let inner = border.inner(frame.area());

    frame.render_widget(border, frame.area());

    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
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
        let keys_help = Paragraph::new(keys_help::keys(app))
            .alignment(Alignment::Center);

        frame.render_widget(keys_help, layout.footer);
    }

    if let Popup::AddTask = app.popup {
        popup::add_task::draw(frame, app);
    }

    if let Popup::EditTask(_) = app.popup {
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
}
