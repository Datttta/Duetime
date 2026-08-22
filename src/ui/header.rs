// header.rs

use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

pub fn draw(frame: &mut Frame, area: Rect) {
    let columns = Layout::horizontal([
        Constraint::Length(11), // extra
        Constraint::Length(16), // TO-DO: 5    
        Constraint::Length(11), // Status: 6 
        Constraint::Length(15), // Plan start: 10 
        Constraint::Length(13), // Plan end: 8 
        Constraint::Length(13), // Act start: 8 
        Constraint::Length(11), // Act end: 6 
        Constraint::Length(7),  // Elapsed: 7 
    ])
    .flex(Flex::Start)
    .split(area);

    frame.render_widget(Paragraph::new("TO-DO"), columns[1]);
    frame.render_widget(Paragraph::new("Status"), columns[2]);
    frame.render_widget(Paragraph::new("Plan start"), columns[3]);
    frame.render_widget(Paragraph::new("Plan end"), columns[4]);
    frame.render_widget(Paragraph::new("Act start"), columns[5]);
    frame.render_widget(Paragraph::new("Act end"), columns[6]);
    frame.render_widget(Paragraph::new("Elapsed"), columns[7]);
}
