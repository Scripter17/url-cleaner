//! Details of a domain host.

use std::ops::Bound;
use std::str::FromStr;
use std::borrow::Cow;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::prelude::*;

/// The details of a domain host.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct DomainDetails {
    /// The start of the domain middle.
    pub middle_start: Option<u32>,
    /// The start of the domain suffix.
    pub suffix_start: Option<u32>,
    /// The location of the [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period.
    pub fqdn_period : Option<u32>
}

/// The enum of errors [`DomainDetails::parse`] can return.
#[derive(Debug, Error)]
pub enum GetDomainDetailsError {
    /// Returned when the provided host might be IPv6.
    #[error("The provided host might be IPv6.")]
    HostIsMaybeIpv6,
    /// Returned when the provided host might be IPv4.
    #[error("The provided host might be IPv4.")]
    HostIsMaybeIpv4,
    /// Returned when the provided host is empty.
    #[error("The provided host is empty.")]
    EmptyHost,
    /// Returned when an [`idna::Errors`] is encountered.
    #[error(transparent)]
    IdnaErrors(#[from] idna::Errors)
}

impl DomainDetails {
    /// Checks if `domain` is a valid domain, then returns its details.
    ///
    /// If you're absolutely certain the value you're using is a valid domain, you can use [`Self::parse_unchecked`].
    /// # Errors
    /// If the host might be IPv6, returns the error [`GetDomainDetailsError::HostIsMaybeIpv6`].
    ///
    /// If the host might be IPv4, returns the error [`GetDomainDetailsError::HostIsMaybeIpv4`].
    ///
    /// If the host is empty, returns the error [`GetDomainDetailsError::EmptyHost`].
    ///
    /// If the call to [`idna::domain_to_ascii_from_cow`] reutrns an error, that error is returned.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert_eq!(DomainDetails::parse(    "example.com"   ).unwrap(), DomainDetails {middle_start: Some(0), suffix_start: Some( 8), fqdn_period: None});
    /// assert_eq!(DomainDetails::parse("www.example.com"   ).unwrap(), DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: None});
    /// assert_eq!(DomainDetails::parse(    "example.co.uk" ).unwrap(), DomainDetails {middle_start: Some(0), suffix_start: Some( 8), fqdn_period: None});
    /// assert_eq!(DomainDetails::parse("www.example.co.uk" ).unwrap(), DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: None});
    /// assert_eq!(DomainDetails::parse(    "example.com."  ).unwrap(), DomainDetails {middle_start: Some(0), suffix_start: Some( 8), fqdn_period: Some(11)});
    /// assert_eq!(DomainDetails::parse("www.example.com."  ).unwrap(), DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: Some(15)});
    /// assert_eq!(DomainDetails::parse(    "example.co.uk.").unwrap(), DomainDetails {middle_start: Some(0), suffix_start: Some( 8), fqdn_period: Some(13)});
    /// assert_eq!(DomainDetails::parse("www.example.co.uk.").unwrap(), DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: Some(17)});
    ///
    /// DomainDetails::parse("127.0.0.1").unwrap_err();
    /// DomainDetails::parse("[::1]").unwrap_err();
    /// ```
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn parse(domain: &str) -> Result<Self, GetDomainDetailsError> {
        if domain.starts_with('[') {Err(GetDomainDetailsError::HostIsMaybeIpv6)?;}
        let domain: Cow<'_, [u8]> = percent_encoding::percent_decode(domain.as_bytes()).into();
        let domain = idna::domain_to_ascii_from_cow(domain, idna::AsciiDenyList::URL)?;
        if domain.is_empty() {Err(GetDomainDetailsError::EmptyHost)?;}
        let last = domain.strip_suffix('.').unwrap_or(&domain).rsplit('.').next().expect("The last segment to always exist.");
        if last.as_bytes().iter().all(|c| c.is_ascii_digit()) {Err(GetDomainDetailsError::HostIsMaybeIpv4)?;}

        Ok(Self::parse_unchecked(&domain))
    }

