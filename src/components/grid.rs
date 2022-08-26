use web_sys::{Document, Element, Window};

pub struct Grid {
    // Tonight: implement a table of divs where each has an onclick handler or
    // something that can change the page still in a declarative style
    //
    // Refactor this file
    hex_colors: Vec<Vec<String>>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            hex_colors: vec![
                vec![
                    "#ff0000".to_string(),
                    "#00ff00".to_string(),
                    "#0000ff".to_string(),
                ],
                vec![
                    "#ff0000".to_string(),
                    "#00ff00".to_string(),
                    "#0000ff".to_string(),
                ],
                vec![
                    "#ff0000".to_string(),
                    "#00ff00".to_string(),
                    "#0000ff".to_string(),
                ],
            ],
        }
    }

    pub fn render(&self, dom: Document) -> Element {
        let mut html = String::from("ooooooey I'm a string");

        let res = dom.create_element("div").unwrap();
        res.set_inner_html(&html);

        res
    }
}
