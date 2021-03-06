use std::convert::TryInto;
use wasm_bindgen_futures::JsFuture;

mod js_pouchdb;
use js_pouchdb::bindings::PouchDB;

mod errors;
use errors::Error;

mod responses;
use responses::InfoResponse;

pub struct DB {
    pub js_db: PouchDB,
}

impl DB {
    pub fn new(name: String) -> DB {
        DB {
            js_db: PouchDB::new(name),
        }
    }

    pub async fn info(&self) -> Result<InfoResponse, Error> {
        JsFuture::from(self.js_db.info()).await?.try_into()
    }

    pub async fn close(&self) -> Result<(), Error> {
        JsFuture::from(self.js_db.close()).await?;
        Ok(())
    }

    pub fn _get_version(&self) -> &'static str {
        "0.0.4-alpha"
    }
}
