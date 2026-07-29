//! [`SpecialNotFilePathSegments`].

use crate::prelude::*;

mod iter;
pub use iter::*;

/// Encode a [`SpecialNotFilePathSegments`].
pub fn encode_special_not_file_path_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (a, value) = percent_encode (value, PATH);
    let (b, value) = forward_slashes(value      );

    (a || b, value)
}

/// Replace `\\` with `/`.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(forward_slashes("///"   ), (false, "///".into()));
/// assert_eq!(forward_slashes("//\\"  ), (true , "///".into()));
/// assert_eq!(forward_slashes("/\\/"  ), (true , "///".into()));
/// assert_eq!(forward_slashes("/\\\\" ), (true , "///".into()));
/// assert_eq!(forward_slashes("\\//"  ), (true , "///".into()));
/// assert_eq!(forward_slashes("\\/\\" ), (true , "///".into()));
/// assert_eq!(forward_slashes("\\\\\\"), (true , "///".into()));
/// ```
pub fn forward_slashes<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();

    match value.memchr(b'\\') {
        Some(mut i) => unsafe {
            let x = value.to_mut().as_mut_vec();

            *x.get_unchecked_mut(i) = b'/';

            while let Some(j) = x.get_unchecked(i + 1..).memchr(b'\\') {
                i += j + 1;

                *x.get_unchecked_mut(i) = b'/';
            }

            (true, value)
        },
        None => (false, value)
    }
}
