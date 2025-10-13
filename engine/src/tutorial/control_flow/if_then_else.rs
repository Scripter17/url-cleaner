//! # If then else
//!
//! The most basic control flow construct is "if then else".
//!
//! For example the following action removes the "t" and "s" query parameters if and only if the URL has a [normalized host](BetterUrl::normalized_host) of "x.com".
//!
//! ```Json
//! {"If": {
//!   "if": {"NormalizedHostIs": "x.com"},
//!   "then": {"RemoveQueryParams": ["t", "s"]}
//! }}
//! ```
//!
//! As shown above, if the "else" branch is to do nothing, it can be omitted.
//!
//! The following action removes the query on every page of a website except for search results, where it allows only the "q" query parameter.
//!
//! ```Json
//! {"If": {
//!   "if": {"PathIs": "/search"},
//!   "then": {"AllowQueryParam": "q"},
//!   "else": "RemoveQuery"
//! }}
//! ```
//!
//! Sometimes, handling every case can get very verbose. For methods to briefly express common situations, see [maps] and [named partitionings](named_partitionings).

pub(crate) use super::*;
