use serde::Deserialize;
use serde_json::Value;
use std::convert::TryFrom;
use wasm_bindgen::JsValue;

use crate::errors::Error;

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

#[derive(Deserialize, Debug)]
pub struct Document<T> {
    pub _id: String,  // TODO make optional
    pub _rev: String, // TODO make optional
    pub data: T,
}

impl<T> TryFrom<JsValue> for Document<T> {
    type Error = crate::errors::Error;
    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let _raw_doc: Value = value.into_serde().unwrap();
        // TODO convert data into document type
        // let data: T = serde_json::from_value(value).unwrap();
        Err(Error::Pouch("Not implemented yet"))
    }
}
