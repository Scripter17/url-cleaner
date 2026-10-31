//! [`DomainHostDetails`].

use crate::prelude::*;

/// The details of a domain host.
/// # Examples
/// ```
/// use better_url::prelude::*;
///
/// assert!(std::mem::size_of::<DomainHostDetails>() <= std::mem::size_of::<Ipv6HostDetails>());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct DomainHostDetails {
    /// The first data usize.
    ///
    /// If the feature `psl` is enabled, the topmost bit is if the prefix is `www` and the remaining bits are the byte index the middle (if present) starts at.
    ///
    /// If the feature `psl` is disabled, none of the bits have any meaning.
    pub data1: usize,
    /// The second data usize.
    ///
    /// If the feature `psl` is enabled, the topmost bit is if the domain is fully qualified and the remaining bits are the byte index the suffix starts at.
    ///
    /// If the feature `psl` is disabled, the topmost bit is if the domain is fully qualified but the remaining bits have no meaning.
    pub data2: usize,
}

impl DomainHostDetails {
    /// Parse a [`DomainHost`] literal.
    /// # Errors
    /// If `value` is empty, returns the error [`InvalidDomainHost`].
    ///
    /// If `value`'s class under [`DOMAIN_BYTE_CLASS_DATA`] has the lowest, third lowest, and/or fifth lowest bits set, returns the error [`InvalidDomainHost`].
    ///
    /// If `value`'s class under [`DOMAIN_BYTE_CLASS_DATA`] has the second lowest bit set and `value` [`ends_in_a_number`], returns the error [`InvalidDomainHost`].
    pub fn parse(value: &str) -> Result<Self, InvalidDomainHost> {
        if value.is_empty() {
            Err(InvalidDomainHost)?;
        }

        let class = value.bytes().fold(0, |acc, b| acc | DOMAIN_BYTE_CLASS_DATA[b as usize]);

        if class & 0b0001_0101 != 0 {
            Err(InvalidDomainHost)?;
        }

        if class & 0b0000_0010 != 0 && ends_in_a_number(value) {
            Err(InvalidDomainHost)?;
        }

        Ok(Self::parse_unchecked(value))
    }

    /// Parse a [`DomainHost`] literal without checking if every byte is valid, if it ends in a number, or if it is empty.
    pub fn parse_unchecked(value: &str) -> Self {
        cfg_select! {
            feature = "psl" => {
                let mut data2 = get_domain_suffix(value).addr() - value.addr();

                let mut data1 = match data2 {
                    0 => 0,
                    _ => unsafe {value.get_unchecked(..data2 - 1)}.memrchr(b'.').map_or(0, |x| x + 1)
                };

                data1 |= ((unsafe {value.get_unchecked(..data1)} == "www.") as usize) << (usize::BITS - 1);
                data2 |= (value.ends_with('.') as usize) << (usize::BITS - 1);

                Self {
                    data1,
                    data2,
                }
            },
            _ => {
                Self {
                    data1: 0,
                    data2: (value.ends_with('.') as usize) << (usize::BITS - 1),
                }
            }
        }
    }

    /** If it has a prefix.                 */ #[cfg(feature = "psl")] pub fn has_prefix(self) -> bool {self.data1 & (isize::MAX as usize) != 0}
    /** If it has a middle.                 */ #[cfg(feature = "psl")] pub fn has_middle(self) -> bool {self.data2 & (isize::MAX as usize) != 0}

    /** The [`Range::start`] of the middle. */ #[cfg(feature = "psl")] pub fn middle_start(self) -> Option<usize> {self.has_middle().then_some(self.data1 & (isize::MAX as usize))}
    /** The [`Range::start`] of the suffix. */ #[cfg(feature = "psl")] pub fn suffix_start(self) ->        usize  {                            self.data2 & (isize::MAX as usize) }

    /** The [`Range::end`] of the prefix.   */ #[cfg(feature = "psl")] pub fn prefix_after(self) -> Option<usize> {(self.data1 & (isize::MAX as usize)).checked_sub(1)}
    /** The [`Range::end`] of the middle.   */ #[cfg(feature = "psl")] pub fn middle_after(self) -> Option<usize> {(self.data2 & (isize::MAX as usize)).checked_sub(1)}

    /** If the prefix is `www`.             */ #[cfg(feature = "psl")] pub fn prefix_is_www(self) -> bool {self.data1 > (isize::MAX as usize)}
    /** If it's fully qualified.            */                         pub fn is_fqdn      (self) -> bool {self.data2 > (isize::MAX as usize)}

    /** Set the middle start.               */ #[cfg(feature = "psl")] pub fn set_middle_start(&mut self, to: usize) {self.data1 = (self.data1 & (1 << (usize::BITS - 1))) | to;}
    /** Set the suffix start.               */ #[cfg(feature = "psl")] pub fn set_suffix_start(&mut self, to: usize) {self.data2 = (self.data2 & (1 << (usize::BITS - 1))) | to;}

    /** Set if the prefix is `www`.         */ #[cfg(feature = "psl")] pub fn set_prefix_is_www(&mut self, to: bool) {self.data1 = (self.data1 & (isize::MAX as usize)) | ((to as usize) << (usize::BITS - 1));}
    /** Set if it's fully qualified.        */                         pub fn set_is_fqdn      (&mut self, to: bool) {self.data2 = (self.data2 & (isize::MAX as usize)) | ((to as usize) << (usize::BITS - 1));}
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
