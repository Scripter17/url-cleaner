//! [`FilePath`].

use crate::prelude::*;

/// Encode a [`FilePath`].
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(encode_file_path(""            ), (true, "/"        .into()));
///
/// assert_eq!(encode_file_path("abc"         ), (true, "/abc"     .into()));
///
/// assert_eq!(encode_file_path("/abc/."      ), (true, "/abc/"    .into()));
/// assert_eq!(encode_file_path("/abc/.."     ), (true, "/"        .into()));
/// assert_eq!(encode_file_path("/abc/./ghi/" ), (true, "/abc/ghi/".into()));
/// assert_eq!(encode_file_path("/abc/../ghi/"), (true, "/ghi/"    .into()));
///
/// assert_eq!(encode_file_path("/c:/."       ), (true, "/c:/"     .into()));
/// assert_eq!(encode_file_path("/c:/.."      ), (true, "/c:/"     .into()));
/// assert_eq!(encode_file_path("/c:/./ghi/"  ), (true, "/c:/ghi/" .into()));
/// assert_eq!(encode_file_path("/c:/../ghi/" ), (true, "/c:/ghi/" .into()));
///
/// assert_eq!(encode_file_path("/c|/."       ), (true, "/c:/"     .into()));
/// assert_eq!(encode_file_path("/c|/.."      ), (true, "/c:/"     .into()));
/// assert_eq!(encode_file_path("/c|/./ghi/"  ), (true, "/c:/ghi/" .into()));
/// assert_eq!(encode_file_path("/c|/../ghi/" ), (true, "/c:/ghi/" .into()));
/// ```
pub fn encode_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    encode_file_path_bytes(cow_str_to_bytes(value))
}



/// Encode a [`FilePath`] from bytes.
pub fn encode_file_path_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
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

    let (needed_resolve, value) = resolve_file_path(value);

    (prepend_slash || to_encode != 0 || forwarded_slashes || needed_resolve, value)
}



/// [`resolve_non_special_path_range`] with the full range.
pub fn resolve_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    resolve_file_path_range(value, ..)
}

/// Resolve an encoded file path using only the segments in the range of bytes.
/// # Panics
/// May or may not panic if the range does not begin with a `/` and/or does not end after the end of a segment.
pub fn resolve_file_path_range<'a, T: Into<Cow<'a, str>>, B: RangeBounds<usize>>(value: T, range: B) -> (bool, Cow<'a, str>) {
    let mut value = cow_str_to_bytes(value);
    let mut changed = false;

    let start = match range.start_bound() {
        Bound::Unbounded    => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
    };

    let mut after = match range.end_bound() {
        Bound::Unbounded    => value.len(),
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
    };

    assert!(start <= after && after <= value.len());

    debug_assert!(start == value.len() || value[start] == b'/');
    debug_assert!(after == value.len() || value[after] == b'/');

    let mut i = start;

    if i == 0 && matches!(&*value, [b'/', x, b'|'] | [b'/', x, b'|', b'/', ..] if x.is_ascii_alphabetic()) {
        value.to_mut()[2] = b':';
        changed = true;
    }

    while i < after {
        let left = unsafe {value.get_unchecked(..i)};
        let rest = unsafe {value.get_unchecked(i + 1..)};

        debug_assert_eq!(value[i], b'/');
        debug_assert!(after == value.len() || value[after] == b'/');

        if let Some(x) = munch_single_dot_segment(rest) {
            changed = true;

            if x.is_empty() {
                unsafe {
                    value.truncate_unchecked(i + 1);
                }
                break;
            } else {
                let l = rest.len() + 1 - x.len();

                value.to_mut().drain(i .. i + l);

                after -= l;
            }
        } else if let Some(x) = munch_double_dot_segment(rest) {
            changed = true;

            let j = match left {
                [b'/', b'a'..=b'z' | b'A'..=b'Z', b':'] => i,
                _                                       => left.memrchr(b'/').unwrap_or(0),
            };

            if x.is_empty() {
                unsafe {
                    value.truncate_unchecked(j + 1);
                }
                break;
            } else {
                let l = rest.len() + 1 - x.len();

                value.to_mut().drain(j .. i + l);

                after -= i + l - j;
            }

            i = j;
        } else if let Some(j) = unsafe {value.get_unchecked(i + 1 .. after)}.memchr(b'/') {
            i += j + 1;
        } else {
            break;
        }
    }

    (changed, unsafe {cow_bytes_to_str_unchecked(value)})
}



/// Convert a [`SpecialNotFilePath`] into a [`FilePath`].
pub fn special_not_file_path_to_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();

    match value.as_bytes() {
        [b'/', x, b'|'] | [b'/', x, b'|', b'/', ..] if x.is_ascii_alphabetic() => {
            unsafe {
                *value.to_mut().as_mut_vec().get_unchecked_mut(2) = b':';
            }
            (true, value)
        },
        _ => (false, value)
    }
}

/// Convert a [`NonSpecialPath`] into a [`FilePath`].
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(non_special_path_to_file_path("/abc/def"     ), (false, "/abc/def".into()));
/// assert_eq!(non_special_path_to_file_path("/abc\\def"    ), (true , "/abc/def".into()));
/// assert_eq!(non_special_path_to_file_path("/abc\\.\\def" ), (true , "/abc/def".into()));
/// assert_eq!(non_special_path_to_file_path("/abc\\..\\def"), (true , "/def"    .into()));
/// assert_eq!(non_special_path_to_file_path("/c:\\..\\def" ), (true , "/c:/def" .into()));
/// ```
pub fn non_special_path_to_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (had_backslashes, mut value) = forward_slashes(value);

    match value.as_bytes() {
        [] => (true, "/".into()),
        [b'/', x, b'|'] | [b'/', x, b'|', b'/', ..] if x.is_ascii_alphabetic() => {
            unsafe {
                *value.to_mut().as_mut_vec().get_unchecked_mut(2) = b':';
            }

            if had_backslashes {
                value = resolve_file_path(value).1;
            }

            (true, value)
        },
        _ => match had_backslashes {
            true  => (true, resolve_file_path(value).1),
            false => (false, value)
        }
    }
}

/// Convert an [`OpaquePath`] into a [`FilePath`].
pub fn opaque_path_to_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let value = value.into().with_insert_str(0, "/");

    let (_, value) = forward_slashes(value);

    let (_, value) = resolve_file_path(value);

    (true, value)
}
