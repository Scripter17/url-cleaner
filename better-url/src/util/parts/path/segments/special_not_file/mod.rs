//! [`SpecialNotFilePathSegments`].

use crate::prelude::*;

mod iter;
pub use iter::*;

/// Encode a [`SpecialNotFilePathSegments`].
pub fn encode_special_not_file_path_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (a, value) = forward_slashes(value      );
    let (b, value) = percent_encode (value, PATH);

    (a || b, value)
}

/// Replace `\\` with `/`.
pub fn forward_slashes<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value   = value.into();
    let mut changed = false;

    let mut i = 0;

    unsafe {
        while let Some(j) = value.get_unchecked(i..).memchr(b'\\') {
            changed = true;

            *value.to_mut().as_mut_vec().get_unchecked_mut(i + j) = b'/';

            i += j + 1;
        }
    }

    (changed, value)
}
