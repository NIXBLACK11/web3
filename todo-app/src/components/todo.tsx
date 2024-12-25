import React, { useEffect, useState } from 'react';
import { PlusCircle, Trash2 } from 'lucide-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

interface Todo {
  id: number;
  text: string;
  completed: boolean;
}

const TodoApp = () => {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [newTodo, setNewTodo] = useState('');
  const [initialized, setInitialized] = useState(false);

  useEffect(()=> {
	
  }, [initialized]);

  const addTodo = (e: React.FormEvent) => {
	e.preventDefault();
	if (newTodo.trim() !== '') {
	setTodos([
		...todos,
		{
		id: Date.now(),
		text: newTodo.trim(),
		completed: false,
		},
	]);
	setNewTodo('');
	}
  };

  const toggleTodo = (id: number) => {
    setTodos(
      todos.map((todo) =>
        todo.id === id ? { ...todo, completed: !todo.completed } : todo
      )
    );
  };

  const deleteTodo = (id: number) => {
    setTodos(todos.filter((todo) => todo.id !== id));
  };

  return (
    <div className="min-h-screen bg-black text-white p-4">
      <div className="max-w-2xl mx-auto">
        {/* Header with Login Button */}
        <div className="flex justify-between items-center mb-8 border-b border-gray-800 pb-6">
          <h1 className="text-3xl font-bold bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent">
            Todo App
          </h1>
          <WalletMultiButton />
        </div>

        {/* Todo Form */}
        <form onSubmit={addTodo} className="mb-6 flex gap-2">
          <input
            type="text"
            value={newTodo}
            onChange={(e) => setNewTodo(e.target.value)}
            placeholder="Add a new todo..."
            className="flex-1 px-4 py-2 rounded-lg bg-gray-900 border border-gray-800 focus:outline-none focus:border-blue-500 text-gray-100 placeholder-gray-500"
          />
          <button
            type="submit"
            className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
          >
            <PlusCircle size={18} />
            Add
          </button>
        </form>

        {/* Todo List */}
        <div className="space-y-2">
          {todos.map((todo) => (
            <div
              key={todo.id}
              className="flex items-center gap-2 p-4 bg-gray-900 rounded-lg border border-gray-800 hover:border-gray-700 transition-colors"
            >
              <input
                type="checkbox"
                checked={todo.completed}
                onChange={() => toggleTodo(todo.id)}
                className="w-5 h-5 bg-gray-800 border-gray-700 rounded text-blue-500 focus:ring-blue-500 focus:ring-offset-gray-900"
              />
              <span
                className={`flex-1 ${
                  todo.completed ? 'text-gray-500 line-through' : 'text-gray-100'
                }`}
              >
                {todo.text}
              </span>
              <button
                onClick={() => deleteTodo(todo.id)}
                className="text-gray-500 hover:text-red-400 transition-colors"
              >
                <Trash2 size={18} />
              </button>
            </div>
          ))}
        </div>

        {/* Empty State */}
        {todos.length === 0 && (
          <div className="text-center text-gray-500 mt-8 p-8 border border-gray-800 rounded-lg bg-gray-900">
            No todos yet. Add some above!
          </div>
        )}
      </div>
    </div>
  );
};

export default TodoApp;