//! The context of URLs.

use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::util::*;

/// The context surrounding a [`Job`]'s URL.
/// 
/// Used by URL Cleaner Site and its userscript to provide details that can make cleaning faster.
/// 
/// For example, on twitter outlinks in tweets have an alt text that contains the entire destination URL that the t.co link points to.
/// 
/// This lets URL Cleaner avoid an entire HTTP request per tweet outlink, which is extremely handy given some design issues with URL Cleaner Site.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobContext {
    /// String variables.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}
