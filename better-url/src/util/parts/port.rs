//! Port stuff.

use crate::prelude::*;

/// Parse a maybe port.
/// # Errors
/// If the call to [`parse_port`] returns an error, that error is returned.
pub fn parse_maybe_port(s: Option<&str>) -> Result<Option<u16>, InvalidPort> {
    Ok(match s {
        Some(s) => Some(parse_port(s)?),
        None    => None
    })
}

/// Parse a port.
/// # Errors
/// If the call to [`u16::from_str`] returns an error, returns the error [`InvalidPort`].
pub fn parse_port(s: &str) -> Result<u16, InvalidPort> {
    s.parse().map_err(|_| InvalidPort)
}
