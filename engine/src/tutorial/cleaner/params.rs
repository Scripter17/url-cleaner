//! # [`Params`]
//!
//! [`Params`] are configuration for [`Cleaner`]s for both choosing optional alternate features and for storing data like [sets](set), [maps](map), [named partitionings](named_partitioning) and so on.
//!
//! Params can be modified with [`ParamsDiff`]s for either the lifetime of a URL Cleaner Engine provider or for specific jobs.
//!
//! For example, URL Cleaner Site can
//!
//! - Be invoked with `--params-diff my_params_diff.json` to apply the params diff in the specified file to all jobs.
//!
//! - Be given another params diff by a user to adjust how a single job is handled, for example by enabling the `unmobile` flag for the default cleaner.
//!
//! Part of the params diff on my personal URL Cleaner Site instance is
//!
//! ```Json
//! {
//!   "flags": ["unmobile", "breezewiki"]
//! }
//! ```
//!
//! which makes all jobs have the `unmobile` and `breezewiki` flags enabled by default.
//!
//! In addition, my phone as a shortcut to send a clean request to my URL Cleaner Site instance in the form of
//!
//! ```Json
//! {
//!   "params_diff": {
//!     "flags": ["embed_compatibility"]
//!   },
//!   "tasks": ["the URL being cleaned"]
//! }
//! ```
//!
//! to use the `embed_compatibility` flag only when I need to.

pub(crate) use super::*;
