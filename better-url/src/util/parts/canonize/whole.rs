//! The whole URL.

use crate::prelude::*;

/// Canonize the input to a URL parser.
///
/// Specifically, remove every leading and trailing codepoint in the range `0..=32` as well as all tabs, newlines, and carriage returns.
pub fn canonize_whole_setter<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    let mut x = value.bytes();

    let a = x. position(|b| b > 0x20).unwrap_or(0);
    let b = x.rposition(|b| b > 0x20).unwrap_or(0);
    let c = x.any(|b| b == b'\t' || b == b'\n' || b == b'\r');

    if a != 0 || b != value.len() {
        changed = true;
        value.retain_range(a..b);
    }

    if c {
        unsafe {
            value.to_mut().as_mut_vec().retain(|&b| b != b'\t' && b != b'\n' && b != b'\r');
        }
        changed = true;
    }

    (changed, value)
}
