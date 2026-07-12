use std::io;
// 1. Import Table and Row instead of Paragraph and Layout
use ratatui::widgets::{Block, Padding, Table, Row, Paragraph}; 
use ratatui::layout::{Constraint, Layout, Flex};
use crossterm::event::{self, Event, KeyCode};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(draw)?;

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
    // 1. Draw the border first
    let screen_border = Block::bordered()
        .title(" Duetime ")
        .padding(Padding::new(2, 4, 1, 1));

    let inner_area = screen_border.inner(frame.area());
    frame.render_widget(screen_border, frame.area());

    // ---------------------------------------------------------
    // STEP 2: Split the inner area into Top (Header) and Bottom (Tasks)
    // ---------------------------------------------------------
    let vertical_chunks = Layout::vertical([
        Constraint::Length(1), // Top chunk: Exactly 1 line tall for the header
        Constraint::Length(1), // Middle chunk: 1 empty line as a separator/spacer
        Constraint::Min(0),    // Bottom chunk: The rest of the screen for tasks
    ])
    .split(inner_area);

    // ---------------------------------------------------------
    // STEP 3: Draw your custom Header (using the top chunk)
    // You can manually align these however you want!
    // ---------------------------------------------------------
    let header_columns = Layout::horizontal([
        Constraint::Length(8), // extra
        Constraint::Length(15), // TO-DO: 5    
        Constraint::Length(11), // Status: 6 
        Constraint::Length(15), // Plan start: 10 
        Constraint::Length(13), // Plan end: 8 
        Constraint::Length(13), // Act start: 8 
        Constraint::Length(11), // Act end: 6 
        Constraint::Length(7),  // Elapsed: 7 
    ])
    .flex(Flex::Start)
    .split(vertical_chunks[0]); // Draw into the top row

    frame.render_widget(Paragraph::new("TO-DO"), header_columns[1]);
    frame.render_widget(Paragraph::new("Status"), header_columns[2]);
    frame.render_widget(Paragraph::new("Plan start"), header_columns[3]);
    frame.render_widget(Paragraph::new("Plan end"), header_columns[4]);
    frame.render_widget(Paragraph::new("Act start"), header_columns[5]);
    frame.render_widget(Paragraph::new("Act end"), header_columns[6]);
    frame.render_widget(Paragraph::new("Elapsed"), header_columns[7]);

    // ---------------------------------------------------------
    // STEP 4: Draw the Tasks (using the bottom chunk)
    // These get COMPLETELY DIFFERENT constraints
    // ---------------------------------------------------------
    let task_widths = [
        Constraint::Length(22), // TO-DO
        Constraint::Length(12), // Status
        Constraint::Length(13), // Plan start
        Constraint::Length(13), // Plan end
        Constraint::Length(11), // Act start
        Constraint::Length(9), // Act end
        Constraint::Length(8),  // Elapsed
    ];

    let rows = vec![
        Row::new(vec![
            "My first task", "PENDING", "14:00", "16:00", "--:--", "--:--", "--:--:--"
        ]),
        Row::new(vec![
            "Second task", "DONE", "09:00", "10:00", "09:00", "09:45", "00:45:00"
        ]),
    ];

    // Notice we do NOT use .header() or .block() here. 
    // We just create a raw table of data.
    let tasks_table = Table::new(rows, task_widths);

    // Render the table into the bottom chunk!
    frame.render_widget(tasks_table, vertical_chunks[2]);
}
