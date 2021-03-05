use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

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

pub async fn new_db() -> Result<String, JsValue> {
    let db = PouchDB::new(String::from("pouch-examples-yew-db"));
    JsFuture::from(db.close()).await?;
    Ok(String::from("Yeheww"))
}
