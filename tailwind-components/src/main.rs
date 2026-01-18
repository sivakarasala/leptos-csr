use leptos::{leptos_dom::logging::console_log, prelude::*};

mod components;
mod counter_demos;
mod tailwind_counter;

use counter_demos::CounterDemos;
use tailwind_counter::TailwindCounter;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        // view! { <CounterDemos /> }
        view! { <TailwindCounter /> }
    });
}