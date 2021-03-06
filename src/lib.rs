mod js_pouchdb;

use js_pouchdb::bindings::PouchDB;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub fn get_version() -> String {
    "0.0.4-alpha".into()
}

pub async fn new_db() -> Result<String, JsValue> {
    let db = PouchDB::new(String::from("pouch-examples-yew-db"));
    JsFuture::from(db.close()).await?;
    Ok(String::from("Yeheww"))
}
