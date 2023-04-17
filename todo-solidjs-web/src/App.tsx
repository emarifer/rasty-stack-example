import logo from "./assets/task-done-flat.svg";
import { TaskForm, TaskList } from "./components";

function App() {
  return (
    <div class="flex flex-col mt-14 w-[400px] mx-auto gap-6">
      <header class="flex flex-col mx-auto w-full">
        <h1 class="text-3xl text-center font-black mb-5">Whattodo!</h1>

        <img
          class="w-32 mb-8 mx-auto hover:scale-110 ease-in-out duration-500"
          src={logo}
          alt="Todo App logo"
        />

        <TaskForm />
      </header>

      <main class="mx-auto my-4 w-full">
        <h2 class="text-3xl font-black">Todo</h2>

        <hr class="mb-6 border-t-2" />

        <TaskList />
      </main>

      <footer class="mt-3 mb-6">
        <a
          class="italic tracking-wider hover:text-sky-500 ease-in duration-300"
          href="https://github.com/emarifer/rasty-stack-example"
          target="_blank"
        >
          MIT Licensed | Copyright © {new Date().getFullYear()} Enrique Marín
        </a>
      </footer>
    </div>
  );
}

export default App;

/*
 * https://blog.logrocket.com/build-task-tracker-solidjs-typescript/
 * https://github.com/ebenezerdon/solid-task-tracker
 * https://www.solidjs.com/tutorial/lifecycles_onmount
 * https://www.solidjs.com/tutorial/stores_nested_reactivity
 * https://www.solidjs.com/tutorial/flow_portal?solved
 *
 * https://www.solidjs.com/tutorial/async_resources
 * https://dev.to/codewithahsan/solidjs-crash-course-building-a-rest-api-client-part-1-57ee
 * https://dev.to/csarnataro/how-to-submit-a-form-with-solid-js-286d
 * https://www.youtube.com/watch?v=aXpTR60AMQc
 * https://www.thisdot.co/blog/how-to-handle-async-data-fetching-using-createresource-in-solidjs
 *
 * https://javascript.info/date#:~:text=Date.parse%20from%20a%20string&text=The%20string%20format%20should%20be,%2C%20minutes%2C%20seconds%20and%20milliseconds.
 * https://stackoverflow.com/questions/6040515/how-do-i-get-month-and-date-of-javascript-in-2-digit-format
 */

/*
 * MANEJO DE ERRORES CON AXIOS Y TYPESCRIPT. VER:
 * https://github.com/axios/axios/issues/3612
 * https://stackoverflow.com/questions/69264472/axios-error-typescript-annotation-must-be-any-or-unknown-if
 * https://dev.to/mdmostafizurrahaman/handle-axios-error-in-typescript-4mf9
 */

/*
 * ASOCIACIÓN DE BUTTON Y FORMULARIO & FORMDATA. VER:
 * https://developer.mozilla.org/en-US/docs/Web/API/FormData/get
 * https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attributes
 */
