//! [`DomainHostDetails`].

use psl::Psl;

use crate::prelude::*;

/// The details of where a domain's parts are.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DomainHostDetails {
    /// The first data usize.
    ///
    /// The topmost bit is if the prefix is `www`.
    ///
    /// The remaining bits are the byte index the middle starts at (if there is a middle).
    ///
    /// This works because allocations can never be longer than [`isize::MAX`], meaning the topmost bit is always free for shenanigans.
    pub data1: usize,
    /// The second data usize.
    ///
    /// The topmost bit is if the domain is fully qualified.
    ///
    /// The remaining bits are the byte index the suffix starts at.
    ///
    /// This works because allocations can never be longer than [`isize::MAX`], meaning the topmost bit is always free for shenanigans.
    pub data2: usize,
}

impl DomainHostDetails {
    /// Parse an encoded domain.
    /// # Errors
    /// If [`ends_in_a_number`] returns [`true`], returns the error [`InvalidDomainHost`].
    ///
    /// If [`Self::parse_not_eian`] returns an error, that error is returned.
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
        debug_assert!(!ends_in_a_number(value), "The domain to not end in a number");

        match value {
            "" => Err(InvalidDomainHost),
            _  => Ok(Self::parse_unchecked(value)),
        }
    }

    /// Parse a domain literal without checking for validity.
    ///
    /// Assumes that `value` is a valid output of [`domain_host_to_ascii`].
    #[allow(clippy::missing_panics_doc, reason = "It's a debug assert???")]
    pub fn parse_unchecked(value: &str) -> Self {
        debug_assert_eq!(value, domain_host_to_ascii(value).expect("The domain to be valid").1, "The domain to be ASCII.");

        let (segments, fq) = match value.as_bytes() {
            [value @ .., b'.'] => (value, true ),
             value             => (value, false),
        };

        let mut data2 = segments.len() - psl::List.find(SplitDots(Some(unsafe {str::from_utf8_unchecked(segments)})).rev().map(str::as_bytes)).len;

        let mut data1 = match data2 {
            0  => 0,
            ss => unsafe {value.get_unchecked(..ss - 1)}.memrchr(b'.').map_or(0, |x| x + 1)
        };

        data1 |= ((unsafe {value.get_unchecked(..data1)} == "www.") as usize) << (usize::BITS - 1);
        data2 |= (fq as usize) << (usize::BITS - 1);

        Self {
            data1,
            data2,
        }
    }

    /** If it has a prefix. **/ pub fn has_prefix(self) -> bool {self.data1 & (isize::MAX as usize) != 0}
    /** If it has a middle. **/ pub fn has_middle(self) -> bool {self.data2 & (isize::MAX as usize) != 0}

    /** The [`Range::start`] of the middle. **/ pub fn middle_start(self) -> Option<usize> {self.has_middle().then_some(self.data1 & (isize::MAX as usize))}
    /** The [`Range::start`] of the suffix. **/ pub fn suffix_start(self) ->        usize  {                            self.data2 & (isize::MAX as usize) }

    /** The [`Range::end`] of the prefix. **/ pub fn prefix_after(self) -> Option<usize> {(self.data1 & (isize::MAX as usize)).checked_sub(1)}
    /** The [`Range::end`] of the middle. **/ pub fn middle_after(self) -> Option<usize> {(self.data2 & (isize::MAX as usize)).checked_sub(1)}

    /** If the prefix is `www`.  **/ pub fn prefix_is_www(self) -> bool {self.data1 > (isize::MAX as usize)}
    /** If it's fully qualified. **/ pub fn is_fqdn      (self) -> bool {self.data2 > (isize::MAX as usize)}

    /** Set the middle start. **/ pub fn set_middle_start(&mut self, to: usize) {self.data1 = (self.data1 & (1 << (usize::BITS - 1))) | to;}
    /** Set the suffix start. **/ pub fn set_suffix_start(&mut self, to: usize) {self.data2 = (self.data2 & (1 << (usize::BITS - 1))) | to;}

    /** Set if the prefix is `www`.  **/ pub fn set_prefix_is_www(&mut self, to: bool) {self.data1 = (self.data1 & (isize::MAX as usize)) | ((to as usize) << (usize::BITS - 1));}
    /** Set if it's fully qualified. **/ pub fn set_is_fqdn      (&mut self, to: bool) {self.data2 = (self.data2 & (isize::MAX as usize)) | ((to as usize) << (usize::BITS - 1));}
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
