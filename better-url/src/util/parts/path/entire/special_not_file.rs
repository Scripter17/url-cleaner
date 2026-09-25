//! [`SpecialNotFilePath`].

use crate::prelude::*;

/// Encode a [`SpecialNotFilePath`].
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(encode_special_not_file_path(""            ), (true, "/"        .into()));
///
/// assert_eq!(encode_special_not_file_path("abc"         ), (true, "/abc"     .into()));
///
/// assert_eq!(encode_special_not_file_path("/abc/."      ), (true, "/abc/"    .into()));
/// assert_eq!(encode_special_not_file_path("/abc/.."     ), (true, "/"        .into()));
/// assert_eq!(encode_special_not_file_path("/abc/./ghi/" ), (true, "/abc/ghi/".into()));
/// assert_eq!(encode_special_not_file_path("/abc/../ghi/"), (true, "/ghi/"    .into()));
///
/// assert_eq!(encode_special_not_file_path("/."          ), (true, "/"        .into()));
/// assert_eq!(encode_special_not_file_path("/.."         ), (true, "/"        .into()));
/// assert_eq!(encode_special_not_file_path("/./ghi/"     ), (true, "/ghi/"    .into()));
/// assert_eq!(encode_special_not_file_path("/../ghi/"    ), (true, "/ghi/"    .into()));
///
/// assert_eq!(encode_special_not_file_path("/c:/."       ), (true, "/c:/"     .into()));
/// assert_eq!(encode_special_not_file_path("/c:/.."      ), (true, "/"        .into()));
/// assert_eq!(encode_special_not_file_path("/c:/./ghi/"  ), (true, "/c:/ghi/" .into()));
/// assert_eq!(encode_special_not_file_path("/c:/../ghi/" ), (true, "/ghi/"    .into()));
/// ```
pub fn encode_special_not_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    encode_special_not_file_path_bytes(cow_str_to_bytes(value))
}



/// Encode a [`SpecialNotFilePath`] from bytes.
pub fn encode_special_not_file_path_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
    let value = value.into();

    let prepend_slash = !matches!(&*value, [b'/' | b'\\', ..]);
    let mut to_encode = 0;

    for &b in &*value {
        if PATH.contains(b) {
            to_encode += 1;
        }
    }



    let value = match to_encode {
        0 => match prepend_slash {
            true  => unsafe {cow_bytes_to_str_unchecked(value)}.with_insert_str(0, "/"),
            false => unsafe {cow_bytes_to_str_unchecked(value)},
        },
        _ => {
            let len = value.len() + to_encode * 2 + prepend_slash as usize;

            let mut ret = String::with_capacity(len);

            if prepend_slash {
                unsafe {
                    *ret.as_mut_ptr() = b'/';
                }
            }

            let mut w = prepend_slash as usize;

            unsafe {
                for &b in &*value {
                    if PATH.contains(b) {
                        *ret.as_mut_ptr().add(w    ) = b'%';
                        *ret.as_mut_ptr().add(w + 1) = NIBBLES[b as usize >> 4];
                        *ret.as_mut_ptr().add(w + 2) = NIBBLES[b as usize & 15];

                        w += 3;
                    } else {
                        *ret.as_mut_ptr().add(w) = b;

                        w += 1;
                    }
                }

                ret.as_mut_vec().set_len(len);
            }

            ret.into()
        }
    };

    let (forwarded_slashes, value) = forward_slashes(value);

    let (needed_resolve, value) = resolve_special_not_file_path(value);

    (prepend_slash || to_encode != 0 || forwarded_slashes || needed_resolve, value)
}



/// Convert a [`NonSpecialPath`] into a [`SpecialNotFilePath`].
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(non_special_path_to_special_not_file_path("/abc/def"     ), (false, "/abc/def".into()));
/// assert_eq!(non_special_path_to_special_not_file_path("/abc\\def"    ), (true , "/abc/def".into()));
/// assert_eq!(non_special_path_to_special_not_file_path("/abc\\.\\def" ), (true , "/abc/def".into()));
/// assert_eq!(non_special_path_to_special_not_file_path("/abc\\..\\def"), (true , "/def"    .into()));
/// assert_eq!(non_special_path_to_special_not_file_path("/c:\\..\\def" ), (true , "/def"    .into()));
/// ```
pub fn non_special_path_to_special_not_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    match forward_slashes(value) {
        (true , value) => (true , resolve_special_not_file_path(value).1),
        (false, value) => (false, value),
    }
}



/// [`resolve_non_special_path_range`] with the full range.
pub fn resolve_special_not_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    resolve_special_not_file_path_range(value, ..)
}

/// Resolve an encoded special not file path using only the segments in the range of bytes.
/// # Panics
/// May or may not panic if the range does not begin with a `/` and/or does not end after the end of a segment.
pub fn resolve_special_not_file_path_range<'a, T: Into<Cow<'a, str>>, B: RangeBounds<usize>>(value: T, range: B) -> (bool, Cow<'a, str>) {
    resolve_non_special_path_range(value, range)
}



/// Convert an [`OpaquePath`] into a [`SpecialNotFilePath`].
pub fn opaque_path_to_special_not_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let value = value.into().with_insert_str(0, "/");

    let (_, value) = forward_slashes(value);

    let (_, value) = resolve_special_not_file_path(value);

    (true, value)
}
