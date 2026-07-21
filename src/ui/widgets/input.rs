use crossterm::cursor::SetCursorStyle;
use crossterm::execute;

use crate::vim::InputState;
use crate::vim::InputMode;
use crate::ui::theme::placeholder_color;

use ratatui::{
    layout::Rect,
    style::{Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Padding},
    Frame,
};

pub fn draw(
    frame: &mut Frame,
    area: Rect,
    input: &InputState,
    placeholder: &str,
    is_selected: bool,
    mode: InputMode
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

        match mode {
            InputMode::Insert => {
                execute!(
                    std::io::stdout(),
                    SetCursorStyle::BlinkingBar
                ).unwrap();
            }

            InputMode::Normal => {
                execute!(
                    std::io::stdout(),
                    SetCursorStyle::SteadyBlock
                ).unwrap();
            }

            InputMode::Visual => {
                execute!(
                    std::io::stdout(),
                    SetCursorStyle::SteadyBlock
                ).unwrap();
            }
        }
    }
}
