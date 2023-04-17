use std::collections::VecDeque;
use std::rc::Rc;
use yew::Reducible;

use crate::model::Task;

/// reducer's Action
pub enum TaskAction {
    Set(VecDeque<Task>),
    Add(Task),
    Toggle(String),
    Delete(String),
}

/// reducer's State
pub struct TaskState {
    pub tasks: VecDeque<Task>,
}

/// Implementation by default when starting the application
impl Default for TaskState {
    fn default() -> Self {
        Self {
            tasks: VecDeque::from([]),
        }
    }
}

/// Implementation of Reducible (required for the reducer)
impl Reducible for TaskState {
    /// Reducer Action Type
    type Action = TaskAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_tasks = match action {
            TaskAction::Set(tasks) => tasks,
            TaskAction::Add(task) => {
                let mut tasks = self.tasks.clone();
                tasks.push_front(task);
                tasks
            }
            TaskAction::Toggle(id) => {
                let mut tasks = self.tasks.clone();
                let task = tasks.iter_mut().find(|task| task.id == id);
                if let Some(t) = task {
                    t.completed = !t.completed;
                }
                tasks
            }
            TaskAction::Delete(id) => {
                let mut tasks = self.tasks.clone();
                tasks.retain(|task| task.id != id);
                tasks
            }
        };

        Self { tasks: next_tasks }.into()
    }
}
