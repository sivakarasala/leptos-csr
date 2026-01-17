use leptos::prelude::*;

#[component]
pub fn SimpleCalculator() -> impl IntoView {
    let first_number = RwSignal::new(0);
    let second_number = RwSignal::new(0);
    let result = RwSignal::new(0);

    let calculate_sum = move |_:()| {
        let sum = first_number.get() + second_number.get();
        result.set(sum);
    };

    let on_first_input = move |ev| {
        let value = event_target_value(&ev);
        let value = value.parse::<i32>().unwrap();
        first_number.set(value);
        calculate_sum(());
    };

    let on_second_input = move |ev| {
        let value = event_target_value(&ev);
        let value = value.parse::<i32>().unwrap();
        second_number.set(value);
        calculate_sum(());
    };

    view! {
        <div style="margin-top: 2rem; padding: 1rem; border: 1px solid #ccc; border-radius: 8px; max-width: 300px;">
            <h2>Simple Calculator</h2>
            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                <div>
                    <label>First number:</label>
                    <input type="number" on:input=on_first_input prop:value=move || first_number.get().to_string() />
                </div>
                <div>
                    <label>Second number:</label>
                    <input type="number" on:input=on_second_input prop:value=move || second_number.get().to_string() />
                </div>
                <div style="margin-top: 1rem; font-weigth: bold;">
                    <span>Result:</span>
                    <span>{move || result.get()}</span>
                </div>
            </div>
        </div>
    }
}