use crate::text_input::InputState;

pub enum Popup {
    None,
    AddTask,
}

pub enum SelectedInput {
    TaskName,
    PlannedStart,
}

pub struct App {
    pub popup: Popup,
    pub waiting_for_t: bool,
    pub running: bool,

    pub task_name: InputState,
    pub planned_start: InputState,

    pub selected_input: SelectedInput,
}

impl App {
    pub fn new() -> Self {
        Self {
            popup: Popup::None,
            waiting_for_t: false,
            running: true,

            task_name: InputState::default(),
            planned_start: InputState::default(),

            selected_input: SelectedInput::TaskName,
        }
    }
}
