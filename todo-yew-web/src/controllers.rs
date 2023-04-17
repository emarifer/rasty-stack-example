use gloo_dialogs::alert;
use wasm_bindgen_futures::spawn_local;
use yew::UseReducerHandle;

use crate::{state::*, todo_api::*};

pub struct TaskController {
    state: UseReducerHandle<TaskState>,
}

impl TaskController {
    pub fn new(state: UseReducerHandle<TaskState>) -> TaskController {
        TaskController { state }
    }

    pub fn init_tasks(&self) {
        let tasks = self.state.clone();

        spawn_local(async move {
            let fetched_tasks = fetch_tasks().await;

            match fetched_tasks {
                Ok(ft) => tasks.dispatch(TaskAction::Set(ft)),
                Err(e) => alert(&e.to_string()),
            }
        });
    }

    pub fn create_task(&self, title: String) {
        let tasks = self.state.clone();

        spawn_local(async move {
            let response = create_task(&title).await;

            match response {
                Ok(task) => tasks.dispatch(TaskAction::Add(task)),
                Err(e) => alert(&e.to_string()),
            }
        });
    }

    pub fn toggle_task(&self, id: String) {
        let tasks = self.state.clone();

        spawn_local(async move {
            let response = toggle_task(id.clone()).await;

            match response {
                Ok(af) if af.rows_affected == 1 => tasks.dispatch(TaskAction::Toggle(id.clone())),
                Ok(_) => alert("Did not get a response"),
                Err(e) => alert(&e.to_string()),
            }
        });
    }

    pub fn delete_task(&self, id: String) {
        let tasks = self.state.clone();

        spawn_local(async move {
            let response = delete_task(id.clone()).await;

            match response {
                Ok(af) if af.rows_affected == 1 => tasks.dispatch(TaskAction::Delete(id.clone())),
                Ok(_) => alert("Did not get a response"),
                Err(e) => alert(&e.to_string()),
            }
        });
    }
}
