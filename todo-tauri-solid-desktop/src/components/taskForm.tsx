import { createTodo, todoList, setTodoList } from "../api";

export function TaskForm() {
  const handleSubmit = async (e: Event) => {
    e.stopPropagation();
    e.preventDefault();

    const formData = new FormData(e.target as HTMLFormElement);

    const title = formData.get("new-task") as string;
    // "new-task" es el name del input

    // console.log(`El título es: ${title}`);
    const newTodo = await createTodo(title);

    newTodo && setTodoList([newTodo, ...todoList]);

    (e.target as HTMLFormElement).reset();
  };
  // undefined es asignable al tipo void:
  // https://stackoverflow.com/questions/69728470/why-is-undefined-assignable-to-void

  return (
    <div class="mx-auto w-full">
      <label class="text-xl font-semibold" for="new-task">
        Add Item
      </label>

      <hr class="mb-4 border-t-2" />

      <form
        class="flex justify-center items-center gap-4"
        onsubmit={handleSubmit}
      >
        <input
          autofocus
          class="rounded-md focus:outline-none focus:ring focus:ring-blue-400 text-xl px-4 py-2 bg-slate-700"
          id="new-task"
          name="new-task"
          type="text"
          required
          placeholder="Enter a to do…"
        />

        <button
          title="Add Todo"
          class="bg-sky-600 hover:bg-sky-400 rounded-md text-xl px-4 py-2"
        >
          <svg class="w-7" fill="currentColor" viewBox="0 0 24 24">
            <path d="M2 18H12V20H2V18ZM2 11H22V13H2V11ZM2 4H22V6H2V4ZM18 18V15H20V18H23V20H20V23H18V20H15V18H18Z" />
          </svg>
        </button>
      </form>
    </div>
  );
}

/*
 * ASOCIACIÓN DE BUTTON Y FORMULARIO & FORMDATA. VER:
 * https://developer.mozilla.org/en-US/docs/Web/API/FormData/get
 * https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attributes
 */
