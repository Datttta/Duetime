use std::{
    fs::File,
    io,
};
use simplelog::{LevelFilter, WriteLogger}; 
use crate::app::App;

mod app;
mod events;
mod ui;
mod vim_text;
mod vim_navigation;
mod tasks;
mod stopwatch;
mod storage_preset;
mod storage_known_tasks;
mod storage_current_tasks;
mod models;
mod keys_help;
mod suggestions;
mod move_items;

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
