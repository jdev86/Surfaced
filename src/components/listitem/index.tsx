import { Todo } from "../../App";

export interface TodoListItemProps {
    todo: Todo
    updateTodo: (id: number, done: boolean) => void
    deleteTodo: (id: number) => void
}

export function TodoListItem({todo, updateTodo, deleteTodo}: TodoListItemProps) {
    const {body, done} = todo
    return (
        <li
        style={{
          'text-decoration': todo.done ? 'line-through' : undefined,
        }}
      >
        <label>
          <input
            type="checkbox"
            checked={done}
            onChange={() => {
              updateTodo(todo.id, todo.done);
            }}
            style={{"margin-right": ".5rem"}}
          />
          {body}
        </label>
        <button 
          style={{ width: 'auto', "text-align": 'center', "margin-left": '1rem' }} 
          onClick={_ => deleteTodo(todo.id)}>X</button>
      </li>
    );
  }