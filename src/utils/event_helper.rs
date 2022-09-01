use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

const ENTER_KEY_CODE: u32 = 13;

pub fn add_event_listener<T>(e: &Element, event_name: &str, handler: T)
where
    T: 'static + FnMut(web_sys::Event),
{
    let cb = Closure::new::<T>(handler);
    e.add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref())
        .unwrap();

    cb.forget();
}

// Cast event to keyboard event. None if any other type of event
pub fn keyboard_event(event: &web_sys::Event) -> Option<&web_sys::KeyboardEvent> {
    wasm_bindgen::JsCast::dyn_ref::<web_sys::KeyboardEvent>(event)
}

// Use this to guard a handler so that it only runs when the enter key is pressed
pub fn enter_key_pressed(event: &web_sys::Event) -> bool {
    if let Some(keyboard_event) = keyboard_event(event) {
        keyboard_event.key_code() == ENTER_KEY_CODE
    } else {
        false
    }
}

//cast the event target to a html input element
pub fn target_input(target: &web_sys::EventTarget) -> Option<&web_sys::HtmlInputElement> {
    wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(target)
}

// get the string from an event targeting an input element
pub fn html_input_value(event: &web_sys::Event) -> Option<String> {
    let event_target = match event.target() {
        Some(et) => et,
        None => return None,
    };

    match target_input(&event_target) {
        Some(hie) => Some(hie.value()),
        None => None,
    }
}
