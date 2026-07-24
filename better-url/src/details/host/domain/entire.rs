//! [`DomainHostDetails`].

use crate::prelude::*;

/// The details of where a domain's parts are.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DomainHostDetails {
    /// If [`Self::ss`] is non-zero, the [`Range::start`] of the middle.
    pub ms: u32,
    /// The [`Range::start`] of the suffix.
    pub ss: u32,
    /// If the domain is fully qualified.
    pub fq: bool,
    /// If the prefix is `www`.
    pub wp: bool,
}

impl DomainHostDetails {
    /// Parse an encoded domain.
    /// # Errors
    /// If the call to [`ends_in_a_number`] returns [`true`], returns the error [`InvalidDomainHost`].
    ///
    /// If the call to [`Self::parse_not_eian`] returns an error, that error is returned.
    pub fn parse(value: &str) -> Result<Self, InvalidDomainHost> {
        match ends_in_a_number(value) {
            true  => Err(InvalidDomainHost),
            false => Self::parse_not_eian(value),
        }
    }

    /// Parse a not-[`ends_in_a_number`] encoded domain.
    /// # Errors
    /// If `value` is empty, returns the error [`InvalidDomainHost`].
    pub fn parse_not_eian(value: &str) -> Result<Self, InvalidDomainHost> {
        debug_assert!(!ends_in_a_number(value));

        match value {
            "" => Err(InvalidDomainHost),
            _  => Ok(Self::parse_unchecked(value)),
        }
    }

    /// Parse a domain literal without checking for validity.
    /// # Panics
    /// If the call to [`psl::suffix`] returns [`None`] (`value` is empty), panics.
    ///
    /// As far as I know, that should only happen with the empty string.
    pub fn parse_unchecked(value: &str) -> Self {
        debug_assert!(!value.is_empty(), "The domain to not be empty.");
        debug_assert!(!encode_domain_host(value).expect("The domain to be valid").0, "The domain to be encoded.");

        let (suffix, fq) = match psl::suffix(value.as_bytes()).expect("The domain to not be empty.").as_bytes() {
            [suffix @ .., b'.'] => (suffix, true ),
             suffix             => (suffix, false),
        };

        let ss = suffix.as_ptr().addr() - value.addr();

        let ms = match ss {
            0  => 0,
            ss => unsafe {value.get_unchecked(..ss - 1)}.memrchr(b'.').map_or(0, |x| x + 1)
        };

        Self {
            ms: ms as u32,
            ss: ss as u32,
            fq,
            wp: unsafe {value.get_unchecked(..ms)} == "www.",
        }
    }
}



impl FromStr for DomainHostDetails {
    type Err = InvalidDomainHost;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for DomainHostDetails {
    type Error = InvalidDomainHost;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<HostDetails> for DomainHostDetails {
    type Error = HostDetails;

    fn try_from(value: HostDetails) -> Result<Self, Self::Error> {
        match value {
            HostDetails::Domain(details) => Ok (details),
            details                      => Err(details),
        }
    }
}

impl TryFrom<FileHostDetails> for DomainHostDetails {
    type Error = FileHostDetails;

    fn try_from(value: FileHostDetails) -> Result<Self, Self::Error> {
        match value {
            FileHostDetails::Domain(details) => Ok (details),
            details                          => Err(details),
        }
    }
}

impl TryFrom<SpecialNotFileHostDetails> for DomainHostDetails {
    type Error = SpecialNotFileHostDetails;

    fn try_from(value: SpecialNotFileHostDetails) -> Result<Self, Self::Error> {
        match value {
            SpecialNotFileHostDetails::Domain(details) => Ok (details),
            details                                    => Err(details),
        }
    }
}
