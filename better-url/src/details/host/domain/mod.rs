//! [`DomainDetails`].

use crate::prelude::*;

mod prefix;
mod middle;
mod suffix;
mod fqddot;
mod origin;
mod labels;
mod normal;

/// Details for a [`DomainHost`].
/// # Examples
/// ```
/// use better_url::prelude::*;
///
/// let domain = "abc.def.example.co.uk.";
/// let details = DomainDetails::parse(domain).unwrap();
///
/// assert_eq!(&domain[details.prefix_range().unwrap()], "abc.def"               );
/// assert_eq!(&domain[details.predot_range().unwrap()],        "."              );
/// assert_eq!(&domain[details.middle_range().unwrap()],         "example"       );
/// assert_eq!(&domain[details.middot_range().unwrap()],                "."      );
/// assert_eq!(&domain[details.suffix_range()         ],                 "co.uk" );
/// assert_eq!(&domain[details.fqddot_range().unwrap()],                      ".");
///
/// assert_eq!(&domain[details.origin_range().unwrap()],         "example.co.uk" );
/// assert_eq!(&domain[details.labels_range()         ], "abc.def.example.co.uk" );
/// assert_eq!(&domain[details.normal_range()         ], "abc.def.example.co.uk" );
///
/// let domain = "www.example.co.uk";
/// let details = DomainDetails::parse(domain).unwrap();
///
/// assert_eq!(&domain[details.prefix_range().unwrap()], "www"               );
/// assert_eq!(&domain[details.predot_range().unwrap()],     "."             );
/// assert_eq!(&domain[details.middle_range().unwrap()],      "example"      );
/// assert_eq!(&domain[details.middot_range().unwrap()],             "."     );
/// assert_eq!(&domain[details.suffix_range()         ],              "co.uk");
/// assert_eq!(        details.fqddot_range()          , None                );
///
/// assert_eq!(&domain[details.origin_range().unwrap()],     "example.co.uk" );
/// assert_eq!(&domain[details.labels_range()         ], "www.example.co.uk" );
/// assert_eq!(&domain[details.normal_range()         ],     "example.co.uk" );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DomainDetails {
    /// If [`Self::ss`] is non-zero, the [`Range::start`] of the middle.
    pub(crate) ms: u32,
    /// The [`Range::start`] of the suffix.
    pub(crate) ss: u32,
    /// The [`Range::end`] of the suffix.
    pub(crate) sa: u32,
    /// If the domain is fully qualified.
    pub(crate) fq: bool,
    /// If the prefix is "www".
    pub(crate) www_prefix: bool,
}

impl DomainDetails {
    /// Parse a domain host.
    /// # Errors
    /// If `s` is empty, returns the error [`InvalidDomainHost`].
    ///
    /// If `s` contains any [`invalid_domain_byte`]s, returns the error [`InvalidDomainHost`].
    ///
    /// If `s` [`ends_in_a_number`], returns the error [`InvalidDomainHost`].
    pub fn parse(s: &str) -> Result<Self, InvalidDomainHost> {
        if s.is_empty() {
            Err(InvalidDomainHost)?;
        }

        if s.bytes().any(invalid_domain_byte) {
            Err(InvalidDomainHost)?;
        }

        if ends_in_a_number(s) {
            Err(InvalidDomainHost)?;
        }

        Ok(Self::parse_unchecked(s))
    }

    /// Parse a domain host without checking if it [`ends_in_a_number`].
    /// # Errors
    /// If `s` is empty, returns the error [`InvalidDomainHost`].
    ///
    /// If `s` contains any [`invalid_domain_byte`]s, returns the error [`InvalidDomainHost`].
    pub(crate) fn parse_not_eian(s: &str) -> Result<Self, InvalidDomainHost> {
        if s.is_empty() {
            Err(InvalidDomainHost)?;
        }

        if s.bytes().any(invalid_domain_byte) {
            Err(InvalidDomainHost)?;
        }

        Ok(Self::parse_unchecked(s))
    }

    /// Parse a domain host without validation checks.
    /// # Panics
    /// If the call to [`psl::suffix`] returns [`None`], panics.
    pub(crate) fn parse_unchecked(s: &str) -> Self {
        // [`psl::suffix_str`] uses [`str::from_utf8`] instead of [`str::from_utf8_unchecked`].
        let suffix = psl::suffix(s.as_bytes()).expect("A non-empty host").trim().as_bytes();

        let ss = (suffix as *const [u8]).addr() - s.addr();
        let sa = ss + suffix.len();

        let ms = match ss {
            0 => 0,
            x => s.my_substr_range(s[..x - 1].rsplit('.').next().expect("There to always be at least one segment")).start
        };

        Self {
            ms: ms as u32,
            ss: ss as u32,
            sa: sa as u32,
            fq: s.ends_with("."),
            www_prefix: &s[..ms] == "www."
        }
    }

    /// The length of the domain.
    #[allow(clippy::len_without_is_empty, reason = "Can't be empty.")]
    pub fn len(self) -> usize {
        self.sa as usize + self.fq as usize
    }
}

impl FromStr for DomainDetails {
    type Err = InvalidDomainHost;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for DomainDetails {
    type Error = InvalidDomainHost;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl TryFrom<HostDetails> for DomainDetails {
    type Error = HostDetails;

    fn try_from(value: HostDetails) -> Result<Self, Self::Error> {
        match value {
            HostDetails::Domain(details) => Ok(details),
            details => Err(details)
        }
    }
}
