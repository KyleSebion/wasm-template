use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().expect("window");
    let document = window.document().expect("document");
    let body = document.body().expect("body");
    let p_el = document.create_element("p")?;
    p_el.set_inner_html("Hello World");
    body.append_child(&p_el)?;
    log!("done");
    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
