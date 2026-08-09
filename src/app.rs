use ratatui::widgets::{TableState, ListState};

use crate::vim_text::{InputState, InputMode};
use crate::vim_navigation::NavigationMode;
use crate::tasks::TaskInfo;
use crate::models::{TaskTemplate, Preset, KnownTask};
use crate::storage_current_tasks;
use crate::storage_preset;
use crate::storage_known_tasks;

#[derive(PartialEq)]
pub enum NewPresetFocus {
    Name,
    Tasks,
}

#[derive(PartialEq, Debug)]
pub enum Popup {
    None,
    AddTask,
    Presets,
    EditTask,
    NewPreset,
    KnownTasks,
    AddKnownTask,
    EditKnownTask(usize),
    TaskInfo,
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
}

impl App {
    pub fn new() -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));

        let mut preset_task_state = ListState::default();
        preset_task_state.select(Some(0));

        let mut preset_state = ListState::default();
        preset_state.select(Some(0));

        let mut known_tasks_state = ListState::default();
        known_tasks_state.select(Some(0));

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
        }
    }

    pub fn add_task_popup (&mut self, destination: TaskDestination) {
        self.task_destination = destination;

        self.task_name.clear();
        self.planned_start.clear();
        self.planned_end.clear();

        self.selected_input = SelectedInput::TaskName;
        self.mode = InputMode::Insert;
        self.popup = Popup::AddTask;
    }

    pub fn edit_task_popup(&mut self) {
        if let Some(index) = self.table_state.selected() {
            let task = &self.tasks[index];

            self.task_destination = TaskDestination::EditTask(index);

            // Load task data into inputs
            self.task_name.text = task.name.clone();
            self.planned_start.text = task.planned_start.clone();
            self.planned_end.text = task.planned_end.clone();

            self.task_name.cursor = self.task_name.text.len();
            self.planned_start.cursor = self.planned_start.text.len();
            self.planned_end.cursor = self.planned_end.text.len();

            self.mode = InputMode::Normal;
            self.selected_input = SelectedInput::TaskName;
            self.popup = Popup::EditTask;
        }
    }

    pub fn edit_preset_task_popup(&mut self) {
        if let Some(index) = self.preset_task_state.selected() {
            self.popup = Popup::AddTask;

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
}
