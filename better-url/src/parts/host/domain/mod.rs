//! [`DomainHost`].

mod domain;
mod prefix;
mod middle;
mod suffix;
mod fqddot;
mod origin;
mod labels;
mod normal;

use crate::prelude::*;

/// A domain host.
///
/// The only invariant the various setter methods upholds is that a [`DomainDetails`] made from [`Self::as_str`] is always identical to [`Self::details`].
///
/// For example, the following is valid and intended behavior:
///
/// ```
/// use better_url::prelude::*;
///
/// let mut domain = DomainHost::try_from("example.co.uk").unwrap();
///
/// domain.set_suffix_segment(0, Some("abc.com")).unwrap();
///
/// assert_eq!(domain.prefix(), Some("example.abc"));
/// assert_eq!(domain.middle(), Some("com"));
/// assert_eq!(domain.suffix(), "uk");
/// ```
#[derive(Debug, Clone)]
pub struct DomainHost<'a> {
    /// The host string.
    pub(crate) host: Cow<'a, str>,
    /// The [`DomainDetails`].
    pub(crate) details: DomainDetails
}

impl<'a> DomainHost<'a> {
    /// The host as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.host
    }

    /// The [`DomainDetails`].
    pub fn details(&self) -> DomainDetails {
        self.details
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (Cow<'a, str>, DomainDetails) {
        (self.host, self.details)
    }



    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> DomainHost<'static> {
        DomainHost {
            host: self.host.into_owned().into(),
            details: self.details
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> DomainHost<'_> {
        DomainHost {
            host: Cow::Borrowed(&self.host),
            details: self.details
        }
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DomainHost<'a> {
    type Error = InvalidDomainHost;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if ends_in_a_number(&value) {
            Err(InvalidDomainHost)?;
        }

        let value = encode_domain(value);

        if value.len() > u32::MAX as usize {
            Err(InvalidDomainHost)?;
        }

        Ok(Self {
            details: value.parse()?,
            host: value,
        })
    }
}

impl<'a> TryFrom<Host<'a>> for DomainHost<'a> {
    type Error = Host<'a>;

    fn try_from(value: Host<'a>) -> Result<Self, Self::Error> {
        match value {
            Host::Domain(x) => Ok(x),
            _ => Err(value)
        }
    }
}