    /// Gets the details of a domain without checking it's actually a domain first.
    ///
    /// If you are at all possibly not working with a domain (like an IP host), please use [`Self::parse`] instead.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert_eq!(DomainDetails::parse_unchecked(    "example.com"   ), DomainDetails {middle_start: Some(0), suffix_start: Some( 8), fqdn_period: None});
    /// assert_eq!(DomainDetails::parse_unchecked("www.example.com"   ), DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: None});
    /// assert_eq!(DomainDetails::parse_unchecked(    "example.co.uk" ), DomainDetails {middle_start: Some(0), suffix_start: Some( 8), fqdn_period: None});
    /// assert_eq!(DomainDetails::parse_unchecked("www.example.co.uk" ), DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: None});
    /// assert_eq!(DomainDetails::parse_unchecked(    "example.com."  ), DomainDetails {middle_start: Some(0), suffix_start: Some( 8), fqdn_period: Some(11)});
    /// assert_eq!(DomainDetails::parse_unchecked("www.example.com."  ), DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: Some(15)});
    /// assert_eq!(DomainDetails::parse_unchecked(    "example.co.uk."), DomainDetails {middle_start: Some(0), suffix_start: Some( 8), fqdn_period: Some(13)});
    /// assert_eq!(DomainDetails::parse_unchecked("www.example.co.uk."), DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: Some(17)});
    /// ```
    pub fn parse_unchecked(domain: &str) -> Self {
        let suffix_start = psl::suffix(domain.as_bytes()).map(|suffix| (suffix.as_bytes().as_ptr() as u32) - (domain.as_ptr() as u32));
        Self {
            #[allow(clippy::indexing_slicing, reason = "Can't panic.")]
            middle_start: suffix_start.and_then(|ss| domain.as_bytes()[..ss as usize].rsplit(|x| *x==b'.').nth(1).map(|middle| (middle.as_ptr() as u32) - (domain.as_ptr() as u32))),
            suffix_start,
            fqdn_period : domain.strip_suffix('.').map(|x| x.len() as u32)
        }
    }

    /// The location of the period between subdomain and domain middle.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert_eq!(DomainDetails::parse(    "example.com"   ).unwrap().subdomain_period(), None   );
    /// assert_eq!(DomainDetails::parse("www.example.com"   ).unwrap().subdomain_period(), Some(3));
    /// assert_eq!(DomainDetails::parse(    "example.co.uk" ).unwrap().subdomain_period(), None   );
    /// assert_eq!(DomainDetails::parse("www.example.co.uk" ).unwrap().subdomain_period(), Some(3));
    /// assert_eq!(DomainDetails::parse(    "example.com."  ).unwrap().subdomain_period(), None   );
    /// assert_eq!(DomainDetails::parse("www.example.com."  ).unwrap().subdomain_period(), Some(3));
    /// assert_eq!(DomainDetails::parse(    "example.co.uk.").unwrap().subdomain_period(), None   );
    /// assert_eq!(DomainDetails::parse("www.example.co.uk.").unwrap().subdomain_period(), Some(3));
    /// ```
    pub fn subdomain_period(&self) -> Option<usize> {
        self.middle_start.and_then(|x| x.checked_sub(1).map(|x| x as usize))
    }
    /// The location of the period between domain middle and domain suffix.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert_eq!(DomainDetails::parse(    "example.com"   ).unwrap().domain_suffix_period(), Some( 7));
    /// assert_eq!(DomainDetails::parse("www.example.com"   ).unwrap().domain_suffix_period(), Some(11));
    /// assert_eq!(DomainDetails::parse(    "example.co.uk" ).unwrap().domain_suffix_period(), Some( 7));
    /// assert_eq!(DomainDetails::parse("www.example.co.uk" ).unwrap().domain_suffix_period(), Some(11));
    /// assert_eq!(DomainDetails::parse(    "example.com."  ).unwrap().domain_suffix_period(), Some( 7));
    /// assert_eq!(DomainDetails::parse("www.example.com."  ).unwrap().domain_suffix_period(), Some(11));
    /// assert_eq!(DomainDetails::parse(    "example.co.uk.").unwrap().domain_suffix_period(), Some( 7));
    /// assert_eq!(DomainDetails::parse("www.example.co.uk.").unwrap().domain_suffix_period(), Some(11));
    /// ```
    pub fn domain_suffix_period(&self) -> Option<usize> {
        self.suffix_start.and_then(|x| x.checked_sub(1).map(|x| x as usize))
    }

