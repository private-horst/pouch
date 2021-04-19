use std::convert::TryInto;
use wasm_bindgen_futures::JsFuture;

use crate::errors::Error;
use crate::pouchdb::bindings::PouchDB;
use crate::types::DatabaseInfo;
use crate::utils::log;

/// The Pouch database wraps a [PouchDB](https://pouchdb.com/) to store data locally in
/// the Browser or in Node, and to synchronize it optionally with a CouchDB server.
pub struct Database {
    /// the JavaScript PouchDB binding
    js_db: PouchDB,
}

impl Database {
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
    /// let db = pouch::Database::new("my_local_database");
    /// ```
    ///
    pub fn new(name: &str) -> Database {
        Database {
            js_db: PouchDB::new(String::from(name)),
        }
    }

    /// Returns infos to the database.
    ///
    /// # Examples
    ///
    /// ```
    /// // the async call returns am info response
    /// let info: DatabaseInfo = db.info().await?;
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
    pub async fn info(&self) -> Result<DatabaseInfo, Error> {
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
