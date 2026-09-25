use crossterm::event::{KeyCode, KeyEvent};
//use log::info;

use ratatui::{
    widgets::{Clear, Block, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    style::{Color, Style},
    text::{Line, Span},
    Frame,
};

use crate::{
    navigation::{
        vim_navigation::NavigationMode,
        vim_navigation,
    },
    app::{App, Popup},
    agenda::actions,
    keys_help, Panel, search,
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame, app);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("All Events")
        .padding(Padding::new(1, 1, 1, 0));

    let inner = block.inner(area);
    
    let chunks = ratatui::layout::Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .split(inner);

    let inner = block.inner(area);
    frame.render_widget(&block, area);

    let event_indices: Vec<usize> = (0..app.events.len()).collect();
    let is_visual = app.n_mode == NavigationMode::Visual;
    let name_length: u16 = 45;

    let visible_height = inner.height as usize;
    let item_count = app.events.len();
    let scroll_offset = app.all_events_table_state.offset();

    // Narrow the table width if a scrollbar is needed to avoid highlight bleeding
    let mut table_area = inner;
    if item_count > visible_height {
        table_area.width = table_area.width.saturating_sub(2);
    }

    let match_info = if app.agenda_search.matches.is_empty() {
        "0/0".to_string()
    } else {
        format!(
            "{}/{}",
            app.agenda_search.current_match + 1,
            app.agenda_search.matches.len()
        )
    };

    if app.search_panel == Some(Panel::Agenda) 
       || app.n_mode == NavigationMode::SearchNavigation 
    {
        let search_line = Line::from(vec![
            Span::raw("/"),
            Span::raw(&app.agenda_search.query),
        ]);

        frame.render_widget(search_line, chunks[1]);
        
        frame.render_widget(
            Paragraph::new(match_info)
                .alignment(Alignment::Right)
                .block(
                    Block::default()
                        .padding(Padding::right(2))
                ),
            chunks[1],
        );

    } else {
        let search_line = Paragraph::new(format!(""))
            .style(Style::default().fg(Color::White));

        frame.render_widget(search_line, chunks[1]);
    }

    // Scrollbar rendering for the popup
    if item_count > visible_height {
        let max_scroll = item_count.saturating_sub(visible_height);

        let mut scrollbar_state = ScrollbarState::new(max_scroll + 1)
            .position(scroll_offset)
            .viewport_content_length(visible_height);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .thumb_symbol("▊")
            .track_symbol(Some(""))
            .begin_symbol(Some(""))
            .end_symbol(Some(""));

        let scrollbar_area = Rect {
            x: area.x + area.width - 2,
            y: inner.y,
            width: 1,
            height: inner.height,
        };

        frame.render_stateful_widget(
            scrollbar,
            scrollbar_area,
            &mut scrollbar_state,
        );
    }

    crate::agenda::ui::draw_events(
        frame,
        table_area,
        app,
        &event_indices,
        is_visual,
        name_length,
    );
}

fn centered_rect(frame: &mut Frame, app: &mut App) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Length(21),
        Constraint::Length(1), // keys_help
    ])
    .flex(Flex::Center)
    .split(frame.area());

    let horizontal = Layout::horizontal([
        Constraint::Length(85)
    ])
    .flex(Flex::Center)
    .split(vertical[0]);
    
    let keys_help = Paragraph::new(keys_help::keys(app)) 
        .alignment(Alignment::Center);
    frame.render_widget(keys_help, vertical[1]);
    
    horizontal[0]
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    // --------------------------------------------------
    // Search typing mode
    // --------------------------------------------------
    if app.n_mode == NavigationMode::Search {
        match search::handle_search_input(&mut app.agenda_search, key) {
            search::SearchInputResult::Continue => {
                actions::search_all_events(app);
            }
            search::SearchInputResult::Navigate => {
                app.n_mode = NavigationMode::SearchNavigation;
            }
            search::SearchInputResult::Cancel => {
                search::clear(&mut app.agenda_search);
                app.n_mode = NavigationMode::Normal;
                app.pending_command = None;
                app.search_panel = None;
            }
        }
        return;
    }

    // --------------------------------------------------
    // Search navigation mode
    // --------------------------------------------------
    if app.n_mode == NavigationMode::SearchNavigation {
        match search::handle_search_navigation(&mut app.agenda_search, key) {
            search::SearchNavigationResult::Continue => {
                if let Some(&index) = app
                    .agenda_search
                    .matches
                    .get(app.agenda_search.current_match)
                {
                    app.all_events_table_state.select(Some(index));
                }
            }
            search::SearchNavigationResult::Cancel => {
                app.n_mode = NavigationMode::Normal;
                app.pending_command = None;
                app.search_panel = None;
            }
        }
        return;
    }

    // --------------------------------------------------
    // Start search
    // --------------------------------------------------
    if app.n_mode == NavigationMode::Normal
        && app.focused_panel == Panel::Agenda
        && key.code == KeyCode::Char('/')
    {
        search::clear(&mut app.agenda_search);
        app.n_mode = NavigationMode::Search;
        app.search_panel = Some(Panel::Agenda);
        app.pending_command = None;
        return;
    }

    // normal navigation
    let mut selected = app.all_events_table_state.selected();

    let handled = vim_navigation::handle(
        key,
        &mut app.pending_command,
        &mut selected,
        app.events.len(),
        &mut app.n_mode,
        &mut app.n_visual_start,
    );

    app.all_events_table_state.select(selected);

    if handled {
        return;
    }

    match key.code {
        KeyCode::Char('a') => {
            app.last_popup = app.popup.clone();
            actions::add_event(app);
        }

        KeyCode::Char('e') => {
            app.last_popup = app.popup.clone();
            actions::edit_event_catalog(app);
        }
        
        KeyCode::Char('i') => {
            app.last_popup = app.popup.clone();
            actions::event_info(app);
        }
        
        KeyCode::Char('l') => {
            actions::all_events(app);
        }

        KeyCode::Char('d') => {
            if app.pending_command == Some('d') {
                actions::delete_event(app);
                app.pending_command = None;
            } else {
                app.pending_command = Some('d')
            }
        }
        
        KeyCode::Char('q') | KeyCode::Esc => {
            app.popup = Popup::None
        }

        _ => {}
    }
}
