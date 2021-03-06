use js_sys::Promise;
use wasm_bindgen::prelude::*;

// TODO find a solution for browser and node
#[wasm_bindgen(module = "pouchdb")]
extern "C" {

    #[wasm_bindgen(js_name = default)]
    pub type PouchDB;

    #[wasm_bindgen(constructor, js_class = default)]
    pub fn new(name: String) -> PouchDB;

    #[wasm_bindgen(method, js_class = default)]
    pub fn close(this: &PouchDB) -> Promise;
}
