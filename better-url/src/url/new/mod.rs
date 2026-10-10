//! Constructors.

mod bytes;
mod str;

use crate::prelude::*;

impl BetterUrl {
    /// Construct from bytes.
    pub(super) fn new_bytes(value: Cow<'_, [u8]>) -> Result<Self, InvalidUrl> {
        let (_, value) = canonize_parser_input_bytes(value);

        let i = value.memchr(b':').ok_or(InvalidUrl::MissingScheme)?;

        let scheme = unsafe {value.get_unchecked(.. i       )};
        let rest   = unsafe {value.get_unchecked(   i + 1 ..)};

        Self::after_scheme_bytes(Scheme::new(scheme)?, rest)
    }

    /// Construct from a string.
    pub(super) fn new_str(value: Cow<'_, str>) -> Result<Self, InvalidUrl> {
        let (_, value) = canonize_parser_input(value);

        let i = value.memchr(b':').ok_or(InvalidUrl::MissingScheme)?;

        let scheme = unsafe {value.get_unchecked(.. i       )};
        let rest   = unsafe {value.get_unchecked(   i + 1 ..)};

        Self::after_scheme_str(Scheme::new(scheme)?, rest)
    }
}
