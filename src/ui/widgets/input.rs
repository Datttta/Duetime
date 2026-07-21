use ratatui::{
    layout::Rect,
    style::{Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Padding},
    Frame,
};

use crate::vim::InputState;
use crate::ui::theme::placeholder_color;

pub fn draw(
    frame: &mut Frame,
    area: Rect,
    input: &InputState,
    placeholder: &str,
    is_selected: bool,
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

    let paragraph = Paragraph::new(line).block(
        Block::bordered().padding(Padding {
            left: 1,
            right: 0,
            top: 0,
            bottom: 0,
        })
    );

    frame.render_widget(paragraph, area);
    
    if is_selected {
        frame.set_cursor_position((
            area.x + 2 + input.cursor as u16,
            area.y + 1,
        ));
    }
}
