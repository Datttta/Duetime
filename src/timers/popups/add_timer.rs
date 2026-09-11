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
    timers::{
        ui::TimerInfo,
        ui,
    },
    app::{App, Popup, TimersPopup, TimerSelectedInput},
    vim_text::{InputResult, InputMode},
    countdown::Countdown,
    storage,
    keys_help,
};

use chrono::{NaiveDate, NaiveTime};
use std::time::Duration;
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
        Constraint::Length(10), // Width for label text
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

fn parse_duration(input: &str) -> Option<Duration> {
    let parts: Vec<&str> = input.split(':').collect();

    if parts.len() != 3 {
        return None;
    }

    let hours: u64 = parts[0].parse().ok()?;
    let minutes: u64 = parts[1].parse().ok()?;
    let seconds: u64 = parts[2].parse().ok()?;

    if minutes >= 60 || seconds >= 60 {
        return None;
    }

    Some(Duration::from_secs(
        hours * 3600 + minutes * 60 + seconds,
    ))
}

pub fn save_timer(app: &mut App) {
    match app.popup {
        Popup::Timers(TimersPopup::AddTimer) | Popup::Timers(TimersPopup::EditTimer) => {

            let name = app.timer_name.text.trim().to_string();

            let duration = match parse_duration(&app.timer_duration.value) {
                Some(duration) => duration,
                None => {
                    app.set_status_message("Invalid duration.".to_string());
                    return;
                }
            };

            let timer = TimerInfo {
                name,
                duration,
                status: "READY".to_string(),
                countdown: Countdown::new(duration),
            };

            if matches!(app.popup, Popup::Timers(TimersPopup::AddTimer)) {
                app.timers.push(timer);
            } else if let Some(index) = app.timers_list_state.selected() {
                if let Some(existing_timer) = app.timers.get_mut(index) {
                    *existing_timer = timer;
                }
            }

            if let Some(index) = app.timers.len().checked_sub(1) {
                app.timers_list_state.select(Some(index));
            }

            app.popup = Popup::None;
        }

        _ => {}
    }
}

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
                TimerSelectedInput::Duration => TimerSelectedInput::Name,
            }
        }

        KeyCode::BackTab | KeyCode::Char('k') => {

            app.timer_selected_input = match app.timer_selected_input {
                TimerSelectedInput::Name if app.mode != InputMode::Insert || key.code == KeyCode::BackTab => TimerSelectedInput::Duration,
                TimerSelectedInput::Name => TimerSelectedInput::Name,
                TimerSelectedInput::Duration => TimerSelectedInput::Name,
            }
        }

        KeyCode::Enter => {
            save_timer(app);
        }

        KeyCode::Char('q') | KeyCode::Esc => {
            close_popup(app);
        }

        _ => {}
    }
}

