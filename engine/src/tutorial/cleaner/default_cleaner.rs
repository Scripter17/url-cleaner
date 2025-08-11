//! # The default cleaner
//!
//! The default cleaner is the [`Cleaner`] included by default in URL Cleaner Engine. It's meant to be a versatile general purpose cleaner that does everything I've ever needed it to do.
//!
//! The default cleaner can be omitted at compile time by disabling the `default-cleaner` feature.
//!
//! You can view the default cleaner as text at [`DEFAULT_CLEANER_STR`] and use it with [`Cleaner::get_default`].

pub(crate) use super::*;
