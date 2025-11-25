//! Types containing information about a URL Cleaner Site server.

use serde::{Serialize, Deserialize};

/// Info about a URL Cleaner Site server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerInfo {
    /// The link to the source code.
    pub source_code: String,
    /// The version.
    pub version: String,
    /// The max payload size.
    pub max_payload: u64
}
