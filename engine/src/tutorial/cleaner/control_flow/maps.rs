//! # Maps
//!
//! [`Map`]s are key-value pairs with two extra properies:
//!
//! 1. The [`None`]/`null` key is written outside the map in the `"if_none"` field.
//!
//! 2. Maps have a fallback value, called `else`, that is returned when using a key not otherwise in the map.
//!
//! Components that use maps "flatten" them, so while [`Action::PartMap`] contains a `map` field of type [`Map`], you would use it as follows:
//!
//! ```Json
//! {"PartMap": {
//!   "part": "NormalizedHost",
//!   "map": {
//!     "example.com": ...
//!   },
//!   "if_none": ...,
//!   "else": ...
//! }}
//! ```

pub(crate) use super::*;
