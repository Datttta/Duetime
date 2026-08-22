use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, Popup, TaskDestination, NewPresetFocus, TasksPopup},
    ui::widgets::{input},
    vim_text::InputMode,
    vim_navigation::NavigationMode,
    move_items::MoveTarget,
    move_items,
    vim_navigation,
    keys_help,
};

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex, Alignment},
    widgets::{Clear, Block, List, ListItem, Padding, Paragraph},
    style::{Style, Color},
    text::Line,
    Frame
};

const PRESET_NAME_WIDTH: u16 = 30;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame, app);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Add Preset")
        .padding(Padding::new(1,1,0,0));

    frame.render_widget(&block, area);

    fn centered_rect(frame: &mut Frame, app: &mut App) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(18),
            Constraint::Length(1), // keys_help
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(50)
        ])
        .flex(Flex::Center)
        .split(vertical[0]);
        
        let keys_help = Paragraph::new(keys_help::keys(app))
            .alignment(Alignment::Center);
        frame.render_widget(keys_help, vertical[1]);
        
        horizontal[0]
    }

    let vertical_inner = Layout::vertical([
        Constraint::Length(3), // preset name
        Constraint::Min(0),    // task list
    ])
    .split(block.inner(area));

    let horizontal_inner = Layout::horizontal([
        Constraint::Length(PRESET_NAME_WIDTH)
    ])
    .flex(Flex::Center)
    .split(vertical_inner[0]);

    let visual_start = app.tasks_visual_start;
    let visual_mode = app.tasks_navigation_mode == NavigationMode::Visual;
    let current = app.preset_task_state.selected();

    let mut tasks: Vec<ListItem> = Vec::new();

    for (index, task) in app.preset_tasks.iter().enumerate() {
        // Insertion line before this task.
        if app.move_state.is_moving()
            && app.move_state.target == Some(MoveTarget::PresetTasks)
            && app.move_state.position == Some(index)
        {
            tasks.push(ListItem::new("────────────────────"));
        }

        let mut item = ListItem::new(Line::from(format!(
            "{} {} - {}",
            task.name,
            task.planned_start.as_deref().unwrap_or(""),
            task.planned_end.as_deref().unwrap_or("")
        )));

        if visual_mode {
            if let (Some(start), Some(end)) = (visual_start, current) {
                let first = start.min(end);
                let last = start.max(end);

                if index >= first && index <= last {
                    item = item.style(
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::White),
                    );
                }
            }
        }

        tasks.push(item);
    }

    // Insertion line after the final task.
    if app.move_state.is_moving()
        && app.move_state.target == Some(MoveTarget::PresetTasks)
        && app.move_state.position == Some(app.preset_tasks.len())
    {
        tasks.push(ListItem::new("────────────────────"));
    }

    input::draw(
        frame,
        horizontal_inner[0],
        &app.preset_name,
        "Preset name",
        app.new_preset_focus == NewPresetFocus::Name,
        app.mode,
    );
    
    let highlight_style = if app.move_state.is_moving() {
        Style::default()
    } else if visual_mode {
        Style::default()
            .fg(Color::Black)
            .bg(Color::White)
    } else {
        Style::default()
    };

    let highlight_symbol = if visual_mode {
        ""
    } else {
        "> "
    };

    let list = List::new(tasks)
        .highlight_style(highlight_style)
        .highlight_symbol(highlight_symbol);


    frame.render_stateful_widget(list, vertical_inner[1], &mut app.preset_task_state);
}

fn delete_preset_task(app: &mut App) {
    if let Some(current) = app.preset_task_state.selected() {
        let (first, last) = if app.tasks_navigation_mode == NavigationMode::Visual {
            if let Some(start) = app.tasks_visual_start{
                (start.min(current), start.max(current))
            } else {
                (current, current)
            }
        } else {
            (current, current)
        };

        app.preset_tasks.drain(first..=last);

        // Keep the selection valid
        if app.preset_tasks.is_empty() {
            app.preset_task_state.select(None);
        } else {
            let new_index = first.min(app.preset_tasks.len() - 1);
            app.preset_task_state.select(Some(new_index));
        }
    }
    
    app.tasks_navigation_mode = NavigationMode::Normal;
    app.pending_command = None;
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    if app.move_state.is_moving()
        && app.move_state.target == Some(MoveTarget::PresetTasks)
    {
        let was_moving = app.move_state.is_moving();
        
        move_items::handle_keys(
            &mut app.move_state,
            &mut app.preset_tasks,
            &mut app.preset_task_state,
            &mut app.pending_command,
            key,
        );

        if was_moving && !app.move_state.is_moving() {
            app.tasks_navigation_mode = NavigationMode::Normal;
            app.tasks_visual_start = None;
        }

        return;

    }

    match key.code {
        KeyCode::Tab => {
            app.new_preset_focus = match app.new_preset_focus {
                NewPresetFocus::Name => NewPresetFocus::Tasks,
                NewPresetFocus::Tasks => NewPresetFocus::Name,
            };
        }

        KeyCode::Enter => {
            if let NewPresetFocus::Name = app.new_preset_focus {
                app.new_preset_focus = NewPresetFocus::Tasks;
                return;
            }

            app.save_preset();
        }

        _ => {
            match app.new_preset_focus {
                NewPresetFocus::Name => {
                    if app.mode == InputMode::Normal {
                        match key.code {
                            KeyCode::Char('a') => {
                                app.add_task(TaskDestination::Preset);
                                return;
                            }

                            KeyCode::Esc => {
                                app.popup = Popup::Tasks(TasksPopup::Presets);
                            }

                            _ => {}
                        }
                    }

                    app.preset_name.handle_key(key, &mut app.mode, (PRESET_NAME_WIDTH - 4).into());
                }

                NewPresetFocus::Tasks => {

                    match key.code {
                        KeyCode::Char('a') => {
                            app.add_task(TaskDestination::Preset);

                            return;
                        }

                        KeyCode::Char('e') => {
                            app.edit_preset_task();
                            return;
                        }

                        KeyCode::Char('x') => {
                            if app.tasks_navigation_mode == NavigationMode::Visual {
                                move_items::start(
                                    &mut app.move_state,
                                    app.preset_task_state.selected(),
                                    app.tasks_visual_start,
                                    app.preset_tasks.len(),
                                    MoveTarget::PresetTasks,
                                );

                                return;
                            }
                        }

                        KeyCode::Char('d') => {
                            if app.pending_command == Some('d') {
                                delete_preset_task(app);
                            } else {
                                app.pending_command = Some('d');
                            }
                        }

                        KeyCode::Char('q') => {
                            app.popup = Popup::Tasks(TasksPopup::Presets);
                        }

                        _ => {}
                    }

                    let mut selected = app.preset_task_state.selected();

                    let handled = vim_navigation::handle(
                        key,
                        &mut app.pending_command,
                        &mut selected,
                        app.preset_tasks.len(),
                        &mut app.tasks_navigation_mode,
                        &mut app.tasks_visual_start,
                    );

                    app.preset_task_state.select(selected);

                    if handled {
                        return;
                    }
                }
            }
        }
    }
}
