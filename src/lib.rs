use derive_more::{Display, Error};
use std::error::Error;

use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Window};

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let dom = window.document().expect("should have a document on window");
    let shell = dom
        .get_element_by_id("slider")
        .expect("should have an app!");

    let app = App::new();

    shell.append_child(&app.render(dom))?;
    log("I can log from Rust too!");

    Ok(())
}

struct App {
    slider: Vec<Vec<i32>>,
}

impl App {
    fn new() -> App {
        App {
            slider: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        }
    }

    fn render(&self, dom: Document) -> Element {
        let app = dom.create_element("div").unwrap();
        app.set_inner_html("Hello from declarative App!");

        app
    }
}

#[derive(Debug, Display, Error)]
struct myError {
    message: String,
}

impl From<JsValue> for myError {
    fn from(error: JsValue) -> Self {
        myError {
            message: format!("{:?}", error),
        }
    }
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}
