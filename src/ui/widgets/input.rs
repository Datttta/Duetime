use crossterm::cursor::SetCursorStyle;
use crossterm::execute;

use crate::vim_text::InputState;
use crate::vim_text::InputMode;
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
    mode: InputMode,
    bordered: bool,
) {
    let visible_width = area.width.saturating_sub(3) as usize;

    let start = input.cursor.saturating_sub(visible_width.saturating_sub(1));

    let visible: String = input
        .text
        .chars()
        .skip(start)
        .take(visible_width)
        .collect();

    let line = if input.text.is_empty() {
        Line::from(
            Span::styled(
                placeholder,
                Style::default().fg(placeholder_color()),
            )
        )
    } else {
        Line::from(visible)
    };

    let block = if bordered {
        Block::bordered().padding(Padding {
            left: 1,
            right: 1,
            top: 0,
            bottom: 0,
        })
    } else {
        Block::new().padding(Padding {
            left: 1,
            right: 1,
            top: 0,
            bottom: 0,
        })
    };

    let paragraph = Paragraph::new(line).block(block);

    frame.render_widget(paragraph, area);
    
    if is_selected {
        let cursor_x = input.cursor.saturating_sub(start);

        frame.set_cursor_position((
            area.x + 2 + cursor_x as u16,
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

pub fn ellipsize(text: &str, max: usize) -> String {
    let len = text.chars().count();

    if len <= max {
        return text.to_string();
    }

    text.chars()
        .take(max.saturating_sub(1))
        .collect::<String>()
        + "…"
}
