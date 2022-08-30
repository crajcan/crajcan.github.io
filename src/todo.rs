use web_sys::{Document, Element};
use crate::element_helper::{insert_html, set_input_value};

pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub id: usize,
}

impl Todo {
    pub fn render(&self, dom: Document) -> Element {
        let li = dom.create_element("li").unwrap();

        li.set_attribute("data-id", &self.id.to_string()).unwrap();
        if self.completed {
            li.set_attribute("class", "completed").unwrap();
        }
        insert_html(
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
        label.set_text_content(Some(&self.title));

        let input = li.query_selector(r#"[data-todo="edit"]"#).unwrap().unwrap();
        set_input_value(&input, &self.title);

        li
    }
}