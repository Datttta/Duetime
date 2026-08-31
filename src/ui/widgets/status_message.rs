use std::time::Instant;

use ratatui::{
    layout::{Rect, Alignment},
    widgets::{Block, Paragraph, Padding},
    Frame,
};

use crate::app::App;

pub fn draw_status_message(
    frame: &mut Frame,
    app: &mut App,
    area: Rect,
) {
    let Some(message) = &app.status_message else {
        return;
    };

    let Some(expires) = app.status_message_until else {
        return;
    };

    if Instant::now() >= expires {
        app.status_message = None;
        app.status_message_until = None;
        return;
    }

    let paragraph = Paragraph::new(message.as_str())
        .alignment(Alignment::Right)
        .block(
            Block::default()
                .padding(Padding::new(0, 2, 0, 0))
        );

    frame.render_widget(paragraph, area);
}
