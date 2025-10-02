//! Types containing information about a URL Cleaner Site server.

use std::borrow::Cow;

use serde::{Serialize, Deserialize};

use url_cleaner_engine::types::*;

#[expect(unused_imports, reason = "Used in doc comments.")]
use crate::JobConfig;

/// Info about a URL Cleaner Site server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerInfo<'a> {
    /// The link to the source code.
    pub source_code: Cow<'a, BetterUrl>,
    /// The version.
    pub version: Cow<'a, str>,
    /// If logging is enabled.
    pub logging_enabled: bool,
    /// The max JSON size.
    pub max_json_size: u64,
    /// The default value for [`JobConfig::read_cache`].
    #[cfg(feature = "cache")]
    pub default_read_cache: bool,
    /// The default value for [`JobConfig::write_cache`].
    #[cfg(feature = "cache")]
    pub default_write_cache: bool,
    /// The default value for [`JobConfig::cache_delay`].
    #[cfg(feature = "cache")]
    pub default_cache_delay: bool,
    /// The default value for [`JobConfig::unthread`].
    pub default_unthread: bool,
    /// The [`UnthreaderMode`] used when unthreading.
    pub unthreader_mode: UnthreaderMode
}
