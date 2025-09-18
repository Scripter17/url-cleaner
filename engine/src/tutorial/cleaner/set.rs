//! [`Set`]
//!
//! A [`Set`] is an optimization over [`HashSet`]s that allows you to check if an `Option<&str>` is in the equivalent of a `HashSet<Option<String>>`.
//!
//! Internally this works by having a `HashSet<String>` for `Some("...")` values and a [`bool`] that's [`true`] if the equivalent `HashSet<Option<String>>` has [`None`].

pub(crate) use super::*;
