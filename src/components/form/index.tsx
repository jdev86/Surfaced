import { createSignal, Setter } from "solid-js";
import { Todo } from "../../App";

export interface TodoFormProps {
    createTodo: (body: string) => void
}

export function TodoForm({ createTodo }: TodoFormProps) {
    const [newTodo, setNewTodo] = createSignal<string>('');
  
    return (
      <form>
        <input
          value={newTodo()}
          onChange={e => {
            setNewTodo(e.currentTarget.value);
          }}
        />
        <button
          type="submit"
          onClick={(e) => {
            e.preventDefault();
            createTodo(newTodo());
            setNewTodo('');
          }}
        >
          Add
        </button>
      </form>
    );
  }