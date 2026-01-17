use leptos::prelude::*;

#[component]
pub fn DerivedSignals() -> impl IntoView {
    let count = RwSignal::new(0);

    let double = move || count.get() * 2;
    let square = move || count.get() * count.get();
    let is_even = move || count.get() % 2 == 0;

    let increment = move |_| count.update(|value| *value += 1);
    let decrement = move |_| count.update(|value| *value -= 1);
    let clear = move |_| count.set(0);

    view! {
        <div>
            <h1>Derived Signals</h1>
            <p>Derived signals automatically update when their dependencies change</p>
            <br />
            <div>
                <button on:click=decrement>-1</button>
                <span>{count}</span>
                <button on:click=increment>+1</button>
                <button on:click=clear>Reset</button>
            </div>
            <br />
            <div>
                <h2>Derived Values</h2>
                <ul>
                    <li>
                        <strong>Double:</strong>
                        <span>{double}</span>
                    </li>
                    <li>
                        <strong>Square:</strong>
                        <span>{square}</span>
                    </li>
                    <li>
                        <strong>Is Even:</strong>
                        <span>{move || if is_even() { "Yes" } else { "No" }}</span>
                    </li>
                </ul>
            </div>
        </div>
    }
}