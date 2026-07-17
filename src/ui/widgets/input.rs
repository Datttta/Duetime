use ratatui::{
    layout::Rect,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::text_input::InputState;

pub fn draw(
    frame: &mut Frame,
    area: Rect,
    input: &InputState,
    title: &str,
) {
    let paragraph = Paragraph::new(input.text.as_str())
        .block(Block::bordered().title(title));

    frame.render_widget(paragraph, area);

    frame.set_cursor_position((
        area.x + 1 + input.cursor as u16,
        area.y + 1,
    ));
}
