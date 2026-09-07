use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex},
    widgets::{Clear, Block, Paragraph, Padding, Wrap},
    text::{Line},
    Frame
};

use crate::{
    app::{App, Popup},
    agenda::ui::format_countdown,
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Event info")
        .padding(Padding::new(1,1,0,0));

    frame.render_widget(&block, area);

    fn centered_rect(frame: &mut Frame) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(23),
            Constraint::Length(1), // keys help
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(55),
        ])
        .flex(Flex::Center)
        .split(vertical[0]);
        
        horizontal[0]
    }


    let inner = block.inner(area);
    frame.render_widget(block, area);
    
    if let Some(index) = app.agenda_table_state.selected() {
        let event = &app.events[index];

        let time = event
            .time
            .map(|time| time.format("%H:%M").to_string())
            .unwrap_or_else(|| "no time".to_string());

        let countdown = format_countdown(event.date);
        
        let paragraph = Paragraph::new(vec![
            Line::from("Event name:"),
            Line::from(event.name.as_str()), 
            Line::from(" "),
            Line::from(format!("Date: {}", event.date.format("%a, %b %-d").to_string())),
            Line::from(format!("Time: {}", time)),
            Line::from(format!("Countdown: {}", countdown)),
            Line::from(format!("Repeat: {}", event.repeat)),
        ])
        .wrap(Wrap { trim: false });
    
        frame.render_widget(paragraph, inner);
    }
}

pub fn handle_keys (app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => {
            app.popup = app.last_popup.clone();
        }

        _ => {}
    }
}

