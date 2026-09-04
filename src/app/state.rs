use ratatui::widgets::{TableState, ListState};

use crate::{
    app::{
        Popup,
        TaskSelectedInput,
        TaskDestination,
        NewPresetFocus,
        InboxSelectedInput,
        Priority,
        Panel,
        AgendaSelectedInput
    },

    storage::{current_tasks, known_tasks, preset, inbox, agenda},
    vim_text::{InputState, InputMode},
    navigation::vim_navigation::NavigationMode,
    tasks_table::ui::TaskInfo,
    inbox::ui::InboxItemInfo,
    agenda::ui::{AgendaEvent, DateTimeInput, TIME_EDITABLE_POSITIONS, DATE_EDITABLE_POSITIONS},
    models::{TaskTemplate, Preset, KnownTask},
    navigation::move_items::MoveState,
};

use std::time::{Duration, Instant};
use chrono::{NaiveDate, NaiveTime, Local};

pub struct App {
    // Core
    pub running: bool,
    pub popup: Popup,
    pub pending_command: Option<char>,

    // Vim modes
    pub mode: InputMode,
    pub n_mode: NavigationMode,
    pub n_visual_start: Option<usize>,

    // Panels
    pub focused_panel: Panel,
    pub previous_panel: Panel,
    pub is_change: bool,

    // TasksTable
    pub tasks: Vec<TaskInfo>,
    pub tasks_table_state: TableState,

    pub task_name: InputState,
    pub planned_start: InputState,
    pub planned_end: InputState,
    pub tasks_selected_input: TaskSelectedInput,

    // Inbox
    pub inbox_items: Vec<InboxItemInfo>,
    pub inbox_tasks_table_state: TableState,

    pub inbox_item: InputState,
    pub inbox_selected_input: InboxSelectedInput,
    pub priority: Priority,

    // Agenda
    pub event: InputState,
    pub events: Vec<AgendaEvent>,
    pub agenda_tasks_table_state: TableState,
    pub agenda_selected_input: AgendaSelectedInput,
    pub date_input: DateTimeInput,
    pub time_input: DateTimeInput,
    pub event_repeat: bool,

    // Presets
    pub presets: Vec<Preset>,
    pub preset_state: ListState,
    pub preset_tasks: Vec<TaskTemplate>,
    pub preset_task_state: ListState,

    pub preset_name: InputState,
    pub edit_preset: Option<usize>,
    pub new_preset_focus: NewPresetFocus,
    pub task_destination: TaskDestination,

    // Known tasks
    pub known_tasks: Vec<KnownTask>,
    pub known_tasks_state: ListState,
    pub known_task_name: InputState,
    pub suggestions: Vec<String>,
    pub selected_suggestion: usize,

    // Misc
    pub last_agenda_update: NaiveDate,
    pub move_state: MoveState,
    pub help_scroll: u16,
    pub next_id: u64,

    // Clipboard / notifications
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
        let mut tasks_table_state = TableState::default();
        tasks_table_state.select(Some(0));

        let mut preset_task_state = ListState::default();
        preset_task_state.select(Some(0));

        let mut preset_state = ListState::default();
        preset_state.select(Some(0));

        let mut known_tasks_state = ListState::default();
        known_tasks_state.select(Some(0));
        
        let mut inbox_tasks_table_state = TableState::default();
        inbox_tasks_table_state.select(Some(0));

        let mut agenda_tasks_table_state = TableState::default();
        agenda_tasks_table_state.select(Some(0));

        let year = Local::now().format("%y").to_string();

        let date_input = DateTimeInput {
            value: format!("00-00-{}", Local::now().format("%y")),
            cursor: 0,
            editable_positions: &DATE_EDITABLE_POSITIONS,
        };

        let time_input = DateTimeInput {
            value: "00:00".to_string(),
            cursor: 0,
            editable_positions: TIME_EDITABLE_POSITIONS,
        };

        Self {
            // Core
            pending_command: None,
            popup: Popup::None,
            running: true,

            // TasksTable
            task_name: InputState::default(),
            planned_start: InputState::default(),
            planned_end: InputState::default(),
            tasks_selected_input: TaskSelectedInput::TaskName,

            tasks: current_tasks::load_current_tasks(),
            tasks_table_state,

            // Inbox
            inbox_item: InputState::default(),
            inbox_items: inbox::load_inbox(),
            inbox_tasks_table_state,
            inbox_selected_input: InboxSelectedInput::InboxItemInput,
            priority: Priority::Low,

            // Agenda
            event: InputState::default(),
            events: agenda::load_agenda(),
            agenda_tasks_table_state,
            agenda_selected_input: AgendaSelectedInput::Name,
            date_input,
            time_input,
            event_repeat: false,
            
            // Navigation
            n_mode: NavigationMode::Normal,
            n_visual_start: Some(0),
            mode: InputMode::Insert,

            // Presets
            presets: preset::load_presets(),
            preset_tasks: Vec::new(),
            preset_name: InputState::default(),
            edit_preset: None,
            task_destination: TaskDestination::AddTask,
            new_preset_focus: NewPresetFocus::Name,
            preset_task_state,
            preset_state,

            // Known tasks
            known_task_name: InputState::default(),
            known_tasks_state,
            suggestions: Vec::new(),
            selected_suggestion: 0,
            known_tasks: known_tasks::load_known_tasks(),

            // Panel
            focused_panel: Panel::TasksTable,
            previous_panel: Panel::TasksTable,
            is_change: true,

            // Misc
            last_agenda_update: Local::now().date_naive(),
            move_state: MoveState::default(),
            help_scroll: 0,
            next_id: 1,

            // Clipboard / status
            clipboard: arboard::Clipboard::new().ok(),
            status_message: None,
            status_message_until: None,
        }
    }
}
