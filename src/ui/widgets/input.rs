use crossterm::{
    cursor::SetCursorStyle,
    execute,
};

use crate::{
    input::vim_text::{InputMode, InputState},
    ui::colors::{placeholder_color, text_selection_color},
};

use ratatui::{
    widgets::{Block, Paragraph, Padding},
    style::{Style, Color},
    text::{Line, Span},
    layout::Rect,
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

    let chars: Vec<char> = input.text.chars().collect();

    let line = if input.text.is_empty() {
        Line::from(
            Span::styled(
                placeholder,
                Style::default().fg(placeholder_color()),
            )
        )
    } else {
        let (min_idx, max_idx) = if mode == InputMode::Visual {
            if let Some(selection_start) = input.visual_start {
                (
                    selection_start.min(input.cursor),
                    selection_start.max(input.cursor),
                )
            } else {
                (0, 0)
            }
        } else {
            (0, 0)
        };

        let spans: Vec<Span> = chars
            .iter()
            .enumerate()
            .skip(start)
            .take(visible_width)
            .map(|(index, c)| {
                let selected =
                    mode == InputMode::Visual
                        && input.visual_start.is_some()
                        && index >= min_idx
                        && index <= max_idx;

                if selected {
                    Span::styled(
                        c.to_string(),
                        Style::default()
                            .fg(Color::Black)
                            .bg(text_selection_color()),
                    )
                } else {
                    Span::raw(c.to_string())
                }
            })
            .collect();

        Line::from(spans)
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
