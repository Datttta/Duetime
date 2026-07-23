use crate::vim_text::{InputState, InputMode};
use crate::tasks::TaskInfo;

use ratatui::widgets::TableState;

pub enum Popup {
    None,
    AddTask,
    EditTask(usize),
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
    pub waiting_for_d: bool,
    pub running: bool,

    pub task_name: InputState,
    pub planned_start: InputState,
    pub planned_end: InputState,

    pub mode: InputMode,
    pub selected_input: SelectedInput,

    pub tasks: Vec<TaskInfo>,

    pub table_state: TableState,
}

impl App {
    pub fn new() -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));

        Self {
            popup: Popup::None,
            waiting_for_t: false,
            waiting_for_d: false,
            running: true,

            mode: InputMode::Insert,

            task_name: InputState::default(),
            planned_start: InputState::default(),
            planned_end: InputState::default(),

            selected_input: SelectedInput::TaskName,
            tasks: Vec::new(),

            table_state,
        }
    }
}
