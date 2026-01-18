use leptos::prelude::*;

use crate::components::counter_basic::CounterBasic;
use crate::components::counter_props::CounterProps;

#[component]
pub fn CounterDemos() -> impl IntoView {
    const INITIAL_VALUE: i32 = 10;
    const STEP: i32 = 5;

    view! {
        <div>
            <section>
                <h2>1. Basic Counter</h2>
                <CounterBasic initial_value=INITIAL_VALUE step=STEP title="Basic Counter".to_string() />
            </section>

            <section>
                <h2>2. Counter with Props</h2>
                <CounterProps title="Custom Prop Counter" />
            </section>
        </div>
    }
}