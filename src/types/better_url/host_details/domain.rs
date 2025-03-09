//! Details of a domain host.

use std::ops::Bound;

use serde::{Serialize, Deserialize};

#[allow(unused_imports, reason = "Doc links.")]
use url::Url;
#[allow(unused_imports, reason = "Doc links.")]
use crate::types::*;

/// Details of a domain.
/// ```
/// # use url_cleaner::types::*;
/// // The weird values aren't guaranteed to be stable; They're just here to illustrate current behavior.
/// // "Weird" means anything with no middle and/or leading empty segments.
/// assert_eq!(DomainDetails::from_domain_str(""                     ), DomainDetails {middle_start: None   , suffix_start: None    , fqdn_period: None    }, ""                     );
/// assert_eq!(DomainDetails::from_domain_str("."                    ), DomainDetails {middle_start: None   , suffix_start: Some( 0), fqdn_period: Some( 0)}, "."                    );
/// assert_eq!(DomainDetails::from_domain_str("com"                  ), DomainDetails {middle_start: None   , suffix_start: Some( 0), fqdn_period: None    }, "com"                  );
/// assert_eq!(DomainDetails::from_domain_str("com."                 ), DomainDetails {middle_start: None   , suffix_start: Some( 0), fqdn_period: Some( 3)}, "com."                 );
/// assert_eq!(DomainDetails::from_domain_str(".com"                 ), DomainDetails {middle_start: None   , suffix_start: Some( 1), fqdn_period: None    }, ".com"                 );
/// assert_eq!(DomainDetails::from_domain_str(".com."                ), DomainDetails {middle_start: None   , suffix_start: Some( 1), fqdn_period: Some( 4)}, ".com."                );
/// assert_eq!(DomainDetails::from_domain_str("example.com"          ), DomainDetails {middle_start: Some(0), suffix_start: Some( 8), fqdn_period: None    }, "example.com"          );
/// assert_eq!(DomainDetails::from_domain_str("example.com."         ), DomainDetails {middle_start: Some(0), suffix_start: Some( 8), fqdn_period: Some(11)}, "example.com."         );
/// assert_eq!(DomainDetails::from_domain_str(".example.com"         ), DomainDetails {middle_start: Some(1), suffix_start: Some( 9), fqdn_period: None    }, ".example.com"         );
/// assert_eq!(DomainDetails::from_domain_str(".example.com."        ), DomainDetails {middle_start: Some(1), suffix_start: Some( 9), fqdn_period: Some(12)}, ".example.com."        );
/// assert_eq!(DomainDetails::from_domain_str("abc.example.com"      ), DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: None    }, "abc.example.com"      );
/// assert_eq!(DomainDetails::from_domain_str("abc.example.com."     ), DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: Some(15)}, "abc.example.com."     );
/// assert_eq!(DomainDetails::from_domain_str(".abc.example.com"     ), DomainDetails {middle_start: Some(5), suffix_start: Some(13), fqdn_period: None    }, ".abc.example.com"     );
/// assert_eq!(DomainDetails::from_domain_str(".abc.example.com."    ), DomainDetails {middle_start: Some(5), suffix_start: Some(13), fqdn_period: Some(16)}, ".abc.example.com."    );
/// assert_eq!(DomainDetails::from_domain_str("def.abc.example.com"  ), DomainDetails {middle_start: Some(8), suffix_start: Some(16), fqdn_period: None    }, "def.abc.example.com"  );
/// assert_eq!(DomainDetails::from_domain_str("def.abc.example.com." ), DomainDetails {middle_start: Some(8), suffix_start: Some(16), fqdn_period: Some(19)}, "def.abc.example.com." );
/// assert_eq!(DomainDetails::from_domain_str(".def.abc.example.com" ), DomainDetails {middle_start: Some(9), suffix_start: Some(17), fqdn_period: None    }, ".def.abc.example.com" );
/// assert_eq!(DomainDetails::from_domain_str(".def.abc.example.com."), DomainDetails {middle_start: Some(9), suffix_start: Some(17), fqdn_period: Some(20)}, ".def.abc.example.com.");
///
/// for subdomain in [None, Some(""), Some("abc"), Some(".abc"), Some("def.abc")] {
///     for middle in [None, Some(""), Some("abc")] {
///         for suffix in [None, Some(""), Some("com"), Some("co.uk")] {
///             for fqdn_period in [None, Some(".")] {
///                 let domain = [subdomain, middle, suffix, fqdn_period].into_iter().filter_map(|x| x).collect::<String>();
///                 // Checks that, at the very least, [`Self::suffix_bounds`] and [`Self::reg_domain_bounds`] agree with [`psl`].
///                 // The other bounds have no equivalents in [`psl`].
///                 let domain_details = DomainDetails::from_domain_str(&domain);
///                 assert_eq!(psl::suffix_str(&domain), domain_details.suffix_bounds    ().and_then(|bounds| domain.get(bounds)),     "suffix, {domain:?}, {domain_details:?}");
///                 assert_eq!(psl::domain_str(&domain), domain_details.reg_domain_bounds().and_then(|bounds| domain.get(bounds)), "reg_domain, {domain:?}, {domain_details:?}");
///
///                 assert!(domain_details.subdomain_period().is_none_or(|i| &domain[i..i+1]=="."));
///                 assert!(domain_details.suffix_period   ().is_none_or(|i| &domain[i..i+1]=="."));
///             }
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainDetails {
    /// The index of the start of the domain's middle, if it exists.
    pub middle_start: Option<usize>,
    /// The index of the start of the domain's suffix, if it exists.
    pub suffix_start: Option<usize>,
    /// The index of the period that marks the domain as fully qualified, if it exists.
    pub fqdn_period : Option<usize>
}

