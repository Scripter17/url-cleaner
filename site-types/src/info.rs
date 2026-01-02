//! Types containing information about a URL Cleaner Site server.

use serde::{Serialize, Deserialize};

/// Info about a URL Cleaner Site server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Info {
    /// The link to the source code.
    pub source_code: String,
    /// The version.
    pub version: String,
    /// Whether or not you need a password to clean URLs.
    pub password_required: bool
}
