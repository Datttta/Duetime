use crate::app::{App, Popup};
use ratatui::text::{Line, Span};
use ratatui::style::{Style, Modifier};

pub fn keys(app: &App) -> Line<'static> {
    let bold = Style::default().add_modifier(Modifier::BOLD);

    match app.popup {
        Popup::None => Line::from(vec![
            Span::styled("at", bold),
            Span::raw(" Add Task   "),
            Span::styled("e", bold),
            Span::raw(" Edit   "),
            Span::styled("dd", bold),
            Span::raw(" Delete   "),
            Span::styled("s", bold),
            Span::raw(" Start   "),
            Span::styled("p", bold),
            Span::raw(" Pause   "),
            Span::styled("r", bold),
            Span::raw(" Reset   "),
            Span::styled("P", bold),
            Span::raw(" Presets  "),
        ]),

        Popup::AddTask | Popup::EditTask(_) => Line::from(vec![
            Span::styled("Enter", bold),
            Span::raw(" Save  "),
            Span::styled("Esc", bold),
            Span::raw(" Cancel"),
        ]),

        Popup::Presets => Line::from(vec![
            Span::styled("n", bold),
            Span::raw(" New  "),
            Span::styled("e", bold),
            Span::raw(" Edit  "),
            Span::styled("dd", bold),
            Span::raw(" Delete  "),
            Span::styled("Enter", bold),
            Span::raw(" Load  "),
            Span::styled("Esc", bold),
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
    }
}
