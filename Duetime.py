import sys
import time
from textual.app import App, ComposeResult
from textual.widgets import Label, Static, Input, Button
from textual.containers import Horizontal, Vertical # <-- Add Vertical here
from textual.screen import ModalScreen

class NewTaskPopup(ModalScreen[dict]):

    def compose(self) -> ComposeResult:
        with Vertical(id="dialog"):
            yield Label("Create New Task", id="dialog-title")
            yield Input(placeholder="Task Name", id="task-name")
            yield Input(
                placeholder="planned-start (e.g. 14:00)",
                id="new-task-planned-start",
            )
            yield Input(
                placeholder="planned-end (e.g. 15:00)",
                id="new-task-planned-end",
            )

            with Horizontal(id="dialog-buttons"):
                yield Button("Save", id="btn-save")
                yield Button("Cancel", id="btn-cancel")

    def save_task(self) -> None:
        task_name = self.query_one("#task-name", Input).value
        task_planned_start = self.query_one(
            "#new-task-planned-start", Input
        ).value
        task_planned_end = self.query_one(
            "#new-task-planned-end", Input
        ).value

        self.dismiss({
            "name": task_name,
            "new-task-planned-start": task_planned_start,
            "new-task-planned-end": task_planned_end,
        })

    def on_key(self, event):
        if event.key == "escape":
            self.dismiss(None)

    def on_input_submitted(self, event: Input.Submitted) -> None:
        """This triggers ONLY when Enter is pressed while typing inside an Input box."""
        self.save_task()

    def on_button_pressed(self, event: Button.Pressed) -> None:
        if event.button.id == "btn-save":
            self.save_task()
        else:
            self.dismiss(None)

class Duetime(App):
    CSS_PATH = "Duetime.tcss" 
    ansi_color = True
    _waiting_for_t = False

    def compose(self):
        # 1. The Vertical container draws ONE big rectangle around everything inside it
        with Vertical(id="task-list", classes="info-box"):
            
            # 2. The Header Row (Horizontal)
            with Horizontal():
                yield Static("TO-DO", id="TODO-label")
                yield Static("Status")
                yield Static("Planned start")
                yield Static("Planned end")
                yield Static("Actual start")
                yield Static("Actual end")
                yield Static("Elapsed")

            # 3. The Task Row (Horizontal, inside the same Vertical box)
            with Horizontal(classes="tasks"):
                yield Static("My first task", classes="task-name") 
                yield Static("PENDING", classes="status") 
                yield Static("14:00", classes="planned-start") 
                yield Static("16:00", classes="planned-end") 
                yield Static("14:01", classes="actual-start") 
                yield Static("16:05", classes="actual-end") 
                yield Static("02:04:00", classes="elapsed") 


    def on_key(self, event):
        if event.key == "q":
            self.exit()

        if event.key == "a":
            self._waiting_for_t = True
        elif event.key == "t" and self._waiting_for_t:
            self._waiting_for_t = False
            self.push_screen(NewTaskPopup(), self.handle_new_task)
        else:
            self._waiting_for_t = False

    def handle_new_task(self, task_data: dict | None):
        if task_data is not None:
            # 1. Build the new horizontal row with the provided data
            if task_data["new-task-planned-start"] == "":
                task_data["new-task-planned-start"] = "--:--"

            if task_data["new-task-planned-end"] == "":
                task_data["new-task-planned-end"] = "--:--"

            new_task_row = Horizontal(
                Static(task_data["name"], classes="task-name"),
                Static("PENDING", classes="status"),
                Static(task_data["new-task-planned-start"], classes="planned-start"),
                Static(task_data["new-task-planned-end"], classes="planned-end"),
                Static("--:--", classes="actual-start"),   # Placeholder
                Static("--:--", classes="actual-end"),     # Placeholder
                Static("--:--:--", classes="elapsed"),        # Placeholder
                classes="tasks"
            )
            
            # 2. Find the main Vertical container and add the new row to it
            task_list = self.query_one("#task-list", Vertical)
            task_list.mount(new_task_row)

            self.notify(
                f"{task_data['name']}: "
                f"{task_data['new-task-planned-start']} - "
                f"{task_data['new-task-planned-end']}"
            )

if __name__ == "__main__":
    Duetime().run()
