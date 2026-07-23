mod app;
mod events;
mod ui;
mod vim_text;
mod tasks;

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
