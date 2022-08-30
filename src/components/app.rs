use web_sys::{Document, Element};

use crate::components::grid::Grid;

pub struct App;

// needs its own file
pub struct Todo {
    title: String,
    completed: bool,
    id: usize,
}

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn init(&self, dom: Document) {
        self.render(dom);
    }

    // export const insertHTML = (el, markup) => {
    //     el.insertAdjacentHTML('afterbegin', markup);
    // }

    // this needs to move to a helper
    pub fn insert_html(element: &Element, markup: &str) {
        element.insert_adjacent_html("afterbegin", markup).unwrap();
    }

    // put this in the same helper as insert_html for now 
    pub fn set_value(element: &Element, value: &str) {
        if let Some(el) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(element) {
            el.set_value(&value);
        }
    }

    // this should be todo.render() or something
    pub fn create_todo_item(todo: Todo, dom: Document) -> Element {
        let li = dom.create_element("li").unwrap();

        li.set_attribute("data-id", &todo.id.to_string()).unwrap();
        if todo.completed {
            li.set_attribute("class", "completed").unwrap();
        }
        Self::insert_html(
            &li,
            r#"<div class="view">
         		<input data-todo="toggle" class="toggle" type="checkbox" "checked">
         		<label data-todo="label"></label>
         		<button class="destroy" data-todo="destroy"></button>
         	</div>
         	<input class="edit" data-todo="edit">"#,
        );

        let label = li
            .query_selector(r#"[data-todo="label"]"#)
            .unwrap()
            .unwrap();
        label.set_text_content(Some(&todo.title));

        let input = li.query_selector(r#"[data-todo="edit"]"#).unwrap().unwrap();
        Self::set_value(&input, &todo.title);

        li
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
        let first_li = Self::create_todo_item(todo, dom);

        todo_list.append_child(&first_li).unwrap();
    }
}
