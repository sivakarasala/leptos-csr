use leptos::prelude::*;

#[component]
pub fn CounterProps(
    #[prop(default = 10)] initial_value: i32,
    #[prop(default = 5)] step: i32,
    #[prop(into)] title: String,
) -> impl IntoView {
    let value = RwSignal::new(initial_value);

    let increment = move |_| value.update(|val| *val += step);
    let decrement = move |_| value.update(|val| *val -= step);
    let reset = move |_| value.set(initial_value);

    view! {
        <div>
            <h3>{title}</h3>
            <button on:click=reset>Reset</button>
            <button on:click=decrement>{format!("-{step}")}</button>
            <span>Value: {value}</span>
            <button on:click=increment>{format!("+{step}")}</button>
        </div>
    }
}