//! Details of a domain host.

use std::ops::Bound;

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

/// The enum of errors [`DomainDetails::from_domain_str`] can return.
#[derive(Debug, Error)]
pub enum GetDomainDetailsError {
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    ParseError(#[from] url::ParseError),
    /// Returned when the provided host isn't a domain.
    #[error("The provided host wasn't a domain.")]
    InvalidDomain
}

impl DomainDetails {
    /// Checks if `domain` is a valid domain, then returns its details.
    ///
    /// If you're absolutely certain the value you're using is a valid domain, you can use [`Self::from_domain_str_unchecked`].
    /// # Errors
    /// If the call to [`url::Host::parse`] returns an error, that error is returned.
    ///
    /// If the call to [`url::Host::parse`] doesn't return [`url::Host::Domain`], returns the error [`GetDomainDetailsError::InvalidDomain`].
    pub fn from_domain_str(domain: &str) -> Result<Self, GetDomainDetailsError> {
        if !matches!(url::Host::parse(domain)?, url::Host::Domain(_)) {return Err(GetDomainDetailsError::InvalidDomain);}

        Ok(Self::from_domain_str_unchecked(domain))
    }

    /// Gets the details of a domain without checking it's actually a domain first.
    ///
    /// If you are at all possibly not working with a domain (like an IP host), please use [`Self::from_domain_str`] instead.
    #[allow(clippy::arithmetic_side_effects, reason = "Shouldn't be possible.")]
    pub fn from_domain_str_unchecked(domain: &str) -> Self {
        Self {
            middle_start: psl::domain_str(domain).map(|notsub| (notsub.as_ptr() as usize) - (domain.as_ptr() as usize)),
            suffix_start: psl::suffix_str(domain).map(|suffix| (suffix.as_ptr() as usize) - (domain.as_ptr() as usize)),
            fqdn_period : domain.strip_suffix('.').map(|x| x.len())
        }
    }

    /// The location of the period between [`UrlPart::Subdomain`] and [`UrlPart::DomainMiddle`].
    pub fn subdomain_period        (&self) -> Option<usize> {self.middle_start.and_then(|x| x.checked_sub(1))}
    /// The location of the period between [`UrlPart::DomainMiddle`] and [`UrlPart::DomainSuffix`].
    pub fn domain_suffix_period    (&self) -> Option<usize> {self.suffix_start.and_then(|x| x.checked_sub(1))}

    /// The bounds of [`UrlPart::Domain`].
    ///
    /// Notably does not include [`Self::fqdn_period`]
    pub fn domain_bounds           (&self) ->        (Bound<usize>, Bound<usize>)  {                                                                  (Bound::Unbounded   , exorub(self.fqdn_period) )}
    /// The bounds of [`UrlPart::Subdomain`].
    pub fn subdomain_bounds        (&self) -> Option<(Bound<usize>, Bound<usize>)> {self.subdomain_period().map(|x|                                   (Bound::Unbounded   , Bound::Excluded(x))      )}
    /// The bounds of [`UrlPart::NotDomainSuffix`].
    pub fn not_domain_suffix_bounds(&self) -> Option<(Bound<usize>, Bound<usize>)> {self.domain_suffix_period().map(|x|                               (Bound::Unbounded   , Bound::Excluded(x))      )}
    /// The bounds of [`UrlPart::DomainMiddle`].
    pub fn domain_middle_bounds    (&self) -> Option<(Bound<usize>, Bound<usize>)> {self.middle_start.zip(self.domain_suffix_period()).map(|(ms, sp)| (Bound::Included(ms), Bound::Excluded(sp))     )}
    /// The bounds of [`UrlPart::RegDomain`].
    ///
    /// Notably does not include [`Self::fqdn_period`]
    pub fn reg_domain_bounds       (&self) -> Option<(Bound<usize>, Bound<usize>)> {self.middle_start.map(|x|                                         (Bound::Included(x) , exorub(self.fqdn_period)))}
    /// The bounds of [`UrlPart::DomainSuffix`].
    ///
    /// Notably does not include [`Self::fqdn_period`]
    pub fn domain_suffix_bounds    (&self) -> Option<(Bound<usize>, Bound<usize>)> {self.suffix_start.map(|x|                                         (Bound::Included(x) , exorub(self.fqdn_period)))}
    /// If [`Self`] describes a [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name), return [`true`].
    pub fn is_fqdn(&self) -> bool {self.fqdn_period.is_some()}
}
