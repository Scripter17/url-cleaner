//! [`DomainPartsDetails`].

use crate::prelude::*;

mod prefix;
mod middle;
mod suffix;
mod fqddot;
mod origin;
mod labels;
mod normal;

/// The details of where a domain's parts are.
/// # Examples
/// ```
/// use better_url::prelude::*;
///
/// let domain = "abc.def.example.co.uk.";
/// let details = DomainPartsDetails::from_raw_unchecked(domain);
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
/// let details = DomainPartsDetails::from_raw_unchecked(domain);
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
pub struct DomainPartsDetails {
    /// If [`Self::ss`] is non-zero, the [`Range::start`] of the middle.
    pub(crate) ms: u32,
    /// The [`Range::start`] of the suffix.
    pub(crate) ss: u32,
    /// The [`Range::end`] of the suffix.
    pub(crate) sa: u32,
    /// If the domain is fully qualified.
    pub(crate) fq: bool,
    /// If the prefix is `www`.
    pub(crate) wp: bool,
    /// The number of segments after the middle.
    pub(crate) mi: Option<NonZero<u8>>,
}

impl DomainPartsDetails {
    /// Parse a domain literal without checking for validity.
    /// # Panics
    /// If the call to [`psl::suffix`] returns [`None`] (`value` is empty), panics.
    pub fn from_raw_unchecked(value: &str) -> Self {
        let suffix = psl::suffix(value.as_bytes()).expect("A non-empty host").trim().as_bytes();

        let ss = (suffix as *const [u8]).addr() - value.addr();
        let sa = ss + suffix.len();

        let ms = match ss {
            0 => 0,
            x => value.as_bytes()[..x - 1].iter().rposition(|&b| b == b'.').map_or(0, |x| x + 1)
        };

        let mi = match ss {
            0 => None,
            _ => NonZero::new({
                let mut acc = 1;
                for &b in suffix {
                    if b == b'.' {
                        acc += 1;
                    }
                }
                acc
            })
        };

        Self {
            ms: ms as u32,
            ss: ss as u32,
            sa: sa as u32,
            fq: sa != value.len(),
            wp: &value[..ms] == "www.",
            mi,
        }
    }

    /// The length of the domain.
    #[expect(clippy::len_without_is_empty, reason = "Can't be empty.")]
    pub fn len(&self) -> usize {
        self.sa as usize + self.fq as usize
    }
}
