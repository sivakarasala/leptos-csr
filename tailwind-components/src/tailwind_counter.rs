use leptos::prelude::*;

#[component]
pub fn TailwindCounter() -> impl IntoView {
    let value = RwSignal::new(0);

    let increment = move |_| value.update(|value| *value += 1);
    let decrement = move |_| value.update(|value| *value -= 1);
    let clear = move |_| value.set(0);

    const CLASS_BUTTON: &str = "bg-sky-500 text-white p-2 rounded-md";

    view! {
        <div class="flex gap-2 p-4">
            <button class=CLASS_BUTTON on:click=clear>Clear</button>
            <button class=CLASS_BUTTON on:click=decrement>-1</button>
            <span class="text-2xl font-bold">Value: {value} !</span>
            <button class=CLASS_BUTTON on:click=increment>+1</button>
        </div>
    }
}