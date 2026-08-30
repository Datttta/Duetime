use crate::app::{App, Popup, TasksPopup, InboxPopup};
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

pub fn keys(app: &App) -> Line<'static> {
    let bold = Style::default().add_modifier(Modifier::BOLD);

    match &app.popup {
        Popup::Tasks(
            TasksPopup::AddTask
            | TasksPopup::EditTask
            | TasksPopup::AddKnownTask
            | TasksPopup::EditKnownTask(_)
        ) => {
            Line::from(vec![
                Span::styled("Enter", bold),
                Span::raw(" Save  "),
                Span::styled("Esc", bold),
                Span::raw(" Cancel"),
            ])
        }

        Popup::Tasks(TasksPopup::Presets) => {
            Line::from(vec![
                Span::styled("a", bold),
                Span::raw(" Add preset  "),
                Span::styled("e", bold),
                Span::raw(" Edit  "),
                Span::styled("dd", bold),
                Span::raw(" Delete  "),
                Span::styled("Enter", bold),
                Span::raw(" Load  "),
                Span::styled("q", bold),
                Span::raw(" Close"),
            ])
        }

        Popup::Tasks(TasksPopup::NewPreset) => {
            Line::from(vec![
                Span::styled("a", bold),
                Span::raw(" Add Task  "),
                Span::styled("e", bold),
                Span::raw(" Edit Task  "),
                Span::styled("dd", bold),
                Span::raw(" Delete Task  "),
                Span::styled("Enter", bold),
                Span::raw(" Save  "),
                Span::styled("Esc", bold),
                Span::raw(" Cancel"),
            ])
        }

        Popup::Tasks(TasksPopup::KnownTasks) => {
            Line::from(vec![
                Span::styled("a", bold),
                Span::raw(" Add Task  "),
                Span::styled("e", bold),
                Span::raw(" Edit Task  "),
                Span::styled("dd", bold),
                Span::raw(" Delete Task  "),
                Span::styled("q", bold),
                Span::raw(" Close"),
            ])
        }

        Popup::Inbox(InboxPopup::InfoInboxItem) => {
            Line::from(vec![
                Span::styled("cc", bold),
                Span::raw(" Copy text  "),
                Span::styled("q", bold),
                Span::raw(" Quit"),
            ])
        }

        Popup::None => {
            Vec::new().into()
        }

        _ => {
            Vec::new().into()
        }
    }
}
