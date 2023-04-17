import axios, { AxiosError } from "axios";
import { createStore } from "solid-js/store";
import { AffectedRow, Todo } from "../models";
import Swal from "sweetalert2";

export const [todoList, setTodoList] = createStore([] as Todo[]);

const errorAlert = (error: AxiosError) =>
  Swal.fire({
    background: "#292b2c",
    color: "white",
    title: "Error",
    text: error.message,
    icon: "error",
    confirmButtonText: "Ok",
  });

const todoApi = axios.create({
  baseURL: "http://localhost:8080",
});

export const getTodos = async (): Promise<void | Todo[]> => {
  return todoApi
    .get<Todo[]>("/todos")
    .then((resp) => resp.data)
    .catch((e: AxiosError) => {
      errorAlert(e);
    });
};
// undefined es asignable al tipo void:
// https://stackoverflow.com/questions/69728470/why-is-undefined-assignable-to-void

export const createTodo = async (title: string): Promise<void | Todo> => {
  return await todoApi
    .post<Todo>(`/todo/${title}`)
    .then((resp) => resp.data)
    .catch((e: AxiosError) => {
      errorAlert(e);
    });
};

export const toggleTodo = async (id: string): Promise<void | AffectedRow> => {
  return todoApi
    .patch<AffectedRow>(`/todo/${id}`)
    .then((resp) => resp.data)
    .catch((e: AxiosError) => {
      errorAlert(e);
    });
};

export const deleteTodo = async (id: string): Promise<void | AffectedRow> => {
  return todoApi
    .delete<AffectedRow>(`/todo/${id}`)
    .then((resp) => resp.data)
    .catch((e: AxiosError) => {
      errorAlert(e);
    });
};

/*
 * MANEJO DE ERRORES CON AXIOS Y TYPESCRIPT. VER:
 * https://github.com/axios/axios/issues/3612
 * https://stackoverflow.com/questions/69264472/axios-error-typescript-annotation-must-be-any-or-unknown-if
 * https://dev.to/mdmostafizurrahaman/handle-axios-error-in-typescript-4mf9
 */
