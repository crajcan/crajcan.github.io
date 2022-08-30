use web_sys::{Document};

use crate::todo::Todo;

pub struct App;

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn init(&self, dom: Document) {
        self.render(dom);
    }

    pub fn render(&self, dom: Document) {
        let todo = Todo {
            title: "Brush the dog :)".to_string(),
            completed: false,
            id: 1,
        };

        // someone remind me to handle unwraps every now and then! lol gross
        let todo_list = dom
            .query_selector(r#"[data-todo="list"]"#)
            .unwrap()
            .unwrap();
        let first_li = todo.render(dom);

        todo_list.append_child(&first_li).unwrap();
    }
}
