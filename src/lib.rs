use js_sys::Promise;
use wasm_bindgen::prelude::*;

// TODO find a solution for browser and node
#[wasm_bindgen(module = "pouchdb")]
extern "C" {

    #[wasm_bindgen(js_name = default)]
    type PouchDB;

    #[wasm_bindgen(constructor, js_class = default)]
    pub fn new(name: String) -> PouchDB;

    #[wasm_bindgen(method, js_class = default)]
    pub fn close(this: &PouchDB) -> Promise;
}

pub fn get_version() -> String {
    "0.0.4-alpha".into()
}

pub fn new_db() -> String {
    let _db = PouchDB::new(String::from("pouch-examples-yew-db"));
    String::from("Yeheww")
}
