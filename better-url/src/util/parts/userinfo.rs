//! Userinfo stuff.

use crate::prelude::*;

/// Make a [`Username`].
pub fn make_username<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    make_username_bytes(cow_str_to_bytes(value))
}

/// Make a [`Password`].
pub fn make_password<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    make_password_bytes(cow_str_to_bytes(value))
}

/// Make a [`Userinfo`].
pub fn make_userinfo<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>, Option<NonZero<usize>>) {
    make_userinfo_bytes(cow_str_to_bytes(value))
}



/// Make a [`Username`] from bytes.
pub fn make_username_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode_bytes(value, USERINFO)
}

/// Make a [`Password`] from bytes.
pub fn make_password_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode_bytes(value, USERINFO)
}

/// Make a [`Userinfo`] from bytes.
pub fn make_userinfo_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>, Option<NonZero<usize>>) {
    let mut value = value.into();

    let colon = match value.memchr(b':') {
        Some(i) if i + 1 == value.len() => {
            unsafe {
                value.truncate_unchecked(i);
            }

            None
        },
        x => x
    };

    let mut to_reserve = 0;

    for &b in &*value {
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

    let mut value = match value {
        Cow::Owned(value) if value.capacity() >= value.len() + to_reserve => value,
        value => {
            let mut ret = Vec::<u8>::with_capacity(value.len() + to_reserve);
            unsafe {
                std::ptr::copy(value.as_ptr(), ret.as_mut_ptr(), value.len());
                ret.set_len(value.len());
            }
            ret
        }
    };

    let mut i = value.len();
    let mut j = i + to_reserve;

    let mut password_start = None;

    unsafe {
        while i > 0 {
            i -= 1;
            j -= 1;

            match *value.get_unchecked(i) {
                b':' if Some(i) == colon => {
                    *value.as_mut_ptr().add(j) = b':';
                    password_start = NonZero::new(j + 1);
                },
                b if USERINFO.contains(b) => {
                    *value.as_mut_ptr().add(j - 2) = b'%';
                    *value.as_mut_ptr().add(j - 1) = NIBBLES[b as usize >> 4];
                    *value.as_mut_ptr().add(j    ) = NIBBLES[b as usize & 15];

                    j -= 2;
                },
                b => *value.as_mut_ptr().add(j) = b
            }
        }

        value.set_len(value.len() + to_reserve);
    }

    (true, unsafe {cow_bytes_to_str_unchecked(value)}, password_start)
}
