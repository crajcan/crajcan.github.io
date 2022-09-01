use crate::utils::event_helper::{enter_key_pressed, html_input_value};
use crate::utils::logger::log;

pub fn add_todo_handler() -> impl FnMut(web_sys::Event) {
    move |event: web_sys::Event| {
        if !enter_key_pressed(&event) {
            return;
        }

        if let Some(val) = html_input_value(&event) {
            if val.is_empty() {
                return;
            }

            log("Todo input is not empty");

            // let todo = Todo::new(value);
            // todo.render(&dom);
            // input_el.set_value("");
        }
    }
}
