use js_sys::Promise;
use wasm_bindgen::prelude::*;

// TODO find a solution for browser and node
#[wasm_bindgen(module = "pouchdb")]
extern "C" {

    //#[wasm_bindgen] // works neither in browser nor in node?
    #[wasm_bindgen(js_name = default)] // works in browser with es6
    pub type PouchDB;

    //#[wasm_bindgen(constructor)]
    #[wasm_bindgen(constructor, js_class = default)] // works in browser with es6
    pub fn new(name: String) -> PouchDB;

    //#[wasm_bindgen(method)]
    #[wasm_bindgen(method, js_class = default)] // works in browser with es6
    pub fn info(this: &PouchDB) -> Promise;

    //#[wasm_bindgen(method)]
    #[wasm_bindgen(method, js_class = default)] // works in browser with es6
    pub fn put(this: &PouchDB, doc: JsValue) -> Promise;

    //#[wasm_bindgen(method)]
    #[wasm_bindgen(method, js_class = default)] // works in browser with es6
    pub fn get(this: &PouchDB, docId: JsValue) -> Promise;

    //#[wasm_bindgen(method)]
    #[wasm_bindgen(method, js_class = default)] // works in browser with es6
    pub fn close(this: &PouchDB) -> Promise;

}
