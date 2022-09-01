use wasm_bindgen::prelude::*;

//why?
pub(crate) mod components;
use components::app::App;

mod handlers;
mod todo;

mod utils;
use crate::utils::element_helper;
use crate::utils::event_helper;
use crate::utils::logger::log;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    log("I can log from Rust too! using my modular logger");

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let dom = window.document().expect("should have a document on window");

    let app = App::new(&dom);

    app.init(dom);

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