    /// The bounds of domain.
    ///
    /// Notably does not include [`Self::fqdn_period`]
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let x =     "example.com"   ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_bounds()],     "example.com"  );
    /// let x = "www.example.com"   ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_bounds()], "www.example.com"  );
    /// let x =     "example.co.uk" ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_bounds()],     "example.co.uk");
    /// let x = "www.example.co.uk" ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_bounds()], "www.example.co.uk");
    /// let x =     "example.com."  ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_bounds()],     "example.com"  );
    /// let x = "www.example.com."  ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_bounds()], "www.example.com"  );
    /// let x =     "example.co.uk."; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_bounds()],     "example.co.uk");
    /// let x = "www.example.co.uk."; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_bounds()], "www.example.co.uk");
    /// ```
    pub fn domain_bounds(&self) -> (Bound<usize>, Bound<usize>) {
        (Bound::Unbounded, exorub(self.fqdn_period.map(|x| x as usize)))
    }
    /// The bounds of subdomain.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let x =     "example.com"   ; assert_eq!(   DomainDetails::parse(x).unwrap().subdomain_bounds()          , None );
    /// let x = "www.example.com"   ; assert_eq!(&x[DomainDetails::parse(x).unwrap().subdomain_bounds().unwrap()], "www");
    /// let x =     "example.co.uk" ; assert_eq!(   DomainDetails::parse(x).unwrap().subdomain_bounds()          , None );
    /// let x = "www.example.co.uk" ; assert_eq!(&x[DomainDetails::parse(x).unwrap().subdomain_bounds().unwrap()], "www");
    /// let x =     "example.com."  ; assert_eq!(   DomainDetails::parse(x).unwrap().subdomain_bounds()          , None );
    /// let x = "www.example.com."  ; assert_eq!(&x[DomainDetails::parse(x).unwrap().subdomain_bounds().unwrap()], "www");
    /// let x =     "example.co.uk."; assert_eq!(   DomainDetails::parse(x).unwrap().subdomain_bounds()          , None );
    /// let x = "www.example.co.uk."; assert_eq!(&x[DomainDetails::parse(x).unwrap().subdomain_bounds().unwrap()], "www");
    /// ```
    pub fn subdomain_bounds(&self) -> Option<(Bound<usize>, Bound<usize>)> {
        self.subdomain_period().map(|x| (Bound::Unbounded, Bound::Excluded(x)))
    }
    /// The bounds of not domain suffix.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let x =     "example.com"   ; assert_eq!(&x[DomainDetails::parse(x).unwrap().not_domain_suffix_bounds().unwrap()],     "example");
    /// let x = "www.example.com"   ; assert_eq!(&x[DomainDetails::parse(x).unwrap().not_domain_suffix_bounds().unwrap()], "www.example");
    /// let x =     "example.co.uk" ; assert_eq!(&x[DomainDetails::parse(x).unwrap().not_domain_suffix_bounds().unwrap()],     "example");
    /// let x = "www.example.co.uk" ; assert_eq!(&x[DomainDetails::parse(x).unwrap().not_domain_suffix_bounds().unwrap()], "www.example");
    /// let x =     "example.com."  ; assert_eq!(&x[DomainDetails::parse(x).unwrap().not_domain_suffix_bounds().unwrap()],     "example");
    /// let x = "www.example.com."  ; assert_eq!(&x[DomainDetails::parse(x).unwrap().not_domain_suffix_bounds().unwrap()], "www.example");
    /// let x =     "example.co.uk."; assert_eq!(&x[DomainDetails::parse(x).unwrap().not_domain_suffix_bounds().unwrap()],     "example");
    /// let x = "www.example.co.uk."; assert_eq!(&x[DomainDetails::parse(x).unwrap().not_domain_suffix_bounds().unwrap()], "www.example");
    /// ```
    pub fn not_domain_suffix_bounds(&self) -> Option<(Bound<usize>, Bound<usize>)> {
        self.domain_suffix_period().map(|x| (Bound::Unbounded, Bound::Excluded(x)))
    }
    /// The bounds of domain middle.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let x =     "example.com"   ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_middle_bounds().unwrap()], "example");
    /// let x = "www.example.com"   ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_middle_bounds().unwrap()], "example");
    /// let x =     "example.co.uk" ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_middle_bounds().unwrap()], "example");
    /// let x = "www.example.co.uk" ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_middle_bounds().unwrap()], "example");
    /// let x =     "example.com."  ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_middle_bounds().unwrap()], "example");
    /// let x = "www.example.com."  ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_middle_bounds().unwrap()], "example");
    /// let x =     "example.co.uk."; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_middle_bounds().unwrap()], "example");
    /// let x = "www.example.co.uk."; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_middle_bounds().unwrap()], "example");
    /// ```
    pub fn domain_middle_bounds(&self) -> Option<(Bound<usize>, Bound<usize>)> {
        self.middle_start.zip(self.domain_suffix_period()).map(|(ms, sp)| (Bound::Included(ms as usize), Bound::Excluded(sp)))
    }
    /// The bounds of reg domain.
    ///
    /// Notably does not include [`Self::fqdn_period`]
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let x =     "example.com"   ; assert_eq!(&x[DomainDetails::parse(x).unwrap().reg_domain_bounds().unwrap()], "example.com"  );
    /// let x = "www.example.com"   ; assert_eq!(&x[DomainDetails::parse(x).unwrap().reg_domain_bounds().unwrap()], "example.com"  );
    /// let x =     "example.co.uk" ; assert_eq!(&x[DomainDetails::parse(x).unwrap().reg_domain_bounds().unwrap()], "example.co.uk");
    /// let x = "www.example.co.uk" ; assert_eq!(&x[DomainDetails::parse(x).unwrap().reg_domain_bounds().unwrap()], "example.co.uk");
    /// let x =     "example.com."  ; assert_eq!(&x[DomainDetails::parse(x).unwrap().reg_domain_bounds().unwrap()], "example.com"  );
    /// let x = "www.example.com."  ; assert_eq!(&x[DomainDetails::parse(x).unwrap().reg_domain_bounds().unwrap()], "example.com"  );
    /// let x =     "example.co.uk."; assert_eq!(&x[DomainDetails::parse(x).unwrap().reg_domain_bounds().unwrap()], "example.co.uk");
    /// let x = "www.example.co.uk."; assert_eq!(&x[DomainDetails::parse(x).unwrap().reg_domain_bounds().unwrap()], "example.co.uk");
    /// ```
    pub fn reg_domain_bounds(&self) -> Option<(Bound<usize>, Bound<usize>)> {
        self.middle_start.map(|x| (Bound::Included(x as usize), exorub(self.fqdn_period.map(|x| x as usize))))
    }
    /// The bounds of domain suffix.
    ///
    /// Notably does not include [`Self::fqdn_period`]
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let x =     "example.com"   ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_suffix_bounds().unwrap()], "com"  );
    /// let x = "www.example.com"   ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_suffix_bounds().unwrap()], "com"  );
    /// let x =     "example.co.uk" ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_suffix_bounds().unwrap()], "co.uk");
    /// let x = "www.example.co.uk" ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_suffix_bounds().unwrap()], "co.uk");
    /// let x =     "example.com."  ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_suffix_bounds().unwrap()], "com"  );
    /// let x = "www.example.com."  ; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_suffix_bounds().unwrap()], "com"  );
    /// let x =     "example.co.uk."; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_suffix_bounds().unwrap()], "co.uk");
    /// let x = "www.example.co.uk."; assert_eq!(&x[DomainDetails::parse(x).unwrap().domain_suffix_bounds().unwrap()], "co.uk");
    /// ```
    pub fn domain_suffix_bounds(&self) -> Option<(Bound<usize>, Bound<usize>)> {
        self.suffix_start.map(|x| (Bound::Included(x as usize), exorub(self.fqdn_period.map(|x| x as usize))))
    }
    /// If [`Self`] describes a [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name), return [`true`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert!(!DomainDetails::parse(    "example.com"   ).unwrap().is_fqdn());
    /// assert!(!DomainDetails::parse("www.example.com"   ).unwrap().is_fqdn());
    /// assert!(!DomainDetails::parse(    "example.co.uk" ).unwrap().is_fqdn());
    /// assert!(!DomainDetails::parse("www.example.co.uk" ).unwrap().is_fqdn());
    /// assert!( DomainDetails::parse(    "example.com."  ).unwrap().is_fqdn());
    /// assert!( DomainDetails::parse("www.example.com."  ).unwrap().is_fqdn());
    /// assert!( DomainDetails::parse(    "example.co.uk.").unwrap().is_fqdn());
    /// assert!( DomainDetails::parse("www.example.co.uk.").unwrap().is_fqdn());
    /// ```
    pub fn is_fqdn(&self) -> bool {
        self.fqdn_period.is_some()
    }
}

impl FromStr for DomainDetails {
    type Err = GetDomainDetailsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}
