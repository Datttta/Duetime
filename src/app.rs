use ratatui::widgets::{TableState, ListState};
use serde::{Serialize, Deserialize};

use crate::{
    vim_text::{InputState, InputMode},
    vim_navigation::NavigationMode,
    tasks::TaskInfo,
    inbox::InboxItemInfo,
    models::{TaskTemplate, Preset, KnownTask},
    move_items::MoveState,
    storage_current_tasks,
    storage_known_tasks,
    storage_preset,
    storage_inbox,
};

use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Priority {
    #[default]
    Low,
    Medium,
    High,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Panel {
    Tasks,
    Inbox,
}

#[derive(PartialEq, Debug)]
pub enum Popup {
    None,
    Tasks(TasksPopup),
    Inbox(InboxPopup),
}

#[derive(PartialEq, Debug)]
pub enum InboxPopup {
    AddInboxItem,
    EditInboxItem,
    InfoInboxItem,
}

#[derive(PartialEq, Debug)]
pub enum TasksPopup {
    AddTask,
    Presets,
    EditTask,
    NewPreset,
    KnownTasks,
    AddKnownTask,
    EditKnownTask(usize),
    TaskInfo,
    Help,
}

#[derive(PartialEq)]
pub enum NewPresetFocus {
    Name,
    Tasks,
}

pub enum TaskDestination {
    AddTask,
    Preset,
    EditTask(usize),
    EditPresetTask(usize),
}

#[derive(PartialEq)]
pub enum SelectedInput {
    TaskName,
    PlannedStart,
    PlannedEnd,
}

#[derive(PartialEq)]
pub enum InboxSelectedFeature {
    InboxItemInput,
    Priority,
}

pub struct App {
    pub popup: Popup,
    pub pending_command: Option<char>,
    pub running: bool,

    pub task_name: InputState,
    pub planned_start: InputState,
    pub planned_end: InputState,

    pub n_mode: NavigationMode,
    pub n_visual_start: Option<usize>,

    pub mode: InputMode,
    pub selected_input: SelectedInput,

    pub tasks: Vec<TaskInfo>,

    pub table_state: TableState,
    pub preset_task_state: ListState,

    pub presets: Vec<Preset>,
    pub edit_preset: Option<usize>,
    pub preset_name: InputState,
    pub preset_tasks: Vec<TaskTemplate>,
    pub task_destination: TaskDestination,
    pub next_id: u64,
    pub new_preset_focus: NewPresetFocus,
    pub preset_state: ListState,

    pub known_tasks: Vec<KnownTask>,
    pub known_task_name: InputState,
    pub known_tasks_state: ListState,
    pub suggestions: Vec<String>,
    pub selected_suggestion: usize,

    pub move_state: MoveState,

    pub help_scroll: u16,

    pub focused_panel: Panel,
    pub previous_panel: Panel,

    pub inbox_item: InputState,
    pub inbox_items: Vec<InboxItemInfo>,
    pub inbox_table_state: TableState,
    pub inbox_selected_feature: InboxSelectedFeature,
    pub priority: Priority,

    pub is_change: bool,

    pub clipboard: Option<arboard::Clipboard>,
    pub status_message: Option<String>,
    pub status_message_until: Option<Instant>,
}

impl App {
    pub fn set_status_message(&mut self, message: String) {
        self.status_message = Some(message);
        self.status_message_until = Some(
            Instant::now() + Duration::from_secs(1)
        );
    }

    pub fn new() -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));

        let mut preset_task_state = ListState::default();
        preset_task_state.select(Some(0));

        let mut preset_state = ListState::default();
        preset_state.select(Some(0));

        let mut known_tasks_state = ListState::default();
        known_tasks_state.select(Some(0));
        
        let mut inbox_table_state = TableState::default();
        inbox_table_state.select(Some(0));

        Self {
            pending_command: None,
            popup: Popup::None,
            running: true,

            task_name: InputState::default(),
            planned_start: InputState::default(),
            planned_end: InputState::default(),

            n_visual_start: Some(0),
            n_mode: NavigationMode::Normal,
            
            mode: InputMode::Insert,
            selected_input: SelectedInput::TaskName,

            table_state,
            preset_task_state,
            
            presets: storage_preset::load_presets(),
            preset_tasks: Vec::new(),
            preset_name: InputState::default(),
            edit_preset: None,
            task_destination: TaskDestination::AddTask,
            next_id: 1,
            new_preset_focus: NewPresetFocus::Name,
            preset_state,

            known_task_name: InputState::default(),
            known_tasks_state,
            suggestions: Vec::new(),
            selected_suggestion: 0,
            known_tasks: storage_known_tasks::load_known_tasks(),
            
            tasks: storage_current_tasks::load_current_tasks(),

            move_state: MoveState::default(),

            help_scroll: 0,

            focused_panel: Panel::Tasks,
            previous_panel: Panel::Tasks,

            inbox_item: InputState::default(),
            inbox_items: storage_inbox::load_inbox(),
            inbox_table_state,
            inbox_selected_feature: InboxSelectedFeature::InboxItemInput,
            priority: Priority::Low,

            is_change: true,

            clipboard: arboard::Clipboard::new().ok(),
            status_message: None,
            status_message_until: None,
        }
    }

    // =================== POPUPS =======================

    pub fn task_add (&mut self, destination: TaskDestination) {
        self.task_destination = destination;

        self.task_name.clear();
        self.planned_start.clear();
        self.planned_end.clear();

        self.selected_input = SelectedInput::TaskName;
        self.mode = InputMode::Insert;
        self.popup = Popup::Tasks(TasksPopup::AddTask);
    }

    pub fn create_preset(&mut self) {
        self.preset_name.clear();
        self.preset_tasks.clear();

        self.mode = InputMode::Insert;
        self.popup = Popup::Tasks(TasksPopup::NewPreset);
        self.new_preset_focus = NewPresetFocus::Name;
    }

    pub fn edit_preset(&mut self) {
        if let Some(index) = self.preset_state.selected() {
            let preset = &self.presets[index];

            self.edit_preset = Some(index);

            self.preset_name.text = preset.name.clone();
            self.preset_name.cursor = self.preset_name.text.len();

            self.preset_tasks = preset.tasks.clone();

            self.popup = Popup::Tasks(TasksPopup::NewPreset);
            self.mode = InputMode::Normal;
            self.new_preset_focus = NewPresetFocus::Name;
        }
    }

    pub fn edit_preset_task(&mut self) {
        if let Some(index) = self.preset_task_state.selected() {
            self.popup = Popup::Tasks(TasksPopup::AddTask);

            self.task_destination = TaskDestination::EditPresetTask(index);

            let preset = &self.preset_tasks[index];

            self.task_name.text = preset.name.clone();
            self.planned_start.text = preset.planned_start.clone().unwrap_or_default();
            self.planned_end.text = preset.planned_end.clone().unwrap_or_default();

            self.task_name.cursor = self.task_name.text.len();
            self.planned_start.cursor = self.planned_start.text.len();
            self.planned_end.cursor = self.planned_end.text.len();

            self.mode = InputMode::Normal;
            self.selected_input = SelectedInput::TaskName;
        }
    }

    pub fn add_known_task(&mut self) {
        self.known_task_name.clear();
        self.mode = InputMode::Insert;
        self.popup = Popup::Tasks(TasksPopup::AddKnownTask);
    }

    pub fn edit_known_task(&mut self) {
        let selected = self.known_tasks_state.selected();
        
        if let Some(index) = selected {
            let suggestion = &self.known_tasks[index];

            self.known_task_name.text = suggestion.name.clone();
            self.mode = InputMode::Insert;
            self.popup = Popup::Tasks(TasksPopup::EditKnownTask(index))
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
                self.popup = Popup::Tasks(TasksPopup::NewPreset);
            }

            TaskDestination::AddTask => {
                let task = TaskInfo {
                    name: self.task_name.text.clone(),
                    status: "PENDING".into(),
                    planned_start: self.planned_start.text.clone(),
                    planned_end: self.planned_end.text.clone(),
                    ..Default::default()
                };

                let position = match self.table_state.selected() {
                    Some(index) => index + 1,
                    None => 0
                };
                
                self.tasks.insert(position.min(self.tasks.len()), task);

                self.table_state.select(Some(position.min(self.tasks.len() - 1)));
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
                    self.popup = Popup::Tasks(TasksPopup::NewPreset);
                }
            }

        }

        self.task_name.clear();
        self.planned_start.clear();
        self.planned_end.clear();
        
        self.suggestions.clear();
        self.selected_suggestion = 0;
            
        storage_current_tasks::save_current_tasks(&self.tasks).unwrap();
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
        self.popup = Popup::Tasks(TasksPopup::Presets);

        self.task_name.clear();
        self.planned_start.clear();
        self.planned_end.clear();

        if self.preset_state.selected().is_none() && !self.presets.is_empty() {
            self.preset_state.select(Some(0));
        }

        storage_preset::save_preset(&self.presets).unwrap();
    }

    pub fn close_popup(&mut self) {
        match self.task_destination {
            TaskDestination::AddTask | TaskDestination::EditTask(_) => {
                self.popup = Popup::None;
            }

            TaskDestination::Preset | TaskDestination::EditPresetTask(_) => {
                self.popup = Popup::Tasks(TasksPopup::NewPreset);
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

            crate::storage_preset::save_preset(&self.presets).unwrap();
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
        crate::storage_known_tasks::save_known_tasks(&self.known_tasks).unwrap();

        self.pending_command = None;
    }

    // ======================= UTILS  =========================

    pub fn total_elapsed(&self) -> Duration {
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
}

