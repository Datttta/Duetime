use std::{
    fs::File,
    time::{Instant, Duration},
    io,
};
use simplelog::{LevelFilter, WriteLogger}; 
use crate::{
    app::App,
    storage::{current_tasks, current_timers},
};
use chrono::Local;

mod app;
mod events;
mod ui;
mod vim_text;
mod navigation;
mod tasks_table;
mod stopwatch;
mod storage;
mod models;
mod keys_help;
mod suggestions;
mod inbox;
mod agenda;
mod timers;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::new();
    let mut first_render = true;

    let mut last_save = Instant::now();

    let _ = WriteLogger::init(
        LevelFilter::Debug,
        simplelog::Config::default(),
        File::create("debug.log").unwrap(),
    );

    while app.running {
        let today = Local::now().date_naive();

        if today != app.last_agenda_update {
            agenda::ui::remove_expired_events(&mut app.events);
            agenda::ui::update_repeating_events(&mut app.events);
            app.last_agenda_update = today;

            crate::storage::agenda::save_agenda(&app.events).unwrap();
        }

        terminal.draw(|frame| ui::draw(frame, &mut app))?;
        
        if first_render {
            for i in 0..app.tasks.len() {
                let task = &mut app.tasks[i];
                
                if task.status == "IN PROGRESS" {
                    task.status = "STOPPED".into();
                }
            }
            
            first_render = false;
        }

        if last_save.elapsed() >= Duration::from_secs(1) {
            current_tasks::save_current_tasks(&app.tasks).unwrap();
            current_timers::save_current_timers(&app.timers).unwrap();
            last_save = Instant::now();
        }

        events::handle_events(&mut app)?;
    }

    ratatui::restore();
    Ok(())
}
