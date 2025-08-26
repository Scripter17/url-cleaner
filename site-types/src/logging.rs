//! Optional logging stuff.

use std::borrow::Cow;

use serde::{Serialize, Deserialize};

use crate::*;

/// The format for logging [`JobConfig`]s and their results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobLog<'a> {
    /// The error variant for when the string isn't a valid [`JobConfig`].
    Err {
        /// The string that should've been but wasn't a valid [`JobConfig`].
        job_config: Cow<'a, str>,
        /// The [`CleanError`].
        result: Cow<'a, CleanError>
    },
    /// The ok variant for when the string is a valid [`JobConfig`].
    Ok {
        /// The [`JobConfig`].
        job_config: Box<JobConfig<'a>>,
        /// The [`CleanResult`].
        result: Cow<'a, CleanSuccess>
    }
}
