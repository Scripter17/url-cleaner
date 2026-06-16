//! Userinfo stuff.

use thiserror::Error;

use crate::prelude::*;

/// The errors that can happen when setting a userinfo.
#[derive(Debug, Error)]
pub enum SetUserinfoError {
    /** [`TooLong`].**/  #[error(transparent)] TooLong(#[from] TooLong),
    /** [`NoHost`]. **/  #[error(transparent)] NoHost (#[from] NoHost ),
}

/// The errors that can happen when setting a username.
#[derive(Debug, Error)]
pub enum SetUsernameError {
    /** [`TooLong`].**/  #[error(transparent)] TooLong(#[from] TooLong),
    /** [`NoHost`]. **/  #[error(transparent)] NoHost (#[from] NoHost ),
}

/// The errors that can happen when setting a password.
#[derive(Debug, Error)]
pub enum SetPasswordError {
    /** [`TooLong`].**/  #[error(transparent)] TooLong(#[from] TooLong),
    /** [`NoHost`]. **/  #[error(transparent)] NoHost (#[from] NoHost ),
}
