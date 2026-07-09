//! [`Path`] and co.

use crate::prelude::*;

/// Canonize the input for the pathname setter to a form parsable by the various [`Path`] types.
pub fn canonize_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    canonize_part_setter(value)
}
