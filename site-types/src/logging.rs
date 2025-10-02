//! Optional logging stuff.

use std::borrow::Cow;

use serde::{Serialize, Deserialize};

use crate::*;

/// The format for logging [`CleanPayload`]s and their results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobLog<'a> {
    /// The error variant for when the string isn't a valid [`CleanPayload`].
    Err {
        /// The string that should've been but wasn't a valid [`CleanPayload`].
        clean_payload: Cow<'a, str>,
        /// The [`CleanError`].
        result: Cow<'a, CleanError>
    },
    /// The ok variant for when the string is a valid [`CleanPayload`].
    Ok {
        /// The [`CleanPayload`].
        clean_payload: Box<CleanPayload<'a>>,
        /// The [`CleanResult`].
        result: Cow<'a, CleanSuccess>
    }
}
