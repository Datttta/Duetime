use crate::app::{App, Popup};
use ratatui::text::{Line, Span};
use ratatui::style::{Style, Modifier};

pub fn keys(app: &App) -> Line<'static> {
    let bold = Style::default().add_modifier(Modifier::BOLD);
    
    match app.popup {
        Popup::None => Line::from(
            " at Add Task | e Edit | dd Delete | s Start | p Pause | r Reset | P Presets | q Quit"
        ),

        Popup::AddTask | Popup::EditTask(_) => Line::from(vec![
            Span::styled("Enter", bold),
            Span::raw(" Save | "),
            Span::styled("Esc", bold),
            Span::raw(" Cancel"),
        ]),

        Popup::Presets => Line::from(
            "j/k Move | Enter Load | n New | e Edit | dd Delete | Esc Close"
        ),

        Popup::NewPreset => Line::from(
            "Tab Switch | a Add Task | e Edit Task | dd Delete Task | Enter Save | Esc Cancel"
        ),
    }
}

