use crate::vim::{InputState, InputMode};
use crate::tasks::TaskInfo;

pub enum Popup {
    None,
    AddTask,
}

#[derive(PartialEq)]
pub enum SelectedInput {
    TaskName,
    PlannedStart,
    PlannedEnd,
}

pub struct App {
    pub popup: Popup,
    pub waiting_for_t: bool,
    pub running: bool,

    pub task_name: InputState,
    pub planned_start: InputState,
    pub planned_end: InputState,

    pub mode: InputMode,

    pub selected_input: SelectedInput,

    pub tasks: Vec<TaskInfo>,
    
    pub selected_task: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            popup: Popup::None,
            waiting_for_t: false,
            running: true,

            mode: InputMode::Insert,

            task_name: InputState::default(),
            planned_start: InputState::default(),
            planned_end: InputState::default(),

            selected_input: SelectedInput::TaskName,
            tasks: Vec::new(),

            selected_task: 0,
        }
    }
}
