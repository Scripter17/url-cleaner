//! The context of URLs.

use serde::{Serialize, Deserialize};

use crate::types::*;

/// The context surrounding a URL.
/// 
/// Used by URL Cleaner Site and its userscript to provide details that can make cleaning faster.
/// 
/// For example, on twitter outlinks in tweets have an alt text that contains the entire destination URL that the t.co link points to.
/// 
/// This lets URL Cleaner avoid an entire HTTP request per tweet outlink.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UrlContext {
    /// String variables.
    pub vars: HashMap<String, String>
}
