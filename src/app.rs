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
}
