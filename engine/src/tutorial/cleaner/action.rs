//! # [`Action`]
//!
//! Actions take a task state and modify it. For most actions this is modifying part of the task's URL, sometimes with a logic to decide if and when to do so.
//!
//! For example the [`RemoveQueryParams`](Action::RemoveQueryParams) removes all query params whose names are in the [`Set`].
//!
//! ```Json
//! {"RemoveQueryParams": ["utm_source", "utm_content", "etc."]}
//! ```
//!
//! Actions have various forms of [control flow](control_flow) that let you only apply modifications if certain conditions are met.
//!
//! For example, the following only removes the `t` and `s` query parameters on `x.com`:
//!
//! ```Json
//! {"If": {
//!   "if": {"HostIs": "x.com"},
//!   "then": {"RemoveQueryParams": ["t", "s"]}
//! }}
//! ```
//!
//! [`Action::If`] allows an optional "else" action that's only applied when the condition isn't satisfied.
//!
//! ```Json
//! {"If": {
//!   "if": {"HostIs": "x.com"},
//!   "then": {"RemoveQueryParams": ["t", "s"]},
//!   "else": "something that applies everywhere but twitter"
//! }}
//! ```
//!
//! Writing a long chain/sequence of [`Action::If`]s is both slow and ugly, so [maps] can be used to, as I call it, "do less nothing":
//!
//! ```Json
//! {"PartMap": {
//!   "part": "Host",
//!   "map": {
//!     "x.com": {"RemoveQueryParams": ["t", "s"]},
//!     "youtube.com": {"RemoveQueryParams": "si"}
//!   }
//! }}
//! ```
//!
//! While, technically, an [`Action::PartMap`] is more expensive than a single [`Action::If`], the cost is only paid once.
//!
//! Having a 100-variant [`Action::PartMap`] is comically faster than an equivalent chain/sequence of 100 [`Action::If`]s.i
//!
//! In addition to [`Action::PartMap`], there is also [`Action::StringMap`], which uses [string sources](string_source) instead of [URL parts](url_part).

pub(crate) use super::*;
