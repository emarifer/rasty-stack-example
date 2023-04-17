use chrono::Timelike;
use yew::{classes, function_component, html, Callback, Event, Html, MouseEvent, Properties};

use crate::model::Task;

#[derive(Properties, PartialEq)]
pub struct TaskItemProps {
    pub task: Task,
    pub deletetask: Callback<String>,
    pub toggletask: Callback<String>,
}

#[function_component(TaskItem)]
pub fn task_item(
    TaskItemProps {
        task,
        deletetask,
        toggletask,
    }: &TaskItemProps,
) -> Html {
    let date = task.created_at.date_naive().format("%d-%m-%Y");

    let time_and_date = &format!(
        "{:02}:{:02} • {}",
        task.created_at.hour(),
        task.created_at.minute(),
        date
    );

    let label_style = "w-full p-3 ml-2 text-lg truncate";

    let completed_task = match task.completed {
        true => Some("text-gray-600 line-through"),
        false => None,
    };

    let handle_click = {
        let task = task.clone();
        let on_delete_task = deletetask.clone();

        move |_e: MouseEvent| on_delete_task.emit(task.id.clone())
    };

    let handle_toggle = {
        let task = task.clone();
        let on_toggle_task = toggletask.clone();

        move |_e: Event| on_toggle_task.emit(task.id.clone())
    };

    html! {
        <li>
            <div class="flex flex-col my-2 pl-4 py-1 border rounded border-gray-700 hover:-translate-y-1.5 ease-in duration-300">
                <div class="flex items-center">
                    <input id={task.clone().id}
                        type="checkbox"
                        checked={task.completed}
                        class="w-5 h-5 accent-purple-600"
                        onchange={handle_toggle} />

                    <label for={task.clone().id}
                    title={task.clone().title}
                    class={classes!(label_style, completed_task)}>
                        {&task.title}
                    </label>
                </div>

                <div class="flex justify-between items-center px-2 pb-2">
                    <p class="text-sm text-gray-600 font-bold">{time_and_date}</p>
                    <button title={"Remove Todo"} onclick={handle_click} class="bg-red-600 hover:bg-red-500 px-2.5 py-1.5 rounded">
                        <svg class="w-5" fill="currentColor" viewBox="0 0 16 16">
                          <path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 \
                          1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1H2.5zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 \
                          0 1-1 0v-7a.5.5 0 0 1 .5-.5zM8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 \
                          8 5zm3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0z"/>
                        </svg>
                    </button>
                </div>
            </div>
        </li>
    }
}
