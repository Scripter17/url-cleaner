//! Entire paths.

mod file;
mod special_not_file;
mod non_special;
mod opaque;

pub use file::*;
pub use special_not_file::*;
pub use non_special::*;
pub use opaque::*;

/// Munch a single dot segment from the start of `x`.
pub(crate) fn munch_single_dot_segment(x: &[u8]) -> Option<&[u8]> {
    match munch_single_dot(x)? {
        x @ ([b'/', ..] | []) => Some(x),
        _ => None
    }
}

/// Munch a double dot segment from the start of `x`.
pub(crate) fn munch_double_dot_segment(x: &[u8]) -> Option<&[u8]> {
    match munch_double_dot(x)? {
        x @ ([b'/', ..] | []) => Some(x),
        _ => None
    }
}

/// [`munch_single_dot`] twice.
pub(crate) fn munch_double_dot(x: &[u8]) -> Option<&[u8]> {
    munch_single_dot(munch_single_dot(x)?)
}

/// Munch a `.`, `%2e`, or `%2E` from the start of `x`.
pub(crate) fn munch_single_dot(x: &[u8]) -> Option<&[u8]> {
    match x {
        [b'.', x @ ..] | [b'%', b'2', b'e' | b'E', x @ ..] => Some(x),
        _ => None
    }
}
