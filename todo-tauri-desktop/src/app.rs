use chrono::{offset::Local, Datelike};
use std::rc::Rc;
use yew::{
    function_component, html, use_effect_with_deps, use_reducer, Callback, Html, KeyboardEvent,
};

use crate::{
    components::{task_form::TaskForm, task_list::TaskList},
    controllers::*,
    helpers::onclose,
    state::TaskState,
};

#[function_component(App)]
pub fn app() -> Html {
    let tasks = use_reducer(TaskState::default);
    let task_controller = Rc::new(TaskController::new(tasks.clone()));

    // Get all tasks on app startup
    {
        let task_controller = task_controller.clone();

        use_effect_with_deps(
            move |_| {
                task_controller.init_tasks();
                || {} // return empty destructor closure (cleanup use_effect)
            },
            (),
        ); // only call on first render
    }

    let on_create_task = {
        let task_controller = task_controller.clone();

        Callback::from(move |title: String| task_controller.create_task(title))
    };

    let on_delete_task = {
        let task_controller = task_controller.clone();

        Callback::from(move |id: String| task_controller.delete_task(id))
    };

    let on_toggle_task = {
        let task_controller = task_controller.clone();

        Callback::from(move |id: String| task_controller.toggle_task(id))
    };

    let on_close = Callback::from(|e: KeyboardEvent| onclose(e));

    html! {
        <div onkeyup={on_close} tabindex={0} class="flex flex-col mt-14 w-[400px] mx-auto gap-6">
            <header class="flex flex-col mx-auto w-full">
                <h1 class="text-3xl text-center font-black mb-5">{"Whattodo!"}</h1>

                <img class="h-32 mb-8 hover:scale-110 ease-in-out duration-500" src="img/task-done-flat.svg" alt="Todo App logo"/>

                <TaskForm createtask={on_create_task} />
            </header>


            <main class="mx-auto my-4 w-full">
                <h1 class="text-3xl font-black">{"Todo"}</h1>

                <hr class="mb-6 border-t-2" />

                <TaskList tasks={tasks.tasks.clone()} deletetask={on_delete_task} toggletask={on_toggle_task} />
            </main>

            <footer class="mt-3 mb-6">
                <a
                  class="italic tracking-wider hover:text-sky-500 ease-in duration-300"
                  href="https://github.com/emarifer/rasty-stack-example"
                  target="_blank"
                >
                  {format!("MIT Licensed | Copyright © {} Enrique Marín", Local::now().year())}
                </a>
            </footer>
        </div>
    }
}

/*
 * ARRANCAR UN CONTENEDOR DOCKER DE SURREALDB CON UN FICHERO docker-compose.yml:
 * sudo docker compose up -d
 *
 * ENTRAR EN LA CLI DE SURREALDB EN EL CONTENEDOR CREADO:
 * sudo docker exec -it surrealdb /surreal sql -c http://localhost:8000 -u root -p root --ns namespace --db database --pretty
 *
 * LISTAR LA TABLE EN LA CLI DE SURREALDB:
 * SELECT * FROM task ORDER BY created_at DESC;
 *
 * VER SI EL CONTENEDOR DOCKER ESTÁ INICIADO:
 * sudo docker ps  (CON EL FLAG --a SE LISTAN TODOS LOS CONTENEDORES, ACTIVOS Y NO ACTIVOS)
 *
 * DETENER EL CONTENEDOR DE DOCKER:
 * sudo docker stop surrealdb
 *
 * VOLVER A INICIAR EL CONTENEDOR DE DOCKER:
 * sudo docker start surrealdb
 */

/*
 * DEVTOOLS EN MODO RELEASE/BUILD EN TAURI. VER:
 * https://github.com/tauri-apps/tauri/discussions/3059
 */

/*
 * https://doc.rust-lang.org/std/collections/vec_deque/struct.VecDeque.html
 * https://freeiconshop.com/
 * https://tailwindcss.com/docs/accent-color
 * https://docs.rs/yew/0.20.0/yew/functional/fn.use_reducer.html
 * https://flowbite.com/docs/forms/checkbox/#bordered
 */
