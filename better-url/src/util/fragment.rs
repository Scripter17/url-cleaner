//! Fragment stuff.

use crate::prelude::*;

/// Turn a valid query into a fragment.
#[allow(clippy::indexing_slicing, reason = "Can't happen.")]
pub fn query_to_fragment(value: Cow<'_, str>) -> Cow<'_, str> {
    let mut value = cow_str_to_bytes(value);

    for i in (0..value.len()).rev() {
        if value[i] == b'`' {
            value.to_mut()[i] = b'%';
            value.to_mut().insert(i + 1, b'6');
            value.to_mut().insert(i + 2, b'0');
        }
    }

    // SAFETY: We added and removed only ASCII bytes.
    unsafe {cow_bytes_to_str(value)}
}
