use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Element, EventTarget};

use crate::{todo::Todo, utils::logger::log};

pub struct App {
    pub input: Element, //owned for now?...
}

pub fn bind_add_todo<T>(e: &Element, handler: T)
where
    T: 'static + FnMut(web_sys::Event),
{
    let cb = Closure::new::<T>(handler);
    e.add_event_listener_with_callback("keyup", cb.as_ref().unchecked_ref())
        .unwrap();
    cb.forget();
}

impl App {
    pub fn new(dom: &Document) -> App {
        let input: Element = dom.query_selector(r#"[data-todo="new"]"#).unwrap().unwrap();

        App { input }
    }

    const ENTER_KEY_CODE: u32 = 13;

    pub fn init(&self, dom: Document) {
        let cb = move |event: web_sys::Event| {
            if let Some(key) = wasm_bindgen::JsCast::dyn_ref::<web_sys::KeyboardEvent>(&event) {
                if key.key_code() == Self::ENTER_KEY_CODE {
                    if let Some(target) = event.target() {
                        if let Some(input_el) =
                            wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target)
                        {
                            if !input_el.value().is_empty() {
                                log("Todo input is not empty");
                                // let todo = Todo::new(value);
                                // todo.render(&dom);
                                // input_el.set_value("");
                            }
                        }
                    }
                }
            }
        };
        bind_add_todo(&self.input, cb);

        self.render(dom);
    }

    pub fn render(&self, dom: Document) {
        let todo = Todo {
            title: "Brush the dog :)".to_string(),
            completed: false,
            id: 1,
        };

        // someone remind me to handle unwraps every now and then! lol gross
        let todo_list: Element = dom
            .query_selector(r#"[data-todo="list"]"#)
            .unwrap()
            .unwrap();
        let first_li = todo.render(dom);

        todo_list.append_child(&first_li).unwrap();
    }
}
