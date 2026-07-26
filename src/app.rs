use crate::vim_text::{InputState, InputMode};
use crate::tasks::TaskInfo;
use crate::ui::popup::presets::{TaskTemplate, Preset};

use ratatui::widgets::TableState;

pub enum Popup {
    None,
    AddTask,
    Presets,
    EditTask(usize),
    Presets,
    NewPreset,
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

    pub templates: Vec<TaskTemplate>,
    pub presets: Vec<Preset>,

    pub mode: InputMode,
    pub selected_input: SelectedInput,

    pub tasks: Vec<TaskInfo>,

    pub table_state: TableState,

    pub presets: Vec<Preset>,
    pub edit_preset: Option<usize>,
    pub preset_name: InputState,
    pub preset_tasks: Vec<TaskTemplate>,
    pub selected_preset_task: usize,
    pub task_destination: TaskDestination,
}

impl App {
    pub fn new() -> Self {
        let table_state = TableState::default();

        Self {
            pending_command: None,

            popup: Popup::None,
            running: true,

            mode: InputMode::Insert,

            task_name: InputState::default(),
            planned_start: InputState::default(),
            planned_end: InputState::default(),
            
            templates: Vec::new(),
            presets: Vec::new(),

            selected_input: SelectedInput::TaskName,
            tasks: Vec::new(),

            table_state,

            presets: Vec::new(),
            preset_tasks: Vec::new(),
            preset_name: InputState::default(),
            edit_preset: None,
            selected_preset_task: 0,
            task_destination: TaskDestination::Tasks,
        }
    }
}
