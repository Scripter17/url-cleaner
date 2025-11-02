//! Types containing information about a URL Cleaner Site server.

use std::borrow::Cow;

use serde::{Serialize, Deserialize};

use url_cleaner_engine::prelude::*;

/// Info about a URL Cleaner Site server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerInfo<'a> {
    /// The link to the source code.
    pub source_code: Cow<'a, BetterUrl>,
    /// The version.
    pub version: Cow<'a, str>,
    /// The max payload size.
    pub max_payload: u64,
    /// The [`UnthreaderMode`] used when unthreading.
    pub unthreader_mode: UnthreaderMode
}
