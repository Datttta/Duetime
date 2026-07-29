use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Popup, TaskDestination, SelectedInput, NewPresetFocus};
use crate::ui::widgets::input;
use crate::vim_text::InputMode;
use crate::ui::popup::presets::Preset;
use crate::vim_navigation;

use ratatui::{
    layout::{Rect, Constraint, Layout, Flex},
    widgets::{Clear, Block, List, ListItem, Padding},
    text::Line,
    Frame
};

const PRESET_NAME_WIDTH: u16 = 30;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(frame);

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .title("Config Preset")
        .padding(Padding::new(1,1,0,0));

    frame.render_widget(&block, area);

    fn centered_rect(frame: &Frame) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length(18),
        ])
        .flex(Flex::Center)
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(50)
        ])
        .flex(Flex::Center)
        .split(vertical[0]);
        
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

    let tasks: Vec<ListItem> = app
        .preset_tasks
        .iter()
        .map(|task| {
            ListItem::new(Line::from(format!(
                "{} {} - {}",
                task.name,
                task.planned_start.as_deref().unwrap_or(""),
                task.planned_end.as_deref().unwrap_or("")
            )))
        })
        .collect();

    input::draw(
        frame,
        horizontal_inner[0],
        &app.preset_name,
        "Preset name",
        app.new_preset_focus == NewPresetFocus::Name,
        app.mode,
    );

    let list = List::new(tasks.clone())
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, vertical_inner[1], &mut app.preset_task_state);
}

fn save_preset(app: &mut App) {
    if let Some(index) = app.edit_preset {
        app.presets[index].name = app.preset_name.text.clone();
        app.presets[index].tasks = std::mem::take(&mut app.preset_tasks);

        app.edit_preset = None;
    } else {
        let preset = Preset {
            id: app.next_id,
            name: app.preset_name.text.clone(),
            tasks: std::mem::take(&mut app.preset_tasks),
        };

        app.next_id += 1;
        app.presets.push(preset);
    }

    app.preset_name.clear();
    app.popup = Popup::Presets;
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Tab => {
            app.new_preset_focus = match app.new_preset_focus {
                NewPresetFocus::Name => NewPresetFocus::Tasks,
                NewPresetFocus::Tasks => NewPresetFocus::Name,
            };
        }

        KeyCode::Enter => {
            save_preset(app);

            app.task_name.clear();
            app.planned_start.clear();
            app.planned_end.clear();

            if app.preset_state.selected().is_none() && !app.presets.is_empty() {
                app.preset_state.select(Some(0));
            }
        }

        _ => {
            match app.new_preset_focus {


                NewPresetFocus::Name => {
                    if app.mode == InputMode::Normal {
                        match key.code {
                            KeyCode::Char('a') => {
                                app.popup = Popup::AddTask;

                                app.task_destination = TaskDestination::Preset;

                                app.task_name.clear();
                                app.planned_start.clear();
                                app.planned_end.clear();

                                app.selected_input = SelectedInput::TaskName;
                                app.mode = InputMode::Insert;

                                return;
                            }

                            KeyCode::Esc => {
                                app.popup = Popup::Presets;
                            }

                            _ => {}
                        }
                    }

                    app.preset_name.handle_key(key, &mut app.mode, (PRESET_NAME_WIDTH - 4).into());
                }

                NewPresetFocus::Tasks => {

                    match key.code {
                        KeyCode::Char('a') => {
                            app.popup = Popup::AddTask;

                            app.task_destination = TaskDestination::Preset;

                            app.task_name.clear();
                            app.planned_start.clear();
                            app.planned_end.clear();

                            app.selected_input = SelectedInput::TaskName;
                            app.mode = InputMode::Insert;

                            return;
                        }

                        KeyCode::Char('e') => {
                            if let Some(index) = app.preset_task_state.selected() {
                                app.popup = Popup::AddTask;

                                app.task_destination = TaskDestination::EditPresetTask(index);

                                let preset = &app.preset_tasks[index];

                                app.task_name.text = preset.name.clone();
                                app.planned_start.text = preset.planned_start.clone().unwrap_or_default();
                                app.planned_end.text = preset.planned_end.clone().unwrap_or_default();

                                app.task_name.cursor = app.task_name.text.len();
                                app.planned_start.cursor = app.planned_start.text.len();
                                app.planned_end.cursor = app.planned_end.text.len();

                                app.mode = InputMode::Normal;
                                app.selected_input = SelectedInput::TaskName;

                                return;
                            }
                        }

                        KeyCode::Char('d') => {
                            if app.pending_command == Some('d') {
                                if let Some(index) = app.preset_task_state.selected() {
                                    app.preset_tasks.remove(index);

                                    // Keep the selection valid
                                    if app.preset_tasks.is_empty() {
                                        app.preset_task_state.select(None);
                                    } else {
                                        let new_index = index.min(app.preset_tasks.len() - 1);
                                        app.preset_task_state.select(Some(new_index));
                                    }
                                }

                                app.pending_command = None;
                            } else {
                                app.pending_command = Some('d');
                            }
                        }

                        KeyCode::Esc => {
                            app.popup = Popup::Presets;
                        }

                        _ => {}
                    }

                    let mut selected = app.preset_task_state.selected();

                    let handled = vim_navigation::handle(
                        key,
                        &mut app.pending_command,
                        &mut selected,
                        app.preset_tasks.len(),
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
