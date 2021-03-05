use js_sys::Promise;
use wasm_bindgen::prelude::*;

// TODO find a solution for browser and node
#[wasm_bindgen(module = "pouchdb")]
extern "C" {

    // #[wasm_bindgen(js_name = default)]
    type PouchDB;

    // TODO solve the "PouchDB is not a constructur" problem
    // #[wasm_bindgen(constructor, js_class = default)]
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> PouchDB;

    // #[wasm_bindgen(method, js_class = default)]
    #[wasm_bindgen(method)]
    pub fn close(this: &PouchDB) -> Promise;
}

#[wasm_bindgen]
pub fn get_version() -> String {
    "0.0.3-alpha".into()
}

#[wasm_bindgen]
pub fn new_db() {
    // let _db = PouchDB::new(String::from("Test"));
}
