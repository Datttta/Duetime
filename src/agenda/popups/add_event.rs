use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, Padding, Paragraph},
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

use chrono::{NaiveDate, NaiveTime};
use log::info;

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
            Constraint::Length(10),
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

    let input = Layout::vertical([
        Constraint::Length(3), // event name
        Constraint::Length(1), // event date
        Constraint::Length(1), // event time
        Constraint::Length(1), // repeat
    ])
    .flex(Flex::Center)
    .split(inner);

    input::draw(
        frame,
        input[0],
        &app.event_name,
        "Event name",
        app.agenda_selected_input == AgendaSelectedInput::Name,
        app.mode,
        true,
    );

    let date_row = Layout::horizontal([
        Constraint::Length(6), // Width for label text
        Constraint::Min(0),
    ])
    .split(input[1]);

    frame.render_widget(Paragraph::new("Date:"), date_row[0]);
    ui::draw_date_time_input(
        frame,
        date_row[1],
        &app.event_date,
        app.agenda_selected_input == AgendaSelectedInput::Date,
    );
    
    let time_row = Layout::horizontal([
        Constraint::Length(6), // Width for label text
        Constraint::Min(0),
    ])
    .split(input[2]);

    frame.render_widget(Paragraph::new("Time:"), time_row[0]);
    
    ui::draw_date_time_input(
        frame,
        time_row[1],
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

pub fn save_event(app: &mut App) {
    match app.popup {
        Popup::Agenda(AgendaPopup::AddEvent) | Popup::Agenda(AgendaPopup::EditEvent) => {
            // 1. Validate inputs
            let name = app.event_name.text.trim().to_string();
            if name.is_empty() {
                app.set_status_message("Event name cannot be empty.".to_string());
                return;
            }

            let date = match NaiveDate::parse_from_str(&app.event_date.value, "%d-%m-%y") {
                Ok(date) => date,
                Err(_) => {
                    app.set_status_message("Invalid date.".to_string());
                    return;
                }
            };

            let time = if app.event_time.value == "--:--" {
                None
            } else {
                match NaiveTime::parse_from_str(&app.event_time.value, "%H:%M") {
                    Ok(time) => Some(time),
                    Err(_) => {
                        app.set_status_message("Invalid time.".to_string());
                        return;
                    }
                }
            };

            let event = AgendaEvent {
                name: name.clone(),
                date,
                time,
                repeat: app.event_repeat,
            };

            // 2. Perform Add or Edit action
            if matches!(app.popup, Popup::Agenda(AgendaPopup::AddEvent)) {
                app.events.push(event);
            } else if let Some(index) = app.agenda_table_state.selected() {
                if let Some(existing_event) = app.events.get_mut(index) {
                    *existing_event = event;
                }
            }

            // 3. Sort events chronologically
            app.events.sort_by(|a, b| {
                a.date.cmp(&b.date).then_with(|| match (a.time, b.time) {
                    (Some(t1), Some(t2)) => t1.cmp(&t2),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                })
            });

            // 4. Update table selection to keep track of the modified/added event
            if let Some(index) = app.events.iter().position(|e| e.name == name && e.date == date) {
                app.agenda_table_state.select(Some(index));
            }
        }
        _ => {}
    }

    storage::agenda::save_agenda(&app.events).unwrap();
    app.popup = Popup::None;
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
            let result = app.event_name.handle_vim_mode(key, &mut app.mode, usize::MAX);

            match result {
                InputResult::Consumed => return,
                InputResult::Ignored => {}
                InputResult::TextChanged => {}
            }
        }

        AgendaSelectedInput::Date => {
            match key.code {
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    app.event_date.insert_digit(c);
                }

                KeyCode::Char('h') => {
                    app.event_date.move_left();
                }

                KeyCode::Char('l') => {
                    app.event_date.move_right();
                }

                _ => {}
            }
        }

        AgendaSelectedInput::Time => {
            match key.code {
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    app.event_time.insert_digit(c);
                }

                KeyCode::Char('h') => {
                    app.event_time.move_left();
                }

                KeyCode::Char('l') => {
                    app.event_time.move_right();
                }

                _ => {}
            }
        }

        AgendaSelectedInput::Repeat => {
            match key.code {
                KeyCode::Char(' ') | KeyCode::Enter => {
                    app.event_repeat = !app.event_repeat;
                    return;
                }

                _ => {}
            }

        }
    }

    match key.code {
        KeyCode::Tab | KeyCode::Char('j') => {
            app.agenda_selected_input = match app.agenda_selected_input {
                AgendaSelectedInput::Name if app.mode != InputMode::Insert || key.code == KeyCode::Tab => AgendaSelectedInput::Date,
                AgendaSelectedInput::Name => AgendaSelectedInput::Name,
                AgendaSelectedInput::Date => AgendaSelectedInput::Time,
                AgendaSelectedInput::Time => AgendaSelectedInput::Repeat,
                AgendaSelectedInput::Repeat => AgendaSelectedInput::Name
            }
        }

        KeyCode::BackTab | KeyCode::Char('k') => {

            app.agenda_selected_input = match app.agenda_selected_input {
                AgendaSelectedInput::Name if app.mode != InputMode::Insert || key.code == KeyCode::BackTab => AgendaSelectedInput::Repeat,
                AgendaSelectedInput::Name => AgendaSelectedInput::Name,
                AgendaSelectedInput::Repeat => AgendaSelectedInput::Time,
                AgendaSelectedInput::Time => AgendaSelectedInput::Date,
                AgendaSelectedInput::Date => AgendaSelectedInput::Name,
            }
        }

        KeyCode::Enter => {
            save_event(app);
        }

        KeyCode::Char('q') | KeyCode::Esc => {
            close_popup(app);
        }

        _ => {}
    }
}
