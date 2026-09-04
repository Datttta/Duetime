use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, Padding, Paragraph},
    text::{Line, Span},
    style::{Style, Color, Modifier},
    Frame
};

use crate::{
    ui::widgets::input,
    vim_text::{InputResult, InputMode},
    app::{App, Popup, InboxPopup, InboxSelectedInput, Priority},
    inbox::ui::InboxItemInfo,
    keys_help,
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame, app);

    frame.render_widget(Clear, area);
    
    let block = Block::bordered()
        .title("Add Event")
        .padding(Padding::new(2, 2, 0, 0));
    
    let inner = block.inner(area);

    frame.render_widget(block, area);
    
    fn centered_rect(frame: &mut Frame, app: &mut App) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(15),
            Constraint::Length(1), // keys_help
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(50),
        ])
        .flex(Flex::Center)
        .split(vertical[0]);

        let keys_help = Paragraph::new(keys_help::keys(app))
                .alignment(Alignment::Center);
        frame.render_widget(keys_help, vertical[1]);
        
        horizontal[0]
    }

    let vertical = Layout::vertical([
        Constraint::Length(10), // input height
    ])
    .split(inner);

    let item_features = Layout::vertical([
        Constraint::Length(3), // event name
        Constraint::Length(1), // event date
        Constraint::Length(1), // event time
        Constraint::Length(1), // repeat
    ])
    .flex(Flex::Center)
    .split(vertical[0]);

    let focused = app.agenda_selected_feature == AgendaSelectedInput::InboxItemInput;
}
