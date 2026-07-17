use crate::text_input::InputState;

pub enum Popup {
    None,
    AddTask,
}

pub struct App {
    pub popup: Popup,
    pub waiting_for_t: bool,
    pub running: bool,

    pub task_name: InputState,
    pub description: InputState,
}

impl App {
    pub fn new() -> Self {
        Self {
            popup: Popup::None,
            waiting_for_t: false,
            running: true,

            task_name: InputState::default(),
            description: InputState::default(),
        }
    }
}
