//! [`NonSpecialPathSegments`].

use crate::prelude::*;

mod iter;
pub use iter::*;

/// Encode a [`NonSpecialPathSegments`].
pub fn encode_non_special_path_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode(value, PATH)
}
