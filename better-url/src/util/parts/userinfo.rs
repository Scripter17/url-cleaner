//! Userinfo stuff.

use crate::prelude::*;

/// Encode a [`Username`].
pub fn encode_username<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode(value, USERINFO)
}

/// Encode a [`Password`].
pub fn encode_password<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode(value, USERINFO)
}

/// Encode a [`Userinfo`].
pub fn encode_userinfo<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>, Option<NonZero<usize>>) {
    let mut value = cow_str_to_bytes(value);

    let mut colon = value.memchr(b':');

    if let Some(i) = colon && i + 1 == value.len() {
        unsafe {
            value.truncate_unchecked(i);
        }
        colon = None;
    }

    let mut to_reserve = 0;

    for &b in value.iter() {
        if USERINFO.contains(b) {
            to_reserve += 2;
        }
    }

    if colon.is_some() {
        to_reserve -= 2;
    }

    if to_reserve == 0 {
        return (false, unsafe {cow_bytes_to_str_unchecked(value)}, colon.and_then(|x| NonZero::new(x + 1)));
    }

    // TODO: If `value` is [`Cow::Owned`] and has enough capacity, use that.

    let mut ret = Vec::<u8>::with_capacity(value.len() + to_reserve);

    let mut i = value.len();
    let mut j = i + to_reserve;

    let mut password_start = None;

    unsafe {
        while i > 0 {
            i -= 1;
            j -= 1;

            match *value.get_unchecked(i) {
                b':' if Some(i) == colon => {
                    *ret.as_mut_ptr().add(j) = b':';
                    password_start = NonZero::new(j + 1);
                },
                b if USERINFO.contains(b) => {
                    *ret.as_mut_ptr().add(j - 2) = b'%';
                    *ret.as_mut_ptr().add(j - 1) = NIBBLES[b as usize >> 4];
                    *ret.as_mut_ptr().add(j    ) = NIBBLES[b as usize & 15];

                    j -= 2;
                },
                b => *ret.as_mut_ptr().add(j) = b
            }
        }

        ret.set_len(value.len() + to_reserve);
    }

    (true, unsafe {cow_bytes_to_str_unchecked(ret)}, password_start)
}

/// [`lossy_percent_decode`] but with clearer intent.
pub fn lossy_decode_username<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    lossy_percent_decode(value)
}

/// [`lossy_percent_decode`] but with clearer intent.
pub fn lossy_decode_password<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    lossy_percent_decode(value)
}

/// [`try_percent_decode`] but with clearer intent.
/// # Errors
/// If the call to [`try_percent_decode`] returns an error, that error is returned.
#[expect(clippy::type_complexity, reason = "It's fine.")]
pub fn try_decode_username<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), (std::str::Utf8Error, Cow<'a, [u8]>)> {
    try_percent_decode(value)
}

/// [`try_percent_decode`] but with clearer intent.
/// # Errors
/// If the call to [`try_percent_decode`] returns an error, that error is returned.
#[expect(clippy::type_complexity, reason = "It's fine.")]
pub fn try_decode_password<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), (std::str::Utf8Error, Cow<'a, [u8]>)> {
    try_percent_decode(value)
}
