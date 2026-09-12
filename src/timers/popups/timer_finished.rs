use crossterm::event::{KeyCode, KeyEvent};
use log::info;

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, Paragraph, Padding, Wrap},
    text::{Line},
    Frame
};

use crate::{
    app::{App, Popup, TimersPopup}, 
};

pub fn draw(
    frame: &mut Frame,
    app: &mut App,
    index: usize
) {
    let area = centered_rect(frame);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .padding(Padding::new(1,1,0,0));

    frame.render_widget(&block, area);

    fn centered_rect(frame: &mut Frame) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(9),
            Constraint::Length(1), // keys help
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(30),
        ])
        .flex(Flex::Center)
        .split(vertical[0]);
        
        horizontal[0]
    }

    let timer = &app.timers[index];

    let inner = block.inner(area);
    frame.render_widget(block, area);
        
    let paragraph = Paragraph::new(vec![
        Line::from("TIME'S UP:").alignment(Alignment::Center),
        Line::from(" "),
        Line::from(timer.name.as_str()).alignment(Alignment::Center), 
    ]);

    frame.render_widget(paragraph, inner);
}

pub fn handle_keys (app: &mut App, key: KeyEvent) {
    match app.popup {
        Popup::Timers(TimersPopup::TimerFinished(index)) => {
            match key.code {
                KeyCode::Char('q') | KeyCode::Enter => {
                    info!("index of itmere: {:?}", index);
                    app.timers.remove(index);

                    if app.timers.is_empty() {
                        app.timers_list_state.select(None);
                    } else if index >= app.timers.len() {
                        app.timers_list_state
                            .select(Some(app.timers.len() - 1));
                    } else {
                        app.timers_list_state.select(Some(index));
                    }

                    app.popup = Popup::None;
                }

                _ => {}
            }
        }

        _ => {}
    }
}

