import { invoke } from '@tauri-apps/api/tauri';
import { Component, createSignal, For, onMount } from 'solid-js';
import { TodoForm } from './components/form';
import { TodoListItem } from './components/listitem';

export type Todo = {
  id: number
  body: string
  done: boolean
}

const App: Component = () => {
  const [todos, setTodos] = createSignal([]);

  function createTodo(body: string) {
    if(body === '') {
      return
    }
    invoke('todos_create', { body }).then(loadTodos)
  }

  function updateTodo(id: number, done: boolean){
    invoke("todos_toggle", {id, done}).then(loadTodos)
  }

  function deleteTodo(id: number){
    invoke("todos_delete", {id}).then(loadTodos)
  }
  
  function loadTodos(){
    invoke("todos_list").then((t: any) => setTodos(JSON.parse(t)))
  }

  onMount(() => loadTodos())

  return (
    <>
    <h2>Surfaced - Surfacing your tasks</h2>
      <ul>
        <For each={todos()}>{(todo: Todo) => <TodoListItem todo={todo} updateTodo={updateTodo} deleteTodo={deleteTodo} />}</For>
      </ul>
      <TodoForm createTodo={createTodo} />
    </>
  );
};

export default App;