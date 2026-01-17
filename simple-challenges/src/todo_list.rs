use leptos::prelude::*;
use web_sys::{KeyboardEvent, MouseEvent};

#[component]
pub fn TodoList() -> impl IntoView {
    let todos = RwSignal::new(vec!["Learn Leptos".to_string()]);

    let new_todo = RwSignal::new(String::new());

    let add_todo = move |_: MouseEvent| {
        let todo = new_todo.get().to_string();
        if !todo.trim().is_empty() {
            todos.update(|list| list.push(todo));
        }
        new_todo.set(String::new());
    };

    let remove_todo = move |index: usize| {
        todos.update(|list| {
            list.remove(index);
        });
    };

    let handle_todo_change = move |ev| {
        let value = event_target_value(&ev);
        new_todo.set(value);
    };

    let handle_todo_keypress = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            
        }
    };

    let render_todos = move || {
        todos.get()
            .into_iter()
            .enumerate()
            .map(|(index, todo)| {
                view! {
                    <li>
                        {todo} <button on:click=move |_| remove_todo(index) style="margin-left: 10px">
                        Remove</button>
                    </li>
                }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div>
            <h3>Todo List</h3>
            <br />
            <div>
                <input
                    type="text"
                    placeholder="Add a new todo"
                    prop:value=new_todo
                    on:input=handle_todo_change
                    on:keypress=handle_todo_keypress
                />
                <button on:click=add_todo>Add</button>
            </div>
            <ul>
                {render_todos}
            </ul>
        </div>
    }
}