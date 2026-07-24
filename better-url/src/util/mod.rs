//! General utility functions.

mod cow_bytes_str;
mod split;
mod parts;
mod percent;
mod ascii_set;
mod normalizer;
mod macros;
mod ext_traits;

pub use cow_bytes_str::*;
pub use split::*;
pub use parts::*;
pub use percent::*;
pub use ascii_set::*;
pub use normalizer::*;

pub(crate) use macros::*;
pub(crate) use ext_traits::*;

use std::borrow::Cow;

/// Join a [`crate::prelude::FilePath`] with an absolute path.
pub(crate) fn file_path_join_abs<'a>(old: &str, join: &'a str) -> Cow<'a, str> {
    match join.as_bytes() {
        [b'/', b'a'..=b'z' | b'A'..=b'Z', b':' | b'|', b'/', ..] | [b'/', b'a'..=b'z' | b'A'..=b'Z', b':' | b'|'] => join.into(),
        _ => match old.as_bytes() {
            [b'/', b'a'..=b'z' | b'A'..=b'Z', b':', b'/', ..] | [b'/', b'a'..=b'z' | b'A'..=b'Z', b':'] => format!("{}{join}", &old[..3]).into(),
            _ => join.into()
        }
    }
}

/// Join a [`crate::prelude::FilePath`] with a relative path.
pub(crate) fn file_path_join_rel<'a>(old: &str, join: &'a str) -> Option<Cow<'a, str>> {
    match join.as_bytes() {
        [] => None,
        [b'a'..=b'z' | b'A'..=b'Z', b':' | b'|', b'/' | b'\\', ..] | [b'a'..=b'z' | b'A'..=b'Z', b':' | b'|'] => Some(join.into()),
        _ => {
            let (x, _) = old.rsplit_once('/').unwrap_or_default();
            Some(format!("{x}/{join}").into())
        }
    }
}

/// Join a [`crate::prelude::SpecialNotFilePath`]/[`crate::prelude::NonSpecialPath`] with a relative path.
pub(crate) fn non_file_path_join_rel<'a>(old: &str, join: &'a str) -> Option<Cow<'a, str>> {
    if join.is_empty() {
        None
    } else {
        let (x, _) = old.rsplit_once('/').unwrap_or_default();
        Some(format!("{x}/{join}").into())
    }
}
