//! [`Scheme`].

use crate::prelude::*;

/// A scheme.
#[derive(Debug, Clone)]
pub struct Scheme<'a> {
    /// The scheme.
    pub(crate) scheme: Cow<'a, str>,
    /// The [`SchemeDetails`].
    pub(crate) details: SchemeDetails,
}

impl<'a> Scheme<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.scheme
    }

    /// The [`SchemeDetails`].
    pub fn details(&self) -> SchemeDetails {
        self.details
    }

    /// The [`SchemeType`].
    pub fn r#type(&self) -> SchemeType {
        self.details.r#type()
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.scheme
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Scheme<'static> {
        Scheme {
            scheme: self.scheme.into_owned().into(),
            details: self.details
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Scheme<'_> {
        Scheme {
            scheme: Cow::Borrowed(&self.scheme),
            details: self.details
        }
    }

    /// [`SchemeDetails::is_special`].
    pub fn is_special(&self) -> bool {
        self.details.is_special()
    }

    /// [`SchemeDetails::is_special_not_file`].
    pub fn is_special_not_file(&self) -> bool {
        self.details.is_special_not_file()
    }

    /// [`SchemeDetails::is_file`].
    pub fn is_file(&self) -> bool {
        self.details.is_file()
    }

    /// [`SchemeDetails::is_non_special`].
    pub fn is_non_special(&self) -> bool {
        self.details.is_non_special()
    }

    /// Its default port.
    pub fn default_port(&self) -> Option<u16> {
        self.details().default_port()
    }

    /// Its default port as a [`str`].
    pub fn default_port_str(&self) -> Option<&'static str> {
        self.details().default_port_str()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Scheme<'a> {
    type Error = InvalidScheme;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        let value = encode_scheme(value)?.1;

        Ok(Self {
            details: SchemeDetails::new_unchecked(&value),
            scheme: value
        })
    }
}
