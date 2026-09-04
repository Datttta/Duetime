use std::time::Duration;

use crate::{
    vim_text::{InputMode},
    app::{NewPresetFocus, Popup, TasksTablePopup, TaskDestination, TaskSelectedInput, Priority},
    tasks_table::ui::TaskInfo,
    models::{TaskTemplate, Preset},
    storage::{current_tasks, preset},
    App,
};

impl App {
    // =================== POPUPS =======================

    pub fn task_add (&mut self, destination: TaskDestination) {
        self.task_destination = destination;

        self.task_name.clear();
        self.planned_start.clear();
        self.planned_end.clear();

        self.tasks_selected_input = TaskSelectedInput::TaskName;
        self.mode = InputMode::Insert;
        self.popup = Popup::TasksTable(TasksTablePopup::AddTask);
    }

    pub fn create_preset(&mut self) {
        self.preset_name.clear();
        self.preset_tasks.clear();

        self.mode = InputMode::Insert;
        self.popup = Popup::TasksTable(TasksTablePopup::NewPreset);
        self.new_preset_focus = NewPresetFocus::Name;
    }

    pub fn edit_preset(&mut self) {
        if let Some(index) = self.preset_state.selected() {
            let preset = &self.presets[index];

            self.edit_preset = Some(index);

            self.preset_name.text = preset.name.clone();
            self.preset_name.cursor = self.preset_name.text.len();

            self.preset_tasks = preset.tasks.clone();

            self.popup = Popup::TasksTable(TasksTablePopup::NewPreset);
            self.mode = InputMode::Normal;
            self.new_preset_focus = NewPresetFocus::Name;
        }
    }

    pub fn edit_preset_task(&mut self) {
        if let Some(index) = self.preset_task_state.selected() {
            self.popup = Popup::TasksTable(TasksTablePopup::AddTask);

            self.task_destination = TaskDestination::EditPresetTask(index);

            let preset = &self.preset_tasks[index];

            self.task_name.text = preset.name.clone();
            self.planned_start.text = preset.planned_start.clone().unwrap_or_default();
            self.planned_end.text = preset.planned_end.clone().unwrap_or_default();

            self.task_name.cursor = self.task_name.text.len();
            self.planned_start.cursor = self.planned_start.text.len();
            self.planned_end.cursor = self.planned_end.text.len();

            self.mode = InputMode::Normal;
            self.tasks_selected_input = TaskSelectedInput::TaskName;
        }
    }

    pub fn add_known_task(&mut self) {
        self.known_task_name.clear();
        self.mode = InputMode::Insert;
        self.popup = Popup::TasksTable(TasksTablePopup::AddKnownTask);
    }

    pub fn edit_known_task(&mut self) {
        let selected = self.known_tasks_state.selected();
        
        if let Some(index) = selected {
            let suggestion = &self.known_tasks[index];

            self.known_task_name.text = suggestion.name.clone();
            self.mode = InputMode::Insert;
            self.popup = Popup::TasksTable(TasksTablePopup::EditKnownTask(index))
        }
    }

    //  ====================== Actions ===================
    pub fn save_task(&mut self) {
        match self.task_destination {

            TaskDestination::Preset => {
                let id = self.next_id;
                self.next_id += 1;

                let preset_task = TaskTemplate {
                    id,
                    name: self.task_name.text.clone(),
                    planned_start: Some(self.planned_start.text.clone()),
                    planned_end: Some(self.planned_end.text.clone()),
                };

                let position = match self.preset_task_state.selected() {
                    Some(index) => index + 1,
                    None => 0
                };
                
                self.preset_tasks.insert(position.min(self.preset_tasks.len()), preset_task);

                self.preset_task_state.select(Some(position.min(self.preset_tasks.len() - 1)));


                self.mode = InputMode::Normal;
                self.popup = Popup::TasksTable(TasksTablePopup::NewPreset);
            }

            TaskDestination::AddTask => {
                let task = TaskInfo {
                    name: self.task_name.text.clone(),
                    status: "PENDING".into(),
                    planned_start: self.planned_start.text.clone(),
                    planned_end: self.planned_end.text.clone(),
                    ..Default::default()
                };

                let position = match self.tasks_table_state.selected() {
                    Some(index) => index + 1,
                    None => 0
                };
                
                self.tasks.insert(position.min(self.tasks.len()), task);

                self.tasks_table_state.select(Some(position.min(self.tasks.len() - 1)));
            }

            TaskDestination::EditTask(index) => {
                if let Some(task) = self.tasks.get_mut(index) {
                    task.name = self.task_name.text.clone();
                    task.planned_start = self.planned_start.text.clone();
                    task.planned_end = self.planned_end.text.clone();
                    self.popup = Popup::None;
                }
                
            }

            TaskDestination::EditPresetTask(index) => {
                if let Some(task) = self.preset_tasks.get_mut(index) {
                    task.name = self.task_name.text.clone();
                    task.planned_start = Some(self.planned_start.text.clone());
                    task.planned_end = Some(self.planned_end.text.clone());
                    self.popup = Popup::TasksTable(TasksTablePopup::NewPreset);
                }
            }

        }

        self.task_name.clear();
        self.planned_start.clear();
        self.planned_end.clear();
        
        self.suggestions.clear();
        self.selected_suggestion = 0;
            
        current_tasks::save_current_tasks(&self.tasks).unwrap();
    }

