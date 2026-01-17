use leptos::prelude::*;

#[component]
pub fn SimpleCounter() -> impl IntoView {
    let (value, set_value) = signal::<i32>(0);
    let increment = move |_| set_value.update(|value| *value += 1);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let clear = move |_| set_value.set(0);

    view! {
        <div>
            <button on:click=clear>Clear</button>
            <button on:click=decrement>-1</button>
            <span>Value: {value}!</span>
            <button on:click=increment>+1</button>
        </div>
    }
}