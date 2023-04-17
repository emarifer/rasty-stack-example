use gloo_dialogs::alert;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_effect, use_node_ref, Callback, Html, MouseEvent, Properties,
};

#[derive(Properties, PartialEq)]
pub struct TaskFormProps {
    pub createtask: Callback<String>,
}

#[function_component(TaskForm)]
pub fn task_form(props: &TaskFormProps) -> Html {
    let input_ref = use_node_ref();

    let handle_click = {
        let input_ref = input_ref.clone();
        let on_create_task = props.createtask.clone();

        Callback::from(move |_e: MouseEvent| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let title = input.value();

                if title.is_empty() {
                    alert("This field can not be blank");
                    return;
                }

                on_create_task.emit(input.value());
                // Reset the input
                input.set_value("");
            }
        })
    };

    {
        let input_ref = input_ref.clone();

        use_effect(move || {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.focus().unwrap();
            }
        });
    }

    html! {
        <div class="mx-auto w-full">
            <label class="text-xl font-semibold" for="new-task">{"Add Item"}</label>

            <hr class="mb-4 border-t-2" />

            <div class="flex justify-center items-center gap-4">
                <input
                    ref={input_ref}
                    class="rounded-md focus:outline-none focus:ring focus:ring-blue-400 text-xl px-4 py-2 bg-slate-700"
                    id="new-task"
                    type="text"
                    placeholder="Enter a to do…" />

                <button onclick={handle_click} title="Add Todo" class="bg-sky-600 hover:bg-sky-400 rounded-md text-xl px-4 py-2">
                    <svg class="w-7" fill="currentColor" viewBox="0 0 24 24">
                        <path d="M2 18H12V20H2V18ZM2 11H22V13H2V11ZM2 4H22V6H2V4ZM18 \
                        18V15H20V18H23V20H20V23H18V20H15V18H18Z" />
                    </svg>
                </button>
            </div>
        </div>
    }
}
