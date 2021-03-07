use std::convert::TryInto;
use wasm_bindgen_futures::JsFuture;

mod utils;
use utils::log;

mod js_pouchdb;
use js_pouchdb::bindings::PouchDB;

mod errors;
pub use errors::Error;

mod responses;
pub use responses::InfoResponse;

/// Returns the crate version
///
/// # Examples
///
/// ```
/// // the version is returned as &str
/// let version = pouch::version();
/// ```
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// The Pouch database wraps a [PouchDB](https://pouchdb.com/) database to store data locally in
/// the Browser or in Node, and to synchronize it optionally with a CouchDB based server.
pub struct DB {
    /// the JavaScript PouchDB binding
    js_db: PouchDB,
}

impl DB {
    /// Creates a new database with the given name. If there is already one with this name
    /// available, this is returned.
    ///
    /// # Arguments
    ///
    /// * `name` - A String with the name of the database
    ///
    /// # Examples
    ///
    /// ```
    /// // create or open a database with the given name
    /// let db = pouch::DB::new("my_local_database");
    /// ```
    ///
    pub fn new(name: &str) -> DB {
        DB {
            js_db: PouchDB::new(String::from(name)),
        }
    }

    /// Returns infos to the database.
    ///
    /// # Examples
    ///
    /// ```
    /// // the async call returns am info response
    /// let info: InfoResponse = db.info().await?;
    ///
    /// // the info response holds the database name as String
    /// let db_name = info.name;
    ///
    /// // the number of stored documents
    /// let doc_count = info.doc_count;
    ///
    /// // the number of updates done on documents
    /// let update_seq = info.update_seq;
    /// ```
    ///
    pub async fn info(&self) -> Result<InfoResponse, Error> {
        log("Pouch: getting database info");
        JsFuture::from(self.js_db.info()).await?.try_into()
    }

    /// Closes the connection to the database. All method calls after this are quitted with an
    /// error.
    ///
    /// # Examples
    ///
    /// ```
    /// // the async call returns a positive result
    /// // if the connection can be closed
    /// db.close().await?;
    /// ```
    ///
    pub async fn close(&self) -> Result<(), Error> {
        JsFuture::from(self.js_db.close()).await?;
        Ok(())
    }
}
