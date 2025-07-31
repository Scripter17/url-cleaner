//! # Named partitioning
//!
//! Named partitionings are conceptually and literally implemented as a map where most keys have one of a few values.
//!
//! The name comes from partitioning sets into mutually exclusive subsets and giving those partition names.
//!
//! For example, instead of writing
//!
//! ```Json
//! {"PartMap": {
//!   "part": "NormalizedHost",
//!   "map": {
//!     "fixupx.com"      : {"SetHost": "x.com"},
//!     "fixvx.com"       : {"SetHost": "x.com"},
//!     "fxtwitter.com"   : {"SetHost": "x.com"},
//!     "girlcockx.com"   : {"SetHost": "x.com"},
//!     "stupidpenisx.com": {"SetHost": "x.com"},
//!     "vxtwitter.com"   : {"SetHost": "x.com"},
//!     "yiffx.com"       : {"SetHost": "x.com"}
//!   }
//! }}
//! ```
//!
//! you can instead defined a [`NamedPartitioning`] like
//!
//! ```Json
//! "normalized_host_categories": {
//!   "twitter_embed_hosts": ["fixupx.com", "fixvx.com", "fxtwitter.com", "girlcockx.com", "stupidpenisx.com", "vxtwitter.com", "yiffx.com"]
//! }
//! ```
//!
//! then use [`Action::PartNamedPartitioning`] like
//!
//! ```Json
//! {"PartNamedPartitioning": {
//!   "named_partitioning": "normalized_host_categories",
//!   "part": "NormalizedHost",
//!   "map": {
//!     "twitter_embed_hosts": {"SetHost": "x.com"}
//!   }
//! }}
//! ```
//!
//! .

pub(crate) use super::*;
