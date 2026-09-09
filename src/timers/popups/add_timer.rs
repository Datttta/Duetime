use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, Padding, Paragraph},
    Frame
};

use crate::{
    ui::widgets::{
        date_time_input::draw_date_time_input,
        input,
    },
    vim_text::{InputResult, InputMode},
    app::{App, Popup, AgendaPopup, TimerSelectedInput},
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
        .title("Add Timer")
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
        Constraint::Length(1), // event time
    ])
    .flex(Flex::Center)
    .split(inner);

    input::draw(
        frame,
        input[0],
        &app.timer_name,
        "Time name",
        app.timer_selected_input == TimerSelectedInput::Name,
        app.mode,
        true,
    );

    let duration_row = Layout::horizontal([
        Constraint::Length(6), // Width for label text
        Constraint::Min(0),
    ])
    .split(input[1]);

    frame.render_widget(Paragraph::new("Duration:"), duration_row[0]);
    draw_date_time_input(
        frame,
        duration_row[1],
        &app.timer_duration,
        app.timer_selected_input == TimerSelectedInput::Duration,
    );
}

//pub fn save_event(app: &mut App) {
//    match app.popup {
//        Popup::Agenda(AgendaPopup::AddEvent) | Popup::Agenda(AgendaPopup::EditEvent) => {
//            // 1. Validate inputs
//            let name = app.event_name.text.trim().to_string();
//            if name.is_empty() {
//                app.set_status_message("Event name cannot be empty.".to_string());
//                return;
//            }
//
//            let date = match NaiveDate::parse_from_str(&app.timer_duration.value, "%d-%m-%y") {
//                Ok(date) => date,
//                Err(_) => {
//                    app.set_status_message("Invalid date.".to_string());
//                    return;
//                }
//            };
//
//            let time = if app.event_time.value == "--:--" {
//                None
//            } else {
//                match NaiveTime::parse_from_str(&app.event_time.value, "%H:%M") {
//                    Ok(time) => Some(time),
//                    Err(_) => {
//                        app.set_status_message("Invalid time.".to_string());
//                        return;
//                    }
//                }
//            };
//
//            let event = AgendaEvent {
//                name: name.clone(),
//                date,
//                time,
//                repeat: app.event_repeat,
//            };
//
//            // 2. Perform Add or Edit action
//            if matches!(app.popup, Popup::Agenda(AgendaPopup::AddEvent)) {
//                app.events.push(event);
//            } else if let Some(index) = app.agenda_table_state.selected() {
//                if let Some(existing_event) = app.events.get_mut(index) {
//                    *existing_event = event;
//                }
//            }
//
//            // 3. Sort events chronologically
//            app.events.sort_by(|a, b| {
//                a.date.cmp(&b.date).then_with(|| match (a.time, b.time) {
//                    (Some(t1), Some(t2)) => t1.cmp(&t2),
//                    (Some(_), None) => std::cmp::Ordering::Less,
//                    (None, Some(_)) => std::cmp::Ordering::Greater,
//                    (None, None) => std::cmp::Ordering::Equal,
//                })
//            });
//
//            // 4. Update table selection to keep track of the modified/added event
//            if let Some(index) = app.events.iter().position(|e| e.name == name && e.date == date) {
//                app.agenda_table_state.select(Some(index));
//            }
//        }
//        _ => {}
//    }
//
//    ui::update_repeating_events(&mut app.events);
//    storage::agenda::save_agenda(&app.events).unwrap();
//    app.popup = Popup::None;
//}

fn close_popup(app: &mut App) {
    if app.timer_selected_input == TimerSelectedInput::Name{
        if app.mode == InputMode::Normal {
            app.popup = Popup::None;
        } 
        return
    }

    app.popup = Popup::None;
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    match app.timer_selected_input {
        TimerSelectedInput::Name => {
            let result = app.timer_name.handle_vim_mode(key, &mut app.mode, usize::MAX);

            match result {
                InputResult::Consumed => return,
                InputResult::Ignored => {}
                InputResult::TextChanged => {}
            }
        }

        TimerSelectedInput::Duration => {
            match key.code {
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    app.timer_duration.insert_digit(c);
                }

                KeyCode::Char('h') => {
                    app.timer_duration.move_left();
                }

                KeyCode::Char('l') => {
                    app.timer_duration.move_right();
                }

                KeyCode::Backspace => {
                    app.timer_duration.zero_backspace(); 
                }

                _ => {}
            }
        }
    }

    match key.code {
        KeyCode::Tab | KeyCode::Char('j') => {
            app.timer_selected_input = match app.timer_selected_input {
                TimerSelectedInput::Name if app.mode != InputMode::Insert || key.code == KeyCode::Tab => TimerSelectedInput::Duration,
                TimerSelectedInput::Name => TimerSelectedInput::Name,
                TimerSelectedInput::Duration => TimerSelectedInput::Duration,
            }
        }

        KeyCode::BackTab | KeyCode::Char('k') => {

            app.timer_selected_input = match app.timer_selected_input {
                TimerSelectedInput::Name if app.mode != InputMode::Insert || key.code == KeyCode::BackTab => TimerSelectedInput::Duration,
                TimerSelectedInput::Name => TimerSelectedInput::Name,
                TimerSelectedInput::Duration => TimerSelectedInput::Name,
            }
        }

        //KeyCode::Enter => {
        //    save_event(app);
        //}

        KeyCode::Char('q') | KeyCode::Esc => {
            close_popup(app);
        }

        _ => {}
    }
}

