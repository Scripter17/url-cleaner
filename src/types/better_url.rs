//! A wrapper around [`url::Url`] with extra metadata.

use std::net::IpAddr;
use std::str::{FromStr, Split};
use std::ops::Deref;

use serde::{Serialize, Deserialize};
use url::{Url, UrlQuery, PathSegmentsMut, ParseError};
use form_urlencoded::Serializer;
use thiserror::Error;

mod host_details;
pub use host_details::*;

#[expect(unused_imports, reason = "Used in docs.")]
use crate::types::*;

/// A wrapper around a [`Url`] with extra metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "Url", into = "Url")]
pub struct BetterUrl {
    /// The [`Url`].
    url: Url,
    /// The [`HostDetails`] of [`Self::url`].
    host_details: Option<HostDetails>
}

/// The error [`BetterUrl::set_port`] returns when it fails.
#[derive(Debug, Error)]
#[error("Failed to set the port.")]
pub struct SetPortError;

/// The error [`BetterUrl::set_ip_host`] returns when it fails.
#[derive(Debug, Error)]
#[error("Failed to set the host to an IP.")]
pub struct SetIpHostError;

/// The error [`BetterUrl::set_password`] returns when it fails.
#[derive(Debug, Error)]
#[error("Failed to set the password.")]
pub struct SetPasswordError;

/// The error [`BetterUrl::set_username`] returns when it fails.
#[derive(Debug, Error)]
#[error("Failed to set the username.")]
pub struct SetUsernameError;

/// The error [`BetterUrl::set_scheme`] returns when it fails.
#[derive(Debug, Error)]
#[error("Failed to set the scheme.")]
pub struct SetSchemeError;

