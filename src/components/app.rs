use web_sys::{Document, Element, Window};

use crate::components::grid::Grid;

pub struct App {
    slider: Vec<Vec<i32>>,
}

impl App {
    pub fn new() -> App {
        App {
            slider: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        }
    }

    pub fn render(&self, dom: Document) -> Element {
        let app = dom.create_element("div").unwrap();
        app.set_inner_html("Hello from my declarative App!");

        let grid = Grid::new();
        app.append_child(&grid.render(dom)).unwrap();

        app
    }
}
