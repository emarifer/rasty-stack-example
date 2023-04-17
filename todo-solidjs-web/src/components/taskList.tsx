import { For, onMount } from "solid-js";
import {
  deleteTodo,
  getTodos,
  setTodoList,
  todoList,
  toggleTodo,
} from "../api";
import { convertDate } from "../helpers";
import { Todo } from "../models";

export function TaskList() {
  onMount(async () => {
    const todos = await getTodos();

    todos && setTodoList(todos);
  });
  // undefined es asignable al tipo void:
  // https://stackoverflow.com/questions/69728470/why-is-undefined-assignable-to-void

  const toggleStatus = async (taskId: string) => {
    const row = await toggleTodo(taskId);

    if (row?.rows_affected === 1) {
      setTodoList(
        (todo) => todo.id === taskId,
        "completed",
        (completed) => !completed
      );
    }
  };

  const deleteTask = async (taskId: string) => {
    const row = await deleteTodo(taskId);

    if (row?.rows_affected === 1) {
      setTodoList(todoList.filter((item) => item.id !== taskId));
    }
  };

  return (
    <ul>
      <For each={todoList}>
        {(todo: Todo) => (
          <li>
            <div class="flex flex-col my-2 pl-4 py-1 border rounded border-gray-700 hover:-translate-y-1.5 ease-in duration-300">
              <div class="flex items-center">
                <input
                  id={todo.id}
                  type="checkbox"
                  checked={todo.completed}
                  class="w-5 h-5 accent-purple-600"
                  onclick={() => toggleStatus(todo.id)}
                />

                <label
                  for={todo.id}
                  title={todo.title}
                  class={`w-full p-3 ml-2 text-lg truncate ${
                    todo.completed ? "text-gray-600 line-through" : ""
                  }`}
                >
                  {todo.title}
                </label>
              </div>

              <div class="flex justify-between items-center px-2 pb-2">
                <p class="text-sm text-gray-600 font-bold">
                  {convertDate(todo.created_at)}
                </p>
                <button
                  title="Remove Todo"
                  onclick={() => deleteTask(todo.id)}
                  class="bg-red-600 hover:bg-red-500 px-2.5 py-1.5 rounded"
                >
                  <svg class="w-5" fill="currentColor" viewBox="0 0 16 16">
                    <path
                      d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 \
                          1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1H2.5zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 \
                          0 1-1 0v-7a.5.5 0 0 1 .5-.5zM8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 \
                          8 5zm3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0z"
                    />
                  </svg>
                </button>
              </div>
            </div>
          </li>
        )}
      </For>
    </ul>
  );
}
