use std::{
    fs::File,
    io,
};
use simplelog::{LevelFilter, WriteLogger}; 
use crate::app::App;
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

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::new();
    let mut first_render = true;

    let _ = WriteLogger::init(
        LevelFilter::Debug,
        simplelog::Config::default(),
        File::create("debug.log").unwrap(),
    );

    while app.running {
        let today = Local::now().date_naive();

        if today != app.last_agenda_update {
            agenda::actions::remove_expired_events(&mut app.events);
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

        events::handle_events(&mut app)?;
    }

    ratatui::restore();
    Ok(())
}
