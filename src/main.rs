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
mod move_task;

use std::io;
use app::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::new();

    while app.running {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        events::handle_events(&mut app)?;
    }

    ratatui::restore();
    Ok(())
}
