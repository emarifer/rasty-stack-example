use std::collections::VecDeque;
use yew::{function_component, html, Callback, Html, Properties};

use super::task_item::TaskItem;
use crate::model::Task;

#[derive(Properties, PartialEq)]
pub struct TaskListProps {
    pub tasks: VecDeque<Task>,
    pub deletetask: Callback<String>,
    pub toggletask: Callback<String>,
}

#[function_component(TaskList)]
pub fn task_list(
    TaskListProps {
        tasks,
        deletetask,
        toggletask,
    }: &TaskListProps,
) -> Html {
    let tasks = tasks
        .iter()
        .map(|task| html!(<TaskItem task={task.clone()} deletetask={deletetask} toggletask={toggletask} />))
        .collect::<Html>();

    html! {
        <ul>
            {tasks}
        </ul>
    }
}
