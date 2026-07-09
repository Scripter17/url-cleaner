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

    /// [`SchemeDetails::type`].
    pub fn r#type(&self) -> SchemeType {
        self.details.r#type()
    }



    /// [`SchemeDetails::is_special`].
    pub fn is_special(&self) -> bool {
        self.details.is_special()
    }

    /// [`SchemeDetails::is_file`].
    pub fn is_file(&self) -> bool {
        self.details.is_file()
    }

    /// [`SchemeDetails::is_special_not_file`].
    pub fn is_special_not_file(&self) -> bool {
        self.details.is_special_not_file()
    }

    /// [`SchemeDetails::is_non_special`].
    pub fn is_non_special(&self) -> bool {
        self.details.is_non_special()
    }



    /// [`SchemeDetails::default_port`].
    pub fn default_port(&self) -> Option<Port<'static>> {
        self.details.default_port()
    }

    /// [`SchemeDetails::default_port_num`]
    pub fn default_port_num(&self) -> Option<u16> {
        self.details.default_port_num()
    }

    /// [`SchemeDetails::default_port_str`].
    pub fn default_port_str(&self) -> Option<&'static str> {
        self.details.default_port_str()
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Scheme<'_> {
        Scheme {
            scheme : Cow::Borrowed(&self.scheme),
            details: self.details,
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Scheme<'static> {
        Scheme {
            scheme : self.scheme.into_owned().into(),
            details: self.details,
        }
    }

    /// Turn into the inner [`Cow`] and [`SchemeDetails`].
    pub fn into_parts(self) -> (Cow<'a, str>, SchemeDetails) {
        (self.scheme, self.details)
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Scheme<'a> {
    type Error = InvalidScheme;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        let (_, value) = encode_scheme(value)?;

        Ok(Self {
            details: SchemeDetails::new_unchecked(&value),
            scheme : value
        })
    }
}

impl From<SpecialSchemeDetails> for Scheme<'static> {
    fn from(value: SpecialSchemeDetails) -> Self {
        match value {
            SpecialSchemeDetails::File          (x) => x.into(),
            SpecialSchemeDetails::SpecialNotFile(x) => x.into(),
        }
    }
}

impl From<FileSchemeDetails> for Scheme<'static> {
    fn from(value: FileSchemeDetails) -> Self {
        Self {
            scheme : value.as_str().into(),
            details: value.into()
        }
    }
}

impl From<SpecialNotFileSchemeDetails> for Scheme<'static> {
    fn from(value: SpecialNotFileSchemeDetails) -> Self {
        Self {
            scheme : value.as_str().into(),
            details: value.into()
        }
    }
}
