use std::io;
use ratatui::widgets::{Block, Paragraph, Padding};
use ratatui::layout::{Constraint, Layout};
use crossterm::event::{self, Event, KeyCode};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(draw)?;

        // Wait for user input
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q'){
                    break;
                }
            }
        }
    }

    ratatui::restore();

    Ok(())
}

fn draw(frame: &mut ratatui::Frame) {
    // 1. Create the border block
    let screen_border = Block::bordered()
        .title(" Duetime ")
        .padding(Padding::new(10, 4, 1, 1));

    // 2. Calculate the area inside the border and padding
    // This is where we will safely put our text
    let inner_area = screen_border.inner(frame.area());

    // 3. Render the empty border block to the full screen FIRST
    frame.render_widget(screen_border, frame.area());

    let columns = Layout::horizontal([
        Constraint::Length(15), // TO-DO: 5    
        Constraint::Length(11), // Status: 6 
        Constraint::Length(15), // Plan start: 10 
        Constraint::Length(13), // Plan end: 8 
        Constraint::Length(13), // Act start: 8 
        Constraint::Length(11), // Act end: 6 
        Constraint::Length(7),  // Elapsed: 7 
    ])
    .split(inner_area);

    // 5. Create your two labels (notice we don't attach the block to them anymore)
    let TODO = Paragraph::new("TO-DO");
    let Status = Paragraph::new("Status");
    let Plan_start = Paragraph::new("Plan start");
    let Plan_end = Paragraph::new("Plan end");
    let Act_start = Paragraph::new("Act start");
    let Act_end = Paragraph::new("Act end");
    let Elapsed = Paragraph::new("Elapsed");

    // 6. Render each label into its specific column chunk!
    frame.render_widget(TODO, columns[0]);
    frame.render_widget(Status, columns[1]);
    frame.render_widget(Plan_start, columns[2]);
    frame.render_widget(Plan_end, columns[3]);
    frame.render_widget(Act_start, columns[4]);
    frame.render_widget(Act_end, columns[5]);
    frame.render_widget(Elapsed, columns[6]);
}
