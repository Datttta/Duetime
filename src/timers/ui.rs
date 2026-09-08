use crate::{
    ui::{
        widgets::input::ellipsize,
        theme::{task_selection_color, unfocused_panel},
    },
    app::{App, Popup, Panel, Priority},
    navigation::vim_navigation::NavigationMode,
};

use ratatui::{
    layout::{Constraint, Rect, Layout, Flex},
    widgets::{Row, Table, Cell, Paragraph, Padding, Block},
    style::{Style, Color},
    text::Line,
    Frame,
};

use serde::{Deserialize, Serialize};

pub fn draw_timers_panel (
    frame: &mut Frame,
    area: Rect,
    app: &mut App
) {
    let border_color = if app.focused_panel == Panel::Timers {
        Color::White
    } else {
        unfocused_panel()
    };

    let border = Block::bordered()
        .title(" Timers ")
        .border_style(Style::default().fg(border_color))
        .padding(Padding::new(0, 0, 1, 0));
    
    let inner = border.inner(area);

    frame.render_widget(border, area);
}