impl DomainDetails {
    /// Creates a [`Self`] from a domain [`str`].
    ///
    /// PLEASE note that passing, for example, `"127.0.0.1"` gives very nonsensical results.
    ///
    /// If you are even remotely possibly not always handling domains, please use [`HostDetails::from_host`] or [`HostDetails::from_host_str`].
    #[allow(clippy::arithmetic_side_effects, reason = "Shouldn't be possible.")]
    pub fn from_domain_str(domain: &str) -> Self {
        Self {
            middle_start: psl::domain_str(domain).map(|notsub| (notsub.as_ptr() as usize) - (domain.as_ptr() as usize)),
            suffix_start: psl::suffix_str(domain).map(|suffix| (suffix.as_ptr() as usize) - (domain.as_ptr() as usize)),
            fqdn_period : domain.strip_suffix('.').map(|x| x.len())
        }
    }

    /// Gets the location of the period separating the subdomain and middle, if it exists.
    pub fn subdomain_period(&self) -> Option<usize> {self.middle_start.and_then(|x| x.checked_sub(1))}
    /// Gets the location of the period separating the middle and suffix, if it exists.
    pub fn suffix_period   (&self) -> Option<usize> {self.suffix_start.and_then(|x| x.checked_sub(1))}

    /// Everything but the fqdn period.
    pub fn domain_bounds    (&self) ->        (Bound<usize>, Bound<usize>)  {(Bound::Unbounded, exorub(self.fqdn_period))}
    /// Gets the range in the domain corresponding to [`UrlPart::Subdomain`].
    pub fn subdomain_bounds (&self) -> Option<(Bound<usize>, Bound<usize>)> {self.subdomain_period().map(|x| (Bound::Unbounded, Bound::Excluded(x)))}
    /// Gets the range in the domain corresponding to [`UrlPart::NotDomainSuffix`].
    pub fn not_suffix_bounds(&self) -> Option<(Bound<usize>, Bound<usize>)> {self.suffix_period()   .map(|x| (Bound::Unbounded, Bound::Excluded(x)))}
    /// Gets the range in the domain corresponding to [`UrlPart::DomainMiddle`].
    pub fn middle_bounds    (&self) -> Option<(Bound<usize>, Bound<usize>)> {self.middle_start.zip(self.suffix_period()).map(|(ms, sp)| (Bound::Included(ms), Bound::Excluded(sp)))}
    /// Gets the range in the domain corresponding to [`UrlPart::RegDomain`].
    ///
    /// Intended to give the same substring as [`psl::domain_str`].
    pub fn reg_domain_bounds(&self) -> Option<(Bound<usize>, Bound<usize>)> {self.middle_start.map(|x| (Bound::Included(x), exorub(self.fqdn_period)))}
    /// Gets the range in the domain corresponding to [`UrlPart::DomainSuffix`].
    ///
    /// Intended to give the same substring as [`psl::suffix_str`].
    pub fn suffix_bounds    (&self) -> Option<(Bound<usize>, Bound<usize>)> {self.suffix_start.map(|x| (Bound::Included(x), exorub(self.fqdn_period)))}

    /// Returns [`true`] if [`Self::fqdn_period`] is [`Some`].
    pub fn is_fqdn(&self) -> bool {self.fqdn_period.is_some()}
}

/// Helper function to make [`DomainDetails`]'s various bounds functions easier to read and write.
fn exorub(i: Option<usize>) -> Bound<usize> {
    match i {
        Some(i) => Bound::Excluded(i),
        None => Bound::Unbounded
    }
}