    pub fn save_preset(&mut self) {
        if let Some(index) = self.edit_preset {
            self.presets[index].name = self.preset_name.text.clone();
            self.presets[index].tasks = std::mem::take(&mut self.preset_tasks);

            self.edit_preset = None;
        } else {
            let preset = Preset {
                id: self.next_id,
                name: self.preset_name.text.clone(),
                tasks: std::mem::take(&mut self.preset_tasks),
            };

            self.next_id += 1;
            self.presets.push(preset);
            self.mode = InputMode::Normal;
        }

        self.preset_name.clear();
        self.popup = Popup::TasksTable(TasksTablePopup::Presets);

        self.task_name.clear();
        self.planned_start.clear();
        self.planned_end.clear();

        if self.preset_state.selected().is_none() && !self.presets.is_empty() {
            self.preset_state.select(Some(0));
        }

        preset::save_preset(&self.presets).unwrap();
    }

    pub fn close_popup(&mut self) {
        match self.task_destination {
            TaskDestination::AddTask | TaskDestination::EditTask(_) => {
                self.popup = Popup::None;
            }

            TaskDestination::Preset | TaskDestination::EditPresetTask(_) => {
                self.popup = Popup::TasksTable(TasksTablePopup::NewPreset);
            }
        }
    }

    pub fn apply_preset(&mut self) {
        if let Some(index) = self.preset_state.selected() {
            // Set preset to main task table
            let preset = &self.presets[index];

            self.tasks = preset
                .tasks
                .iter()
                .map(|task| TaskInfo {
                    name: task.name.clone(),
                    status: "PENDING".to_string(),
                    planned_start: task.planned_start.clone().unwrap_or_default(),
                    planned_end: task.planned_end.clone().unwrap_or_default(),
                    ..Default::default()
                })
                .collect();

            self.popup = Popup::None;
        }
    }

    pub fn delete_preset(&mut self) {
        if let Some(index) = self.preset_state.selected() {
            self.presets.remove(index);
            
            if self.presets.is_empty() {
                self.preset_state.select(None);
            } else {
                let new_index = index.min(self.presets.len() - 1);
                self.preset_state.select(Some(new_index));
            }

            crate::storage::preset::save_preset(&self.presets).unwrap();
        }

        self.pending_command = None;
    }

    pub fn delete_known_task(&mut self) {
        let selected = self.known_tasks_state.selected();
        
        if let Some(index) = selected {
            self.known_tasks.remove(index);

            // Keep the selection valid
            if self.known_tasks.is_empty() {
                self.known_tasks_state.select(None);
            } else {
                let new_index = index.min(self.known_tasks.len() - 1);
                self.known_tasks_state.select(Some(new_index));
            }
        }
        crate::storage::known_tasks::save_known_tasks(&self.known_tasks).unwrap();

        self.pending_command = None;
    }

    // ======================= UTILS  =========================

    pub fn total_elapsed(&mut self) -> Duration {
        self.tasks
            .iter()
            .map(|task| task.stopwatch.elapsed())
            .sum()
    }

    pub fn priority_rank(priority: Priority) -> u8 {
        match priority {
            Priority::High => 0,
            Priority::Medium => 1,
            Priority::Low => 2,
        }
    }

    pub fn copy_inbox_input(&mut self) {
        let Some(index) = self.inbox_tasks_table_state.selected() else {
            return;
        };

        let text = self.inbox_items[index].input.clone();

        let Some(clipboard) = self.clipboard.as_mut() else {
            self.set_status_message(
                "Clipboard unavailable".to_string()
            );

            log::error!("Could not copy inbox item: clipboard unavailable");
            return;
        };

        match clipboard.set_text(text.clone()) {
            Ok(()) => {
                match clipboard.get_text() {
                    Ok(_) => {
                        self.set_status_message(
                            "Copied to clipboard".to_string()
                        );
                    }

                    Err(error) => {
                        log::error!(
                            "Clipboard write succeeded, but read-back failed: {}",
                            error
                        );

                        self.set_status_message(
                            "Copied, but clipboard could not be verified".to_string()
                        );
                    }
                }
            }

            Err(error) => {
                log::error!("Failed to copy inbox item: {}", error);

                self.set_status_message(
                    format!("Copy failed: {}", error)
                );
            }
        }
    }
}

