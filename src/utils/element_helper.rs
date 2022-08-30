use web_sys::Element;

pub fn insert_html(element: &Element, markup: &str) {
    element.insert_adjacent_html("afterbegin", markup).unwrap();
}

//element must be an input
pub fn set_input_value(element: &Element, value: &str) {
    if let Some(el) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(element) {
        el.set_value(&value);
    }
}