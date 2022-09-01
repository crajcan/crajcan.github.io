use web_sys::{Document, Element};

use crate::{event_helper::*, todo::Todo, handlers::*};

pub struct App {
    pub input: Element, //owned for now?...
}

impl App {
    pub fn new(dom: &Document) -> App {
        let input: Element = dom.query_selector(r#"[data-todo="new"]"#).unwrap().unwrap();

        App { input }
    }

    pub fn bind_add_todo_handler(&self, dom: &Document) {
        add_event_listener(&self.input, "keyup", add_todo_handler())
    }

    pub fn init(&self, dom: Document) {
        self.bind_add_todo_handler(&dom);

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
