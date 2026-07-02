//! Userinfo stuff.

use crate::prelude::*;

/// Encode a [`Username`].
pub fn encode_username<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode::<'_, _, false, false, false>(cow_str_to_bytes(value.into()), USERINFO)
}

/// Encode a [`Password`].
pub fn encode_password<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode::<'_, _, false, false, false>(cow_str_to_bytes(value.into()), USERINFO)
}

/// Encode a [`Userinfo`].
pub fn encode_userinfo<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>, Option<NonZero<usize>>) {
    let (changed, mut value) = percent_encode::<'_, _, false, false, true>(cow_str_to_bytes(value.into()), USERINFO);

    match value.strip_suffix(':') {
        Some(x) => {
            value.retain_substr(x);
            (true, value, None)
        },
        None => {
            let ps = value.find(":").and_then(|x| NonZero::new(x + 1));
            (changed, value, ps)
        }
    }
}
