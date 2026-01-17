use leptos::prelude::*;

#[component]
pub fn SimpleCounterRw() -> impl IntoView {
    let value = RwSignal::new(0);

    let increment = move |_| value.update(|value| *value += 1);
    let decrement = move |_| value.update(|value| *value -= 1);
    let clear = move |_| value.set(0);

    view! {
        <div>
            <h1>RwSignal</h1>
            <br />
            <button on:click=clear>Clear</button>
            <button on:click=decrement>-1</button>
            <span>Value: {value} !</span>
            <button on:click=increment>+1</button>
        </div>
    }
}