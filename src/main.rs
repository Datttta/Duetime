//use log::info;
use simplelog::{LevelFilter, WriteLogger};
use std::env;

use std::{
    fs::File,
    io,
    sync::{Arc, atomic::AtomicBool},
    time::{Duration, Instant},
};

use crate::{
    app::{App, Panel, Popup, TimersPopup},
    core::{events, sounds::alarm_sound, update},
    storage::{current_tasks, current_timers},
};

use crossterm::{
    event::{EnableBracketedPaste, EnableFocusChange},
    execute,
};

mod app;
mod core;
mod input;
mod models;
mod storage;
mod ui;

mod agenda;
mod inbox;
mod tasks_table;
mod timers;

mod search;
mod suggestions;

fn main() -> io::Result<()> {
    let current_dir = env::current_dir().expect("Failed to get current directory");

    let cargo_toml_exists = match current_dir.parent() {
        Some(parent_dir) => parent_dir.join("Cargo.toml").try_exists().unwrap_or(false),
        None => false,
    };

    if cargo_toml_exists {
        let _ = WriteLogger::init(
            LevelFilter::Debug,
            simplelog::Config::default(),
            File::create("debug.log").unwrap(),
        );
    }

    if update::check_for_updates() {
        return Ok(());
    }

    let mut terminal = ratatui::init();

    let mut app = App::new();
    let mut first_render = true;
    let mut last_save = Instant::now();

    execute!(io::stdout(), EnableFocusChange, EnableBracketedPaste)?;

    while app.running {
        agenda::ui::update_repeating_events(&mut app.events);

        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        // set tasks table stopwatch to stopped if the app was closed while IN PROGRESS
        if first_render {
            for i in 0..app.tasks.len() {
                let task = &mut app.tasks[i];

                if task.status == "IN PROGRESS" {
                    task.status = "STOPPED".into();
                }
            }

            first_render = false;
        }

        // check finished timers
        for (index, timer) in app.timers.iter_mut().enumerate() {
            if timer.countdown.remaining() == Duration::ZERO && app.active_alarm.is_none() {
                app.popup = Popup::Timers(TimersPopup::TimerFinished(index));

                let is_playing = Arc::new(AtomicBool::new(true));
                alarm_sound(Arc::clone(&is_playing));

                app.active_alarm = Some(is_playing);
            }
        }

        // save to files every second
        if last_save.elapsed() >= Duration::from_secs(1) {
            current_tasks::save_current_tasks(&app.tasks).unwrap();
            current_timers::save_current_timers(&app.timers).unwrap();
            last_save = Instant::now();
        }

        if !app.terminal_focus {
            app.focused_panel = Panel::None;
        }

        events::handle_events(&mut app)?;
    }

    ratatui::restore();
    Ok(())
}
