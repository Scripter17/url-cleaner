//! # [`Params`]
//!
//! [`Params`] are configuration for [`Cleaner`]s for both choosing optional alternate features and for storing data like [sets](set), [maps](map), [named partitionings](named_partitioning) and so on.
//!
//! Params can be modified with [`ParamsDiff`]s for either the lifetime of a URL Cleaner Engine provider or for specific jobs.
//!
//! For example, URL Cleaner Site can be invoked with `--params-diff my_params_diff.json` to apply it to all jobs and accept job configs with their own [`ParamsDiff`] to apply on top of it.
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
//! In addition, my phone has a shortcut to send a job config to my URL Cleaner Site instance in the form of
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
