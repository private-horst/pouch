use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_version() -> String {
    "0.0.3-alpha".into()
}
