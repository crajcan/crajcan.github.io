use crate::todo::Todo;
use crate::utils::event_helper::{enter_key_pressed, html_input_value, target_input};
use crate::utils::logger::log;
use crate::App;

pub fn create_todo_handler(app: &App) -> impl FnMut(web_sys::Event) {
    move |event: web_sys::Event| {
        if !enter_key_pressed(&event) {
            return;
        }

        if let Some(val) = html_input_value(&event) {
            if val.is_empty() {
                return;
            }

            log("Todo input is not empty");

            let todo = Todo {
                title: val,
                completed: false,
                id: 1,
            };

            // todo.render(&dom);
            // input_el.set_value("");
        }
    }
}
