use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, Padding, Paragraph},
    text::{Line, Span},
    style::{Style, Color, Modifier},
    Frame
};

use crate::{
    ui::widgets::input,
    vim_text::{InputResult, InputMode},
    app::{App, Popup, AgendaPopup, AgendaSelectedInput},
    agenda::{
        ui::AgendaEvent,
        ui,
    },
    storage,
    keys_help,
};

use chrono::{Local, NaiveDate, NaiveTime};
use log::info;

const EDITABLE_POSITIONS: [usize; 6] = [0, 1, 3, 4, 6, 7];

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame, app);

    frame.render_widget(Clear, area);
    
    let block = Block::bordered()
        .title("Add Event")
        .padding(Padding::new(2, 2, 0, 0));
    
    let inner = block.inner(area);

    frame.render_widget(block, area);
    
    fn centered_rect(frame: &mut Frame, app: &mut App) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(15),
            Constraint::Length(1), // keys_help
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(50),
        ])
        .flex(Flex::Center)
        .split(vertical[0]);

        let keys_help = Paragraph::new(keys_help::keys(app))
                .alignment(Alignment::Center);
        frame.render_widget(keys_help, vertical[1]);
        
        horizontal[0]
    }

    let vertical = Layout::vertical([
        Constraint::Length(10), // input height
    ])
    .split(inner);

    let input = Layout::vertical([
        Constraint::Length(3), // event name
        Constraint::Length(1), // event date
        Constraint::Length(1), // event time
        Constraint::Length(1), // repeat
    ])
    .flex(Flex::Center)
    .split(vertical[0]);

    input::draw(
        frame,
        input[0],
        &app.event,
        "Event name",
        app.agenda_selected_input == AgendaSelectedInput::Name,
        app.mode,
        true,
    );

    ui::draw_date_time_input(
        frame,
        input[1],
        &app.event_date,
        app.agenda_selected_input == AgendaSelectedInput::Date,
    );
    
    ui::draw_date_time_input(
        frame,
        input[2],
        &app.event_time,
        app.agenda_selected_input == AgendaSelectedInput::Time,
    );
    
    ui::draw_repeat_input(
        frame,
        input[3],
        app.event_repeat,
        app.agenda_selected_input == AgendaSelectedInput::Repeat,
    );
}

pub fn save_event(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let name = app.event.text.trim().to_string();

    if name.is_empty() {
        app.set_status_message("Event name cannot be empty.".to_string());
        return Ok(());
    }

    let date = match NaiveDate::parse_from_str(
        &app.event_date.value,
        "%d-%m-%y",
    ) {
        Ok(date) => date,
        Err(_) => {
            app.set_status_message("Invalid date.".to_string());
            return Ok(());
        }
    };

    let time = if app.event_time.value == "00:00" {
        None
    } else {
        match NaiveTime::parse_from_str(
            &app.event_time.value,
            "%H:%M",
        ) {
            Ok(time) => Some(time),
            Err(_) => {
                app.set_status_message("Invalid time.".to_string());
                return Ok(());
            }
        }
    };

    let event = AgendaEvent {
        name,
        date,
        time,
        repeat: app.event_repeat,
    };

    app.events.push(event);

    storage::agenda::save_agenda(&app.events).unwrap();

    app.popup = Popup::None;

    Ok(())
}

fn close_popup(app: &mut App) {
    info!("Selecte input: {:?}", app.agenda_selected_input);
    info!("app mode: {:?}", app.mode);
    if app.agenda_selected_input == AgendaSelectedInput::Name{
        if app.mode == InputMode::Normal {
            app.popup = Popup::None;
        } 
        return
    }

    app.popup = Popup::None;
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    match app.agenda_selected_input {
        AgendaSelectedInput::Name => {
            let result = app.event.handle_vim_mode(key, &mut app.mode, usize::MAX);

            match result {
                InputResult::Consumed => return,
                InputResult::Ignored => {}
                InputResult::TextChanged => {}
            }

            match key.code {
                KeyCode::Char(' ') | KeyCode::Enter => {
                    app.event_repeat = !app.event_repeat;
                }

                _ => {}
            }
        }

        _ => {}
    }

    match key.code {
        KeyCode::Enter => {
            save_event(app);
        }

        KeyCode::Char('q') | KeyCode::Esc => {
            close_popup(app);
        }

        _ => {}
    }
}
