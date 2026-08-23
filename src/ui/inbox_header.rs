use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

pub fn draw(frame: &mut Frame, area: Rect) {
    let columns = Layout::horizontal([
        Constraint::Percentage(2), // extra
        Constraint::Percentage(78), // Item    
        Constraint::Percentage(20), // Priority
    ])
    .flex(Flex::Start)
    .split(area);

    frame.render_widget(Paragraph::new("Item"), columns[1]);
    frame.render_widget(Paragraph::new("Priority"), columns[2]);
}

