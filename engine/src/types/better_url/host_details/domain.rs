//! Details of a domain host.

use std::ops::Bound;
use std::str::FromStr;

use serde::{Serialize, Deserialize};
#[expect(unused_imports, reason = "Doc links.")]
use url::Url;
use thiserror::Error;

use crate::util::*;
#[expect(unused_imports, reason = "Doc links.")]
use crate::types::*;

/// The details of a domain host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainDetails {
    /// The start of the [`UrlPart::DomainMiddle`].
    pub middle_start: Option<usize>,
    /// The start of the [`UrlPart::DomainSuffix`].
    pub suffix_start: Option<usize>,
    /// The location of the [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period.
    pub fqdn_period : Option<usize>
}

/// The enum of errors [`DomainDetails::parse`] can return.
#[derive(Debug, Error)]
pub enum GetDomainDetailsError {
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    ParseError(#[from] url::ParseError),
    /// Returned when the provided host isn't a domain.
    #[error("The provided host wasn't a domain.")]
    NotADomain
}

impl DomainDetails {
    /// Checks if `domain` is a valid domain, then returns its details.
    ///
    /// If you're absolutely certain the value you're using is a valid domain, you can use [`Self::parse_unchecked`].
    /// # Errors
    /// If the call to [`url::Host::parse`] returns an error, that error is returned.
    ///
    /// If the call to [`url::Host::parse`] doesn't return [`url::Host::Domain`], returns the error [`GetDomainDetailsError::NotADomain`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
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
    pub fn parse(domain: &str) -> Result<Self, GetDomainDetailsError> {
        if !matches!(url::Host::parse(domain)?, url::Host::Domain(_)) {return Err(GetDomainDetailsError::NotADomain);}

        Ok(Self::parse_unchecked(domain))
    }

    /// Gets the details of a domain without checking it's actually a domain first.
    ///
    /// If you are at all possibly not working with a domain (like an IP host), please use [`Self::parse`] instead.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
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
    #[allow(clippy::arithmetic_side_effects, reason = "Shouldn't be possible.")]
    pub fn parse_unchecked(domain: &str) -> Self {
        let suffix_start = psl::suffix(domain.as_bytes()).map(|suffix| (suffix.as_bytes().as_ptr() as usize) - (domain.as_ptr() as usize));
        Self {
            #[allow(clippy::indexing_slicing, reason = "Can't panic.")]
            middle_start: suffix_start.and_then(|ss| domain.as_bytes()[..ss].rsplit(|x| *x==b'.').nth(1).map(|middle| (middle.as_ptr() as usize) - (domain.as_ptr() as usize))),
            suffix_start,
            fqdn_period : domain.strip_suffix('.').map(|x| x.len())
        }
    }

    /// The location of the period between [`UrlPart::Subdomain`] and [`UrlPart::DomainMiddle`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
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
        self.middle_start.and_then(|x| x.checked_sub(1))
    }
    /// The location of the period between [`UrlPart::DomainMiddle`] and [`UrlPart::DomainSuffix`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
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
        self.suffix_start.and_then(|x| x.checked_sub(1))
    }

    /// The bounds of [`UrlPart::Domain`].
    ///
    /// Notably does not include [`Self::fqdn_period`]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
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
        (Bound::Unbounded, exorub(self.fqdn_period))
    }
    /// The bounds of [`UrlPart::Subdomain`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
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
    /// The bounds of [`UrlPart::NotDomainSuffix`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
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
    /// The bounds of [`UrlPart::DomainMiddle`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
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
        self.middle_start.zip(self.domain_suffix_period()).map(|(ms, sp)| (Bound::Included(ms), Bound::Excluded(sp)))
    }
    /// The bounds of [`UrlPart::RegDomain`].
    ///
    /// Notably does not include [`Self::fqdn_period`]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
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
        self.middle_start.map(|x| (Bound::Included(x), exorub(self.fqdn_period)))
    }
    /// The bounds of [`UrlPart::DomainSuffix`].
    ///
    /// Notably does not include [`Self::fqdn_period`]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
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
        self.suffix_start.map(|x| (Bound::Included(x), exorub(self.fqdn_period)))
    }
    /// If [`Self`] describes a [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name), return [`true`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
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