/// The enum of errors [`BetterUrl::set_domain`] can return.
#[derive(Debug, Error)]
pub enum SetDomainError {
    /// Returned when the resulting value isn't parseable as a domain.
    #[error(transparent)]
    SetDomainHostError(#[from] SetDomainHostError),
    /// Returned when trying to set the host of a URL that must have a host to [`None`].
    #[error(transparent)]
    ParseError(#[from] ParseError)
}

/// The enum of errors [`BetterUrl::set_subdomain`] can return.
#[derive(Debug, Error)]
pub enum SetSubdomainError {
    /// Returned when the [`BetterUrl`]'s host isn't a domain.
    #[error("The host was not a domain.")]
    HostIsNotADomain,
    /// Returned when trying to set the [`UrlPart::Subdomain`] on a domain without a [`UrlPart::RegDomain`].
    #[error("Tried to set the subdomain on a domain without a reg domain.")]
    MissingRegDomain,
    /// Returned when the resulting value isn't parseable as a domain.
    #[error(transparent)]
    SetDomainHostError(#[from] SetDomainHostError)
}

/// The enum of errors [`BetterUrl::set_not_domain_suffix`] can return.
#[derive(Debug, Error)]
pub enum SetNotDomainSuffixError {
    /// Returned when the [`BetterUrl`]'s host isn't a domain.
    #[error("The host was not a domain.")]
    HostIsNotADomain,
    /// Returned when trying to set the [`UrlPart::NotDomainSuffix`] on a domain without a [`UrlPart::DomainSuffix`].
    #[error("Tried to set the not domain suffix on a domain without a domain suffix.")]
    MissingDomainSuffix,
    /// Returned when the resulting value isn't parseable as a domain.
    #[error(transparent)]
    SetDomainHostError(#[from] SetDomainHostError)
}

/// The enum of errors [`BetterUrl::set_domain_middle`] can return.
#[derive(Debug, Error)]
pub enum SetDomainMiddleError {
    /// Returned when the [`BetterUrl`]'s host isn't a domain.
    #[error("The host was not a domain.")]
    HostIsNotADomain,
    /// Returned when trying to set the [`UrlPart::DomainMiddle`] on a domain without a [`UrlPart::DomainSuffix`].
    #[error("Tried to set the domain middle on a domain without a domain suffix.")]
    MissingDomainSuffix,
    /// Returned when the resulting value isn't parseable as a domain.
    #[error(transparent)]
    SetDomainHostError(#[from] SetDomainHostError)
}

/// The enum of errors [`BetterUrl::set_reg_domain`] can return.
#[derive(Debug, Error)]
pub enum SetRegDomainError {
    /// Returned when the [`BetterUrl`]'s host isn't a domain.
    #[error("The host was not a domain.")]
    HostIsNotADomain,
    /// Returned when the resulting value isn't parseable as a domain.
    #[error(transparent)]
    SetDomainHostError(#[from] SetDomainHostError)
}

/// The enum of errors [`BetterUrl::set_domain_suffix`] can return.
#[derive(Debug, Error)]
pub enum SetDomainSuffixError {
    /// Returned when the [`BetterUrl`]'s host isn't a domain.
    #[error("The host was not a domain.")]
    HostIsNotADomain,
    /// Returned when the resulting value isn't parseable as a domain.
    #[error(transparent)]
    SetDomainHostError(#[from] SetDomainHostError)
}

/// The enum of errors [`BetterUrl::set_domain_host`] can return.
#[derive(Debug, Error)]
pub enum SetDomainHostError {
    /// Returned when the provided host isn't a domain.
    #[error("The provided host was not a domain.")]
    ProvidedHostIsNotADomain,
    /// Returned when a [`ParseError`] is encountered.
    #[error(transparent)]
    ParseError(#[from] ParseError)
}

/// The error returned by [`BetterUrl::path_segments`] and [`BetterUrl::path_segments_mut`] return when the [`BetterUrl`]'s path doesn't have segments.
#[derive(Debug, Error)]
#[error("The URL does not have path segments.")]
pub struct UrlDoesNotHavePathSegments;

impl BetterUrl {
    /// Parse a URL.
    /// # Errors
    /// If the call to [`Url::parse`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// let url = BetterUrl::parse("https://example.com").unwrap();
    /// ```
    pub fn parse(value: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::from_str(value)
    }

    /// Get the contained [`HostDetails`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// let url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(url.host_details(), Some(&HostDetails::Domain(DomainDetails {middle_start: Some(0), suffix_start: Some(8), fqdn_period: None})));
    /// 
    /// let url = BetterUrl::parse("https://127.0.0.1").unwrap();
    ///
    /// assert_eq!(url.host_details(), Some(&HostDetails::Ipv4(Ipv4Details {})));
    /// 
    /// let url = BetterUrl::parse("https://[::1]").unwrap();
    ///
    /// assert_eq!(url.host_details(), Some(&HostDetails::Ipv6(Ipv6Details {})));
    /// ```
    pub fn host_details(&self) -> Option<&HostDetails> {
        self.host_details.as_ref()
    }

    /// If [`Self::host_details`] returns [`HostDetails::Domain`], return it.
    /// ```
    /// # use url_cleaner::types::*;
    /// let url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(url.domain_details(), Some(&DomainDetails {middle_start: Some(0), suffix_start: Some(8), fqdn_period: None}));
    /// assert_eq!(url.ipv4_details  (), None);
    /// assert_eq!(url.ipv6_details  (), None);
    /// ```
    pub fn domain_details(&self) -> Option<&DomainDetails> {self.host_details()?.domain_details()}
    /// If [`Self::host_details`] returns [`HostDetails::Ipv4`], return it.
    /// ```
    /// # use url_cleaner::types::*;
    /// let url = BetterUrl::parse("https://127.0.0.1").unwrap();
    ///
    /// assert_eq!(url.domain_details(), None);
    /// assert_eq!(url.ipv4_details  (), Some(&Ipv4Details {}));
    /// assert_eq!(url.ipv6_details  (), None);
    /// ```
    pub fn ipv4_details  (&self) -> Option<&Ipv4Details> {self.host_details()?.ipv4_details()}
    /// If [`Self::host_details`] returns [`HostDetails::Ipv6`], return it.
    /// ```
    /// # use url_cleaner::types::*;
    /// let url = BetterUrl::parse("https://[::1]").unwrap();
    ///
    /// assert_eq!(url.domain_details(), None);
    /// assert_eq!(url.ipv4_details  (), None);
    /// assert_eq!(url.ipv6_details  (), Some(&Ipv6Details {}));
    /// ```
    pub fn ipv6_details  (&self) -> Option<&Ipv6Details> {self.host_details()?.ipv6_details()}

    /// [`Url::domain`] but without the [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().domain(), Some("example.com"      ));
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().domain(), Some("example.co.uk"    ));
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().domain(), Some("www.example.com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().domain(), Some("www.example.co.uk"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().domain(), Some("www.example.com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().domain(), Some("www.example.co.uk"));
    /// ```
    pub fn domain(&self) -> Option<&str> {self.host_str()?.get(self.domain_details()?.domain_bounds())}
    /// If [`Self`] has a [`UrlPart::Subdomain`], return it.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().subdomain(), None);
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().subdomain(), None);
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().subdomain(), Some("www"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().subdomain(), Some("www"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().subdomain(), Some("www"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().subdomain(), Some("www"));
    /// ```
    pub fn subdomain(&self) -> Option<&str> {self.host_str()?.get(self.domain_details()?.subdomain_bounds()?)}
    /// If [`Self`] has a [`UrlPart::NotDomainSuffix`], return it.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().not_domain_suffix(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().not_domain_suffix(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().not_domain_suffix(), Some("www.example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().not_domain_suffix(), Some("www.example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().not_domain_suffix(), Some("www.example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().not_domain_suffix(), Some("www.example"));
    /// ```
    pub fn not_domain_suffix(&self) -> Option<&str> {self.host_str()?.get(self.domain_details()?.not_domain_suffix_bounds()?)}
    /// If [`Self`] has a [`UrlPart::DomainMiddle`], return it.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().domain_middle(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().domain_middle(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().domain_middle(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().domain_middle(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().domain_middle(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().domain_middle(), Some("example"));
    /// ```
    pub fn domain_middle(&self) -> Option<&str> {self.host_str()?.get(self.domain_details()?.domain_middle_bounds()?)}
    /// If [`Self`] has a [`UrlPart::RegDomain`], return it.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().reg_domain(), Some("example.com"  ));
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().reg_domain(), Some("example.co.uk"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().reg_domain(), Some("example.com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().reg_domain(), Some("example.co.uk"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().reg_domain(), Some("example.com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().reg_domain(), Some("example.co.uk"));
    /// ```
    pub fn reg_domain(&self) -> Option<&str> {self.host_str()?.get(self.domain_details()?.reg_domain_bounds()?)}
    /// If [`Self`] has a [`UrlPart::DomainSuffix`], return it.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().domain_suffix(), Some("com"  ));
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().domain_suffix(), Some("co.uk"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().domain_suffix(), Some("com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().domain_suffix(), Some("co.uk"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().domain_suffix(), Some("com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().domain_suffix(), Some("co.uk"));
    /// ```
    pub fn domain_suffix(&self) -> Option<&str> {self.host_str()?.get(self.domain_details()?.domain_suffix_bounds()?)}

    /// Gets an object that can iterate over the segments of [`Self`]'s path.
    /// # Errors
    /// If the call to [`Url::path_segments`] returns [`None`], returns the error [`UrlDoesNotHavePathSegments`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().path_segments().unwrap().collect::<Vec<_>>(), [""]);
    /// assert_eq!(BetterUrl::parse("https://example.com/a/b/c" ).unwrap().path_segments().unwrap().collect::<Vec<_>>(), ["a", "b", "c"]);
    /// assert_eq!(BetterUrl::parse("https://example.com/a/b/c/").unwrap().path_segments().unwrap().collect::<Vec<_>>(), ["a", "b", "c", ""]);
    /// ```
    pub fn path_segments    (&self) -> Result<Split<'_, char>, UrlDoesNotHavePathSegments> {self.url.path_segments().ok_or(UrlDoesNotHavePathSegments)}
    /// Gets an object that can mutate the segments of [`Self`]'s path.
    /// # Errors
    /// If the call to [`Url::path_segments_mut`] returns an error, returns the error [`UrlDoesNotHavePathSegments`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c/").unwrap();
    ///
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/a/b/c");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/a/b");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/a");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/");
    /// ```
    pub fn path_segments_mut(&mut self                        ) -> Result<PathSegmentsMut<'_>, UrlDoesNotHavePathSegments> {self.url.path_segments_mut().map_err(|()| UrlDoesNotHavePathSegments)}

    /// Sets the [`UrlPart::Scheme`].
    /// # Errors
    /// If the call to [`Url::set_scheme`] returns an error, returns the error [`SetSchemeError`].
    pub fn set_scheme       (&mut self, scheme  : &str        ) -> Result<(), SetSchemeError>   {self.url.set_scheme(scheme).map_err(|()| SetSchemeError)}
    /// Sets the [`UrlPart::Username`].
    /// # Errors
    /// If the call to [`Url::set_username`] returns an error, returns the error [`SetUsernameError`].
    pub fn set_username     (&mut self, username: &str        ) -> Result<(), SetUsernameError> {self.url.set_username(username).map_err(|()| SetUsernameError)}
    /// Sets the [`UrlPart::Password`].
    /// # Errors
    /// If the call to [`Url::set_password`] returns an error, returns the error [`SetPasswordError`].
    pub fn set_password     (&mut self, password: Option<&str>) -> Result<(), SetPasswordError> {self.url.set_password(password).map_err(|()| SetPasswordError)}
    /// Sets the [`UrlPart::Host`].
    /// # Errors
    /// If the call to [`Url::set_host`] returns an error, the error is returned..
    pub fn set_host         (&mut self, host    : Option<&str>) -> Result<(), ParseError>       {self.url.set_host(host)?; self.host_details = self.url.host().map(|host| HostDetails::from_host(&host)); Ok(())}
    /// Sets the [`UrlPart::Host`].
    /// # Errors
    /// If the call to [`Url::set_ip_host`] returns an error, returns the error [`SetIpHostError`].
    pub fn set_ip_host      (&mut self, address : IpAddr      ) -> Result<(), SetIpHostError>   {self.url.set_ip_host(address).map_err(|()| SetIpHostError)?; self.host_details = self.url.host().map(|host| HostDetails::from_host(&host)); Ok(())}
    /// Sets the [`UrlPart::Port`].
    /// # Errors
    /// If the call to [`Url::set_port`] returns an error, returns the error [`SetPortError`].
    pub fn set_port         (&mut self, port    : Option<u16> ) -> Result<(), SetPortError>     {self.url.set_port(port).map_err(|()| SetPortError)}
    /// [`Url::set_path`].
    pub fn set_path         (&mut self, path    : &str        )                                 {self.url.set_path(path)}
    /// [`Url::set_query`].
    pub fn set_query        (&mut self, query   : Option<&str>)                                 {self.url.set_query(query)}
    /// [`Url::query_pairs_mut`].
    pub fn query_pairs_mut  (&mut self                        ) -> Serializer<'_, UrlQuery<'_>> {self.url.query_pairs_mut()}
    /// [`Url::set_fragment`].
    pub fn set_fragment     (&mut self, fragment: Option<&str>)                                 {self.url.set_fragment(fragment)}



    /// Sets the [`UrlPart::Host`] to a domain.
    ///
    /// Please note that this overwrites the presence of the [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period, and is named accordingly.
    /// # Errors
    /// If the call to [`HostDetails::from_host_str`] returns an error, that error is returned.
    ///
    /// If the call to [`HostDetails::from_host_str`] doesn't return a [`HostDetails::Domain`], returns the error [`SetDomainHostError::ProvidedHostIsNotADomain`].
    pub fn set_domain_host(&mut self, domain: &str) -> Result<(), SetDomainHostError> {
        if let Ok(host_details @ HostDetails::Domain(_)) = HostDetails::from_host_str(domain) {
            self.url.set_host(Some(domain))?;
            self.host_details = Some(host_details);
        } else {
            Err(SetDomainHostError::ProvidedHostIsNotADomain)?;
        }
        Ok(())
    }



    /// Sets [`Self`]'s [`UrlPart::Domain`].
    ///
    /// Please note that if `self`'s domain has a [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period, that period is preserved.
    /// # Errors
    /// If `to` isn't a domain, returns the error [`SetSubdomainError::HostIsNotADomain`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// url.set_domain(Some("example2.com")).unwrap();
    /// assert_eq!(url.domain(), Some("example2.com"));
    ///
    /// // Note the period at the end, marking it a fully qualified domain name.
    /// let mut url = BetterUrl::parse("https://example.com.").unwrap();
    ///
    /// // Note the absence of the period at the end.
    /// assert_eq!(url.domain(), Some("example.com"));
    /// // Note the presence of the period at the end.
    /// assert_eq!(url.host_str(), Some("example.com."));
    /// // Note the passed value not having a period at the end.
    /// url.set_domain(Some("example2.com")).unwrap();
    /// // Note the absence of the period at the end.
    /// assert_eq!(url.domain(), Some("example2.com"));
    /// // Note the continued presence of the period at the end.
    /// assert_eq!(url.host_str(), Some("example2.com."));
    /// ```
    pub fn set_domain(&mut self, to: Option<&str>) -> Result<(), SetDomainError> {
        Ok(match (self.host_details(), to) {
            (Some(HostDetails::Domain(DomainDetails {fqdn_period: Some(_), ..})), Some(to)) => self.set_domain_host(&format!("{to}."))?,
            (_, Some(to)) => self.set_domain_host(to)?,
            (_, None) => self.set_host(to)?
        })
    }

    /// Sets [`Self`]'s [`UrlPart::Subdomain`].
    ///
    /// Please note that if `self`'s domain has a [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period, that period is preserved.
    /// # Errors
    /// If [`Self`]'s host isn't a domain, returns the error [`SetSubdomainError::HostIsNotADomain`].
    ///
    /// If [`Self`] doesn't have a [`UrlPart::RegDomain`], returns the error [`SetSubdomainError::MissingRegDomain`]/
    pub fn set_subdomain(&mut self, to: Option<&str>) -> Result<(), SetSubdomainError> {
        Ok(match self.host_details() {
            #[allow(clippy::useless_format, reason = "Visual consistency.")]
            Some(HostDetails::Domain(domain_details)) => match (to, self.reg_domain(), domain_details.is_fqdn()) {
                (Some(to), Some(rd), false) => self.set_domain_host(&format!("{to}.{rd}"))?,
                (Some(to), Some(rd), true ) => self.set_domain_host(&format!("{to}.{rd}."))?,
                (Some(_ ), None    , false) => Err(SetSubdomainError::MissingRegDomain)?,
                (Some(_ ), None    , true ) => Err(SetSubdomainError::MissingRegDomain)?,
                (None    , Some(rd), false) => self.set_domain_host(&format!("{rd}"))?,
                (None    , Some(rd), true ) => self.set_domain_host(&format!("{rd}."))?,
                (None    , None    , false) => self.set_domain_host(&format!(""))?,
                (None    , None    , true ) => self.set_domain_host(&format!("."))?
            },
            _ => Err(SetSubdomainError::HostIsNotADomain)?
        })
    }

    /// Sets [`Self`]'s [`UrlPart::NotDomainSuffix`].
    ///
    /// Please note that if `self`'s domain has a [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period, that period is preserved.
    /// # Errors
    /// If [`Self`]'s host isn't a domain, returns the error [`SetSubdomainError::HostIsNotADomain`].
    ///
    /// If [`Self`] doesn't have a [`UrlPart::DomainSuffix`], returns the error [`SetNotDomainSuffixError::MissingDomainSuffix`]/
    pub fn set_not_domain_suffix(&mut self, to: Option<&str>) -> Result<(), SetNotDomainSuffixError> {
        Ok(match self.host_details() {
            #[allow(clippy::useless_format, reason = "Visual consistency.")]
            Some(HostDetails::Domain(domain_details)) => match (to, self.domain_suffix(), domain_details.is_fqdn()) {
                (Some(to), Some(su), false) => self.set_domain_host(&format!("{to}.{su}"))?,
                (Some(to), Some(su), true ) => self.set_domain_host(&format!("{to}.{su}."))?,
                (Some(_ ), None    , false) => Err(SetNotDomainSuffixError::MissingDomainSuffix)?,
                (Some(_ ), None    , true ) => Err(SetNotDomainSuffixError::MissingDomainSuffix)?,
                (None    , Some(su), false) => self.set_domain_host(&format!("{su}"))?,
                (None    , Some(su), true ) => self.set_domain_host(&format!("{su}."))?,
                (None    , None    , false) => self.set_domain_host(&format!(""))?,
                (None    , None    , true ) => self.set_domain_host(&format!("."))?
            },
            _ => Err(SetNotDomainSuffixError::HostIsNotADomain)?
        })
    }

    /// Sets [`Self`]'s [`UrlPart::DomainMiddle`].
    ///
    /// Please note that if `self`'s domain has a [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period, that period is preserved.
    /// # Errors
    /// If [`Self`]'s host isn't a domain, returns the error [`SetSubdomainError::HostIsNotADomain`].
    ///
    /// If [`Self`] doesn't have a [`UrlPart::DomainSuffix`], returns the error [`SetDomainMiddleError::MissingDomainSuffix`]/
    pub fn set_domain_middle(&mut self, to: Option<&str>) -> Result<(), SetDomainMiddleError> {
        Ok(match self.host_details() {
            #[allow(clippy::useless_format, reason = "Visual consistency.")]
            Some(HostDetails::Domain(domain_details)) => match (self.subdomain(), to, self.domain_suffix(), domain_details.is_fqdn()) {
                (Some(sd), Some(to), Some(su), false) => self.set_domain_host(&format!("{sd}.{to}.{su}"))?,
                (Some(sd), Some(to), Some(su), true ) => self.set_domain_host(&format!("{sd}.{to}.{su}."))?,
                (Some(_ ), Some(_ ), None    , false) => Err(SetDomainMiddleError::MissingDomainSuffix)?,
                (Some(_ ), Some(_ ), None    , true ) => Err(SetDomainMiddleError::MissingDomainSuffix)?,
                (Some(sd), None    , Some(su), false) => self.set_domain_host(&format!("{sd}.{su}"))?,
                (Some(sd), None    , Some(su), true ) => self.set_domain_host(&format!("{sd}.{su}."))?,
                (Some(sd), None    , None    , false) => self.set_domain_host(&format!("{sd}"))?,
                (Some(sd), None    , None    , true ) => self.set_domain_host(&format!("{sd}."))?,
                (None    , Some(to), Some(su), false) => self.set_domain_host(&format!("{to}.{su}"))?,
                (None    , Some(to), Some(su), true ) => self.set_domain_host(&format!("{to}.{su}."))?,
                (None    , Some(_ ), None    , false) => Err(SetDomainMiddleError::MissingDomainSuffix)?,
                (None    , Some(_ ), None    , true ) => Err(SetDomainMiddleError::MissingDomainSuffix)?,
                (None    , None    , Some(su), false) => self.set_domain_host(&format!("{su}"))?,
                (None    , None    , Some(su), true ) => self.set_domain_host(&format!("{su}."))?,
                (None    , None    , None    , false) => self.set_domain_host(&format!(""))?,
                (None    , None    , None    , true ) => self.set_domain_host(&format!("."))?
            },
            _ => Err(SetDomainMiddleError::HostIsNotADomain)?
        })
    }

    /// Sets [`Self`]'s [`UrlPart::RegDomain`].
    ///
    /// Please note that if `self`'s domain has a [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period, that period is preserved.
    /// # Errors
    /// If [`Self`]'s host isn't a domain, returns the error [`SetRegDomainError::HostIsNotADomain`].
    pub fn set_reg_domain(&mut self, to: Option<&str>) -> Result<(), SetRegDomainError> {
        Ok(match self.host_details() {
            #[allow(clippy::useless_format, reason = "Visual consistency.")]
            Some(HostDetails::Domain(domain_details)) => match (self.subdomain(), to, domain_details.is_fqdn()) {
                (Some(sd), Some(to), false) => self.set_domain_host(&format!("{sd}.{to}"))?,
                (Some(sd), Some(to), true ) => self.set_domain_host(&format!("{sd}.{to}."))?,
                (Some(sd), None    , false) => self.set_domain_host(&format!("{sd}"))?,
                (Some(sd), None    , true ) => self.set_domain_host(&format!("{sd}."))?,
                (None    , Some(to), false) => self.set_domain_host(&format!("{to}"))?,
                (None    , Some(to), true ) => self.set_domain_host(&format!("{to}."))?,
                (None    , None    , false) => self.set_domain_host(&format!(""))?,
                (None    , None    , true ) => self.set_domain_host(&format!("."))?
            },
            _ => Err(SetRegDomainError::HostIsNotADomain)?
        })
    }

    /// Sets [`Self`]'s [`UrlPart::DomainSuffix`].
    ///
    /// Please note that if `self`'s domain has a [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period, that period is preserved.
    /// # Errors
    /// If [`Self`]'s host isn't a domain, returns the error [`SetDomainSuffixError::HostIsNotADomain`].
    pub fn set_domain_suffix(&mut self, to: Option<&str>) -> Result<(), SetDomainSuffixError> {
        Ok(match self.host_details() {
            #[allow(clippy::useless_format, reason = "Visual consistency.")]
            Some(HostDetails::Domain(domain_details)) => match (self.not_domain_suffix(), to, domain_details.is_fqdn()) {
                (Some(ns), Some(to), false) => self.set_domain_host(&format!("{ns}.{to}"))?,
                (Some(ns), Some(to), true ) => self.set_domain_host(&format!("{ns}.{to}."))?,
                (Some(ns), None    , false) => self.set_domain_host(&format!("{ns}"))?,
                (Some(ns), None    , true ) => self.set_domain_host(&format!("{ns}."))?,
                (None    , Some(to), false) => self.set_domain_host(&format!("{to}"))?,
                (None    , Some(to), true ) => self.set_domain_host(&format!("{to}."))?,
                (None    , None    , false) => self.set_domain_host(&format!(""))?,
                (None    , None    , true ) => self.set_domain_host(&format!("."))?
            },
            _ => Err(SetDomainSuffixError::HostIsNotADomain)?
        })
    }
}

impl Deref for BetterUrl {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.url
    }
}

impl PartialEq<BetterUrl> for BetterUrl {
    fn eq(&self, other: &BetterUrl) -> bool {
        self.url == other.url
    }
}

impl Eq for BetterUrl {}

impl PartialOrd for BetterUrl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BetterUrl {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.url.cmp(&other.url)
    }
}

impl std::convert::AsRef<Url> for BetterUrl {
    fn as_ref(&self) -> &Url {
        &self.url
    }
}

impl std::convert::AsRef<str> for BetterUrl {
    fn as_ref(&self) -> &str {
        self.url.as_ref()
    }
}

impl From<BetterUrl> for Url {
    fn from(value: BetterUrl) -> Self {
        value.url
    }
}

impl From<Url> for BetterUrl {
    fn from(value: Url) -> Self {
        Self {
            host_details: value.host().map(|host| HostDetails::from_host(&host)),
            url: value
        }
    }
}

impl std::hash::Hash for BetterUrl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.url, state)
    }
}

impl PartialEq<Url> for BetterUrl {
    fn eq(&self, other: &Url) -> bool {
        (&**self) == other
    }
}

impl FromStr for BetterUrl {
    type Err = <Url as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Url::from_str(s).map(Into::into)
    }
}

impl TryFrom<&str> for BetterUrl {
    type Error = <Self as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}
