import sys
import time
from textual.app import App, ComposeResult
from textual.widgets import Label, Static, Input, Button
from textual.containers import Horizontal, Vertical
from textual.screen import ModalScreen

class NewTaskPopup(ModalScreen[dict]):
    def __init__(self, task_data: dict | None = None, **kwargs):
        super().__init__(**kwargs)
        # Store the passed data (if any)
        self.task_data = task_data

    def compose(self) -> ComposeResult:
        with Vertical(id="dialog"):
            title = "Edit Task" if self.task_data else "Create New Task"
            yield Label(title, id="dialog-title")
            
            name_input = Input(placeholder="Task Name", id="task-name")
            start_input = Input(placeholder="planned-start (e.g. 14:00)", id="new-task-planned-start")
            end_input = Input(placeholder="planned-end (e.g. 15:00)", id="new-task-planned-end")
            
            if self.task_data:
                name_input.value = self.task_data.get("name", "")
                
                # We strip out "--:--" placeholders so the user doesn't have to delete them
                p_start = self.task_data.get("new-task-planned-start", "")
                start_input.value = "" if p_start == "--:--" else p_start
                
                p_end = self.task_data.get("new-task-planned-end", "")
                end_input.value = "" if p_end == "--:--" else p_end

            yield name_input
            yield start_input
            yield end_input

            yield Horizontal(
                Button("Save", id="btn-save"),
                Button("Cancel", id="btn-cancel"),
                id="dialog-buttons"  # Add this
            )

    def on_mount(self) -> None:
        """Remove text selection when the popup mounts."""
        # Get the name input and clear any selection
        name_input = self.query_one("#task-name", Input)
        # Move cursor to the end of the text without selecting
        name_input.cursor_position = len(name_input.value)

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
    
    # State flags
    _waiting_for_t = False
    _waiting_for_d = False
    selected_index = 0 # Tracks which task is highlighted

    def compose(self):
        with Vertical(id="task-list", classes="info-box"):
            with Horizontal():
                yield Static("TO-DO", id="TODO-label")
                yield Static("Status")
                yield Static("Planned start")
                yield Static("Planned end")
                yield Static("Actual start")
                yield Static("Actual end")
                yield Static("Elapsed")

            # I've given the first task an ID so we can identify it's not a header easily if needed, 
            # though using classes="tasks" is exactly the right approach.
            with Horizontal(classes="tasks"):
                yield Static("My first task", classes="task-name") 
                yield Static("PENDING", classes="status") 
                yield Static("14:00", classes="planned-start") 
                yield Static("16:00", classes="planned-end") 
                yield Static("14:01", classes="actual-start") 
                yield Static("16:05", classes="actual-end") 
                yield Static("02:04:00", classes="elapsed") 

    def on_mount(self):
        """Runs once when the app starts. Good place to initialize the cursor."""
        self.update_cursor()

    def update_cursor(self):
        """Visually updates the tasks by adding/removing the 'selected' CSS class."""
        # Get a list of all horizontal rows that represent tasks
        tasks = list(self.query("Horizontal.tasks"))
        
        if not tasks:
            return

        # Ensure index stays within bounds (e.g., if you delete the last item)
        self.selected_index = max(0, min(self.selected_index, len(tasks) - 1))

        # Loop through tasks and apply the .selected class to the active one
        for i, task in enumerate(tasks):
            if i == self.selected_index:
                task.add_class("selected")
            else:
                task.remove_class("selected")

    def delete_current_task(self):
        """Handles removing the selected task from the UI."""
        tasks = list(self.query("Horizontal.tasks"))
        if not tasks:
            return
        
        # Remove the widget from the DOM
        tasks[self.selected_index].remove()
        
        # Adjust the index so the cursor doesn't disappear if we deleted the bottom item
        if self.selected_index >= len(tasks) - 1:
            self.selected_index = max(0, len(tasks) - 2)
            
        # Re-apply the visual highlight to the new active item
        # We use call_after_refresh to ensure the DOM has updated before querying again
        self.call_after_refresh(self.update_cursor)

    def handle_new_task(self, task_data: dict | None):
        if task_data is not None:
            if task_data["new-task-planned-start"] == "":
                task_data["new-task-planned-start"] = "--:--"
            if task_data["new-task-planned-end"] == "":
                task_data["new-task-planned-end"] = "--:--"

            new_task_row = Horizontal(
                Static(task_data["name"], classes="task-name"),
                Static("PENDING", classes="status"),
                Static(task_data["new-task-planned-start"], classes="planned-start"),
                Static(task_data["new-task-planned-end"], classes="planned-end"),
                Static("--:--", classes="actual-start"),  
                Static("--:--", classes="actual-end"),    
                Static("--:--:--", classes="elapsed"),    
                classes="tasks"
            )
            
            task_list = self.query_one("#task-list", Vertical)
            task_list.mount(new_task_row)

            # Important: Update the cursor to recognize the newly added row
            self.call_after_refresh(self.update_cursor)

            self.notify(
                f"{task_data['name']}: "
                f"{task_data['new-task-planned-start']} - "
                f"{task_data['new-task-planned-end']}"
            )

    def handle_edit_task(self, task_data: dict | None):
        """Callback for when the popup closes after editing."""
        if task_data is not None:
            tasks = list(self.query("Horizontal.tasks"))
            if not tasks:
                return
            
            # Re-format empty strings back into placeholders
            if task_data["new-task-planned-start"] == "":
                task_data["new-task-planned-start"] = "--:--"
            if task_data["new-task-planned-end"] == "":
                task_data["new-task-planned-end"] = "--:--"
            
            # Select the row we were highlighting
            current_task = tasks[self.selected_index]
            
            # Update the text of the specific children widgets using their classes
            current_task.query_one(".task-name", Static).update(task_data["name"])
            current_task.query_one(".planned-start", Static).update(task_data["new-task-planned-start"])
            current_task.query_one(".planned-end", Static).update(task_data["new-task-planned-end"])
            
            self.notify(f"Updated: {task_data['name']}")


    def on_key(self, event):
        # Handle quitting
        if event.key == "q":
            self.exit()

        # Handle 'at' for Add Task
        if event.key == "a":
            self._waiting_for_t = True
            self._waiting_for_d = False # Cancel any pending 'd'
        elif event.key == "t" and self._waiting_for_t:
            self._waiting_for_t = False
            self.push_screen(NewTaskPopup(), self.handle_new_task)
            
        # Handle 'dd' for Delete Task
        elif event.key == "d":
            self._waiting_for_t = False # Cancel any pending 't'
            if self._waiting_for_d:
                self.delete_current_task()
                self._waiting_for_d = False # Reset state
            else:
                self._waiting_for_d = True

        # Handle 'e' for Edit Task (NEW)
        elif event.key == "e":
            self._waiting_for_t = self._waiting_for_d = False
            tasks = list(self.query("Horizontal.tasks"))
            
            if tasks:
                current_task = tasks[self.selected_index]
                
                # Extract text using .render() instead of .renderable
                existing_data = {
                    "name": str(current_task.query_one(".task-name", Static).render()),
                    "new-task-planned-start": str(current_task.query_one(".planned-start", Static).render()),
                    "new-task-planned-end": str(current_task.query_one(".planned-end", Static).render()),
                }
                
                # Call popup WITH data (Edit mode) and point it to a new callback
                self.push_screen(NewTaskPopup(existing_data), self.handle_edit_task)
                
        # Handle Cursor Movement (j/down, k/up)
        elif event.key in ("j", "down"):
            self._waiting_for_t = self._waiting_for_d = False
            self.selected_index += 1
            self.update_cursor()
            
        elif event.key in ("k", "up"):
            self._waiting_for_t = self._waiting_for_d = False
            self.selected_index -= 1
            self.update_cursor()
            
        # Reset states if any other key is pressed
        else:
            self._waiting_for_t = False
            self._waiting_for_d = False

if __name__ == "__main__":
    Duetime().run()
