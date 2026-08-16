use crate::app::{App, Popup};
use ratatui::text::{Line, Span};
use ratatui::style::{Style, Modifier};

pub fn keys(app: &App) -> Line<'static> {
    let bold = Style::default().add_modifier(Modifier::BOLD);

    match app.popup {
        Popup::AddTask | Popup::EditTask | Popup::AddKnownTask | Popup::EditKnownTask(_) => Line::from(vec![
            Span::styled("Enter", bold),
            Span::raw(" Save  "),
            Span::styled("Esc", bold),
            Span::raw(" Cancel"),
        ]),

        Popup::Presets => Line::from(vec![
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
        ]),

        Popup::NewPreset => Line::from(vec![
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
        ]),

        Popup::KnownTasks => Line::from(vec![
            Span::styled("a", bold),
            Span::raw(" Add Task  "),
            Span::styled("e", bold),
            Span::raw(" Edit Task  "),
            Span::styled("dd", bold),
            Span::raw(" Delete Task  "),
            Span::styled("q", bold),
            Span::raw(" Close"),
        ]),

        _ => Vec::new().into()
    }
}
