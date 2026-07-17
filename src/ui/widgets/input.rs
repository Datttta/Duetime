use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::text_input::InputState;
use crate::theme::placeholder_color;

pub fn draw(
    frame: &mut Frame,
    area: Rect,
    input: &InputState,
    placeholder: &str,
) {
    let line = if input.text.is_empty() {
        Line::from(
            Span::styled(
                placeholder,
                Style::default().fg(placeholder_color()),
            )
        )
    } else {
        Line::from(input.text.as_str())
    };

    let paragraph = Paragraph::new(line)
        .block(Block::bordered());

    frame.render_widget(paragraph, area);

    frame.set_cursor_position((
        area.x + 1 + input.cursor as u16,
        area.y + 1,
    ));
}
