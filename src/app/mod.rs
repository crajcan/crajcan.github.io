use web_sys::{Document, Element, Window};

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
        app.set_inner_html("Hello from declarative App!");

        app
    }
}

struct Grid {
    // Tonight: implement a table of divs where each has an onclick handler or
    // something that can change the page still in a declarative style
    //
    // seems like I shorted out rust-analyzer and copilot, figure that out
    // Refactor this file
    hex_colors: Vec<Vec<String>>,
}