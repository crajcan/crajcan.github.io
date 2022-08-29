use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Window};

//why?
pub(crate) mod components;
use components::app::App;

mod utils;
use crate::utils::logger::log;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    log("I can log from Rust too! using my modular logger");

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let dom = window.document().expect("should have a document on window");

    // why is rust-analyzer not giving me types?
    let shell: Element = dom
        .get_element_by_id("shell")
        .expect("should have a shell to hold the app");

    let app = App::new();

    app.init(dom);

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
