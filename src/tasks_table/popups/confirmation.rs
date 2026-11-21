use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, Paragraph, Padding, Wrap},
    text::{Line},
    Frame
};

use crate::{
    app::{App},
    ui::widgets::keys_help,
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame, app);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title(" Confirmation ")
        .padding(Padding::new(1,1,0,0));

    fn centered_rect(frame: &mut Frame, app: &mut App) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(4),
            Constraint::Length(1), // keys help
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal_box = Layout::horizontal([
            Constraint::Length(33),
        ])
        .flex(Flex::Center)
        .split(vertical[0]);

        let horizontal_help = Layout::horizontal([
            Constraint::Length(15)
        ])
        .flex(Flex::Center)
        .split(vertical[1]);

        frame.render_widget(Clear, horizontal_help[0]);

        let keys_help = Paragraph::new(keys_help::keys(app))
            .alignment(Alignment::Center);
            
        frame.render_widget(keys_help, horizontal_help[0]);
        
        horizontal_box[0]
    }


    let inner = block.inner(area);
    frame.render_widget(block, area);
    
    let paragraph = Paragraph::new(vec![
        Line::from("Overwrite the current tasks?"),
    ])
    .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, inner);
}

pub fn handle_keys (app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('n') | KeyCode::Char('N') => {
            app.popup = app.last_popup.clone();
        }
        
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.apply_preset();
            app.popup = app.last_popup.clone();
        }

        _ => {}
    }
}
