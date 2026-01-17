use leptos::prelude::*;

mod simple_counter;
mod simple_counter_rw;
mod derived_signals;
mod simple_calculator;
mod todo_list;

use simple_counter::SimpleCounter;
use simple_counter_rw::SimpleCounterRw;
use derived_signals::DerivedSignals;
use simple_calculator::SimpleCalculator;
use todo_list::TodoList;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { 
            <div>
                <SimpleCounter /> 
                <SimpleCounterRw />
                <DerivedSignals />
                <SimpleCalculator />
                <TodoList />
            </div>
        }
    })
}