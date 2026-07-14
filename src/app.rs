pub enum Popup {
    None,
    AddTask,
}

pub struct App {
    pub popup: Popup,
    pub waiting_for_t: bool,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            popup: Popup::None,
            waiting_for_t: false,
            running: true,
        }
    }
}
