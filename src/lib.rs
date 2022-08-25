use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let dom = window.document().expect("should have a document on window");
    let app = dom
        .get_element_by_id("slider")
        .expect("should have an app!");

    let val = dom.create_element("div")?;
    val.set_inner_html("Hello from div app!");

    app.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
