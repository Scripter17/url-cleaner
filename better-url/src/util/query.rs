//! Query stuff.

use crate::prelude::*;

/// Turn a valid non-special query into a special query.
#[allow(clippy::indexing_slicing, reason = "Can't happen.")]
pub fn specialize_query(query: Cow<'_, str>) -> Cow<'_, str> {
    let mut query = cow_str_to_bytes(query);

    for i in (0..query.len()).rev() {
        if query[i] == b'\'' {
            query.to_mut()[i] = b'%';
            query.to_mut().insert(i + 1, b'2');
            query.to_mut().insert(i + 2, b'7');
        }
    }

    // SAFETY: We added and removed only ASCII bytes.
    unsafe {cow_bytes_to_str(query)}
}

/// Turn a valid fragment into a non-special query.
#[allow(clippy::indexing_slicing, reason = "Can't happen.")]
pub fn fragment_to_non_special_query(value: Cow<'_, str>) -> Cow<'_, str> {
    let mut value = cow_str_to_bytes(value);

    for i in (0..value.len()).rev() {
        if value[i] == b'#' {
            value.to_mut()[i] = b'%';
            value.to_mut().insert(i + 1, b'2');
            value.to_mut().insert(i + 2, b'3');
        }
    }

    // SAFETY: We added and removed only ASCII bytes.
    unsafe {cow_bytes_to_str(value)}
}

/// Turn a valid fragment into a special query.
#[allow(clippy::indexing_slicing, reason = "Can't happen.")]
pub fn fragment_to_special_query(value: Cow<'_, str>) -> Cow<'_, str> {
    let mut value = cow_str_to_bytes(value);

    for i in (0..value.len()).rev() {
        match value[i] {
            b'#'  => {let value = value.to_mut(); value[i] = b'%'; value.insert(i + 1, b'2'); value.insert(i + 2, b'3')},
            b'\'' => {let value = value.to_mut(); value[i] = b'%'; value.insert(i + 1, b'2'); value.insert(i + 2, b'7')},
            _ => {}
        }
    }

    // SAFETY: We added and removed only ASCII bytes.
    unsafe {cow_bytes_to_str(value)}
}
