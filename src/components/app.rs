use web_sys::{Document, Element};

use crate::components::grid::Grid;

pub struct App;

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn init(&self, dom: Document) {
        let shell: Element = dom
            .get_element_by_id("shell")
            .expect("should have a shell to hold the app");

        shell.append_child(&self.render(dom)).unwrap();
    }

    pub fn render(&self, dom: Document) -> Element {
        let app = dom.create_element("div").unwrap();
        app.set_inner_html("Hello from my new blue declarative App!");

        let grid = Grid::new();
        app.append_child(&grid.render(dom)).unwrap();

        app
    }
}
