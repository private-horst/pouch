use serde::Deserialize;
use std::convert::TryFrom;
use wasm_bindgen::JsValue;

#[derive(Deserialize, Debug)]
pub struct DatabaseInfo {
    pub doc_count: i32,
    pub update_seq: i32,
    pub idb_attachment_format: String,
    pub db_name: String,
    pub auto_compaction: bool,
    pub adapter: String,
}

impl TryFrom<JsValue> for DatabaseInfo {
    type Error = crate::errors::Error;
    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let info: DatabaseInfo = value.into_serde().unwrap();
        Ok(info)
    }
}
