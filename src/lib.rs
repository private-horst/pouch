#![doc(html_favicon_url = "https://www.pouch.rs/assets/images/favicon.ico")]
#![doc(html_logo_url = "https://www.pouch.rs/assets/images/logo.svg")]

/// The pouch database
mod database;
pub use crate::database::Database;

/// Types used with the database
pub mod types;

/// Pouch specific error types
pub mod errors;

/// Bindings to the PouchDB module
mod js_pouchdb;

/// Utils like logging
mod utils;

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

/// The Pouch prelude
///
/// The purpose of this module is to alleviate imports of many common types:
///
/// ```
/// # #![allow(unused_imports)]
/// use yew::pouch::*;
/// ```
pub mod prelude {
    pub use crate::database::Database;
    pub use crate::errors::Error;
    pub use crate::types::DatabaseInfo;
}

pub use self::prelude::*;
