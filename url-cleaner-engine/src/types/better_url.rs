//! A wrapper around [`url::Url`] with extra metadata.

use std::net::IpAddr;
use std::str::{FromStr, Split};
use std::ops::Deref;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use url::{Url, UrlQuery, PathSegmentsMut, ParseError};
use form_urlencoded::Serializer;
use thiserror::Error;

pub mod host_details;
pub use host_details::*;

#[expect(unused_imports, reason = "Used in docs.")]
use crate::types::*;
use crate::util::*;

/// A wrapper around a [`Url`] with extra metadata.
///
/// Currently the only included metadata is a [`HostDetails`], which currently only caches [PSL](https://publicsuffix.org/) information for more efficient [`UrlPart::RegDomain`], [`UrlPart::DomainSuffix`], etc..
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

/// The error [`BetterUrl::set_host`] returns when it fails.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct SetHostError(#[from] pub ParseError);

/// The enum of errors [`BetterUrl::set_domain`] can return.
#[derive(Debug, Error)]
pub enum SetDomainError {
    /// Returned when the resulting value isn't parsable as a domain.
    #[error(transparent)]
    SetDomainHostError(#[from] SetDomainHostError),
    /// Returned when the call to [`BetterUrl::set_host`] returns an error.
    #[error(transparent)]
    SetHostError(#[from] SetHostError)
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
    /// Returned when the resulting value isn't parsable as a domain.
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
    /// Returned when the resulting value isn't parsable as a domain.
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
    /// Returned when the resulting value isn't parsable as a domain.
    #[error(transparent)]
    SetDomainHostError(#[from] SetDomainHostError)
}

/// The enum of errors [`BetterUrl::set_reg_domain`] can return.
#[derive(Debug, Error)]
pub enum SetRegDomainError {
    /// Returned when the [`BetterUrl`]'s host isn't a domain.
    #[error("The host was not a domain.")]
    HostIsNotADomain,
    /// Returned when the resulting value isn't parsable as a domain.
    #[error(transparent)]
    SetDomainHostError(#[from] SetDomainHostError)
}

/// The enum of errors [`BetterUrl::set_domain_suffix`] can return.
#[derive(Debug, Error)]
pub enum SetDomainSuffixError {
    /// Returned when the [`BetterUrl`]'s host isn't a domain.
    #[error("The host was not a domain.")]
    HostIsNotADomain,
    /// Returned when the resulting value isn't parsable as a domain.
    #[error(transparent)]
    SetDomainHostError(#[from] SetDomainHostError)
}

/// The enum of errors [`BetterUrl::set_fqdn`] can return.
#[derive(Debug, Error)]
pub enum SetFqdnPeriodError {
    /// Returned when the URL doesn't have a host.
    #[error("The URL didn't have a host.")]
    NoHost,
    /// Returned when the URL's host isn't a domain.
    #[error("The URL's host wasn't a domain.")]
    HostIsNotADomain
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

/// The enum of errors [`BetterUrl::set_query_param`] can return.
#[derive(Debug, Error)]
pub enum SetQueryParamError {
    /// Returned when a query parameter with the specified index can't be set/created.
    #[error("A query parameter with the specified index could not be set/created.")]
    QueryParamIndexNotFound
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
    /// use url_cleaner_engine::types::*;
    /// let url = BetterUrl::parse("https://example.com").unwrap();
    /// ```
    pub fn parse(value: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::from_str(value)
    }

    /// Get the contained [`HostDetails`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// let url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(0), suffix_start: Some(8), fqdn_period: None})));
    ///
    /// let url = BetterUrl::parse("https://127.0.0.1").unwrap();
    ///
    /// assert_eq!(url.host_details(), Some(HostDetails::Ipv4(Ipv4Details {})));
    ///
    /// let url = BetterUrl::parse("https://[::1]").unwrap();
    ///
    /// assert_eq!(url.host_details(), Some(HostDetails::Ipv6(Ipv6Details {})));
    /// ```
    pub fn host_details(&self) -> Option<HostDetails> {
        self.host_details
    }

    /// If [`Self::host_details`] returns [`HostDetails::Domain`], return it.
    /// ```
    /// use url_cleaner_engine::types::*;
    /// let url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(url.domain_details(), Some(DomainDetails {middle_start: Some(0), suffix_start: Some(8), fqdn_period: None}));
    /// assert_eq!(url.ipv4_details  (), None);
    /// assert_eq!(url.ipv6_details  (), None);
    /// ```
    pub fn domain_details(&self) -> Option<DomainDetails> {
        self.host_details()?.domain_details()
    }
    /// If [`Self::host_details`] returns [`HostDetails::Ipv4`], return it.
    /// ```
    /// use url_cleaner_engine::types::*;
    /// let url = BetterUrl::parse("https://127.0.0.1").unwrap();
    ///
    /// assert_eq!(url.domain_details(), None);
    /// assert_eq!(url.ipv4_details  (), Some(Ipv4Details {}));
    /// assert_eq!(url.ipv6_details  (), None);
    /// ```
    pub fn ipv4_details(&self) -> Option<Ipv4Details> {
        self.host_details()?.ipv4_details()
    }
    /// If [`Self::host_details`] returns [`HostDetails::Ipv6`], return it.
    /// ```
    /// use url_cleaner_engine::types::*;
    /// let url = BetterUrl::parse("https://[::1]").unwrap();
    ///
    /// assert_eq!(url.domain_details(), None);
    /// assert_eq!(url.ipv4_details  (), None);
    /// assert_eq!(url.ipv6_details  (), Some(Ipv6Details {}));
    /// ```
    pub fn ipv6_details(&self) -> Option<Ipv6Details> {
        self.host_details()?.ipv6_details()
    }

    /// [`Url::domain`] but without the [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().domain(), Some("example.com"      ));
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().domain(), Some("example.co.uk"    ));
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().domain(), Some("www.example.com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().domain(), Some("www.example.co.uk"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().domain(), Some("www.example.com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().domain(), Some("www.example.co.uk"));
    /// ```
    pub fn domain(&self) -> Option<&str> {
        self.host_str()?.get(self.domain_details()?.domain_bounds())
    }
    /// If [`Self`] has a [`UrlPart::Subdomain`], return it.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().subdomain(), None);
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().subdomain(), None);
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().subdomain(), Some("www"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().subdomain(), Some("www"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().subdomain(), Some("www"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().subdomain(), Some("www"));
    /// ```
    pub fn subdomain(&self) -> Option<&str> {
        self.host_str()?.get(self.domain_details()?.subdomain_bounds()?)
    }
    /// If [`Self`] has a [`UrlPart::NotDomainSuffix`], return it.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().not_domain_suffix(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().not_domain_suffix(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().not_domain_suffix(), Some("www.example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().not_domain_suffix(), Some("www.example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().not_domain_suffix(), Some("www.example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().not_domain_suffix(), Some("www.example"));
    /// ```
    pub fn not_domain_suffix(&self) -> Option<&str> {
        self.host_str()?.get(self.domain_details()?.not_domain_suffix_bounds()?)
    }
    /// If [`Self`] has a [`UrlPart::DomainMiddle`], return it.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().domain_middle(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().domain_middle(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().domain_middle(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().domain_middle(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().domain_middle(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().domain_middle(), Some("example"));
    /// ```
    pub fn domain_middle(&self) -> Option<&str> {
        self.host_str()?.get(self.domain_details()?.domain_middle_bounds()?)
    }
    /// If [`Self`] has a [`UrlPart::RegDomain`], return it.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().reg_domain(), Some("example.com"  ));
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().reg_domain(), Some("example.co.uk"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().reg_domain(), Some("example.com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().reg_domain(), Some("example.co.uk"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().reg_domain(), Some("example.com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().reg_domain(), Some("example.co.uk"));
    /// ```
    pub fn reg_domain(&self) -> Option<&str> {
        self.host_str()?.get(self.domain_details()?.reg_domain_bounds()?)
    }
    /// If [`Self`] has a [`UrlPart::DomainSuffix`], return it.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().domain_suffix(), Some("com"  ));
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().domain_suffix(), Some("co.uk"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().domain_suffix(), Some("com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().domain_suffix(), Some("co.uk"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().domain_suffix(), Some("com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().domain_suffix(), Some("co.uk"));
    /// ```
    pub fn domain_suffix(&self) -> Option<&str> {
        self.host_str()?.get(self.domain_details()?.domain_suffix_bounds()?)
    }
    /// If [`Self`] is a [fully qualified domain anme](https://en.wikipedia.org/wiki/Fully_qualified_domain_name), returns the FQDN period.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().fqdn_period(), None     );
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().fqdn_period(), None     );
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().fqdn_period(), None     );
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().fqdn_period(), None     );
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().fqdn_period(), Some("."));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().fqdn_period(), Some("."));
    /// ```
    pub fn fqdn_period(&self) -> Option<&str> {
        self.host_str()?.get(self.domain_details()?.fqdn_period?..)
    }

    /// Gets an object that can iterate over the segments of [`Self`]'s path.
    /// # Errors
    /// If the call to [`Url::path_segments`] returns [`None`], returns the error [`UrlDoesNotHavePathSegments`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().path_segments().unwrap().collect::<Vec<_>>(), [""]);
    /// assert_eq!(BetterUrl::parse("https://example.com/a/b/c" ).unwrap().path_segments().unwrap().collect::<Vec<_>>(), ["a", "b", "c"]);
    /// assert_eq!(BetterUrl::parse("https://example.com/a/b/c/").unwrap().path_segments().unwrap().collect::<Vec<_>>(), ["a", "b", "c", ""]);
    /// ```
    pub fn path_segments(&self) -> Result<Split<'_, char>, UrlDoesNotHavePathSegments> {
        self.url.path_segments().ok_or(UrlDoesNotHavePathSegments)
    }
    /// Gets an object that can mutate the segments of [`Self`]'s path.
    /// # Errors
    /// If the call to [`Url::path_segments_mut`] returns an error, returns the error [`UrlDoesNotHavePathSegments`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c/").unwrap();
    ///
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/a/b/c");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/a/b");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/a");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/");
    /// ```
    pub fn path_segments_mut(&mut self) -> Result<PathSegmentsMut<'_>, UrlDoesNotHavePathSegments> {
        debug!(self, BetterUrl::path_segments_mut);
        self.url.path_segments_mut().map_err(|()| UrlDoesNotHavePathSegments)
    }

    /// Sets the [`UrlPart::Scheme`].
    /// # Errors
    /// If the call to [`Url::set_scheme`] returns an error, returns the error [`SetSchemeError`].
    pub fn set_scheme(&mut self, scheme: &str) -> Result<(), SetSchemeError> {
        debug!(self, BetterUrl::set_scheme, scheme);
        self.url.set_scheme(scheme).map_err(|()| SetSchemeError)
    }
    /// Sets the [`UrlPart::Username`].
    /// # Errors
    /// If the call to [`Url::set_username`] returns an error, returns the error [`SetUsernameError`].
    pub fn set_username(&mut self, username: &str) -> Result<(), SetUsernameError> {
        debug!(self, BetterUrl::set_username, username);
        self.url.set_username(username).map_err(|()| SetUsernameError)
    }
    /// Sets the [`UrlPart::Password`].
    /// # Errors
    /// If the call to [`Url::set_password`] returns an error, returns the error [`SetPasswordError`].
    pub fn set_password(&mut self, password: Option<&str>) -> Result<(), SetPasswordError> {
        debug!(self, BetterUrl::set_password , password);
        self.url.set_password(password).map_err(|()| SetPasswordError)
    }
    /// Sets the [`UrlPart::Host`].
    /// # Errors
    /// If the call to [`Url::set_host`] returns an error, the error is returned..
    pub fn set_host(&mut self, host: Option<&str>) -> Result<(), SetHostError> {
        debug!(self, BetterUrl::set_host, host);
        self.url.set_host(host)?;
        self.host_details = self.url.host().map(|host| HostDetails::from_host(&host));
        Ok(())
    }
    /// Sets the [`UrlPart::Host`].
    /// # Errors
    /// If the call to [`Url::set_ip_host`] returns an error, returns the error [`SetIpHostError`].
    pub fn set_ip_host(&mut self, address: IpAddr) -> Result<(), SetIpHostError> {
        debug!(self, BetterUrl::set_ip_host, address);
        self.url.set_ip_host(address).map_err(|()| SetIpHostError)?;
        self.host_details = self.url.host().map(|host| HostDetails::from_host(&host));
        Ok(())
    }
    /// Sets the [`UrlPart::Port`].
    /// # Errors
    /// If the call to [`Url::set_port`] returns an error, returns the error [`SetPortError`].
    pub fn set_port(&mut self, port: Option<u16>) -> Result<(), SetPortError> {
        debug!(self, BetterUrl::set_port, port);
        self.url.set_port(port).map_err(|()| SetPortError)
    }
    /// [`Url::set_path`].
    pub fn set_path(&mut self, path: &str) {
        debug!(self, BetterUrl::set_path, path);
        self.url.set_path(path)
    }
    /// [`Url::set_query`].
    pub fn set_query(&mut self, query: Option<&str>) {
        debug!(self, BetterUrl::set_query, query);
        self.url.set_query(query)
    }
    /// [`Url::query_pairs_mut`].
    pub fn query_pairs_mut(&mut self) -> Serializer<'_, UrlQuery<'_>> {
        debug!(self, BetterUrl::query_pairs_mut);
        self.url.query_pairs_mut()
    }
    /// [`Url::set_fragment`].
    pub fn set_fragment(&mut self, fragment: Option<&str>) {
        debug!(self, BetterUrl::set_fragment, fragment);
        self.url.set_fragment(fragment)
    }
    /// An iterator over query parameters without percent decoding anything.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let url = BetterUrl::parse("https://example.com?a=1&%61=2&a=3&b=%41&%62=%42&b=%43").unwrap();
    ///
    /// let mut raw_query_pairs = url.raw_query_pairs().unwrap();
    ///
    /// assert_eq!(raw_query_pairs.next(), Some(("a"  , Some("1"))));
    /// assert_eq!(raw_query_pairs.next(), Some(("%61", Some("2"))));
    /// assert_eq!(raw_query_pairs.next(), Some(("a"  , Some("3"))));
    /// assert_eq!(raw_query_pairs.next(), Some(("b"  , Some("%41"))));
    /// assert_eq!(raw_query_pairs.next(), Some(("%62", Some("%42"))));
    /// assert_eq!(raw_query_pairs.next(), Some(("b"  , Some("%43"))));
    /// ```
    pub fn raw_query_pairs(&self) -> Option<impl Iterator<Item = (&str, Option<&str>)>> {Some(self.query()?.split('&').map(|kev| kev.split_once('=').map_or((kev, None), |(k, v)| (k, Some(v)))))}
    /// Return [`true`] if [`Self::get_query_param`] would return `Some(Some(_))`, but doesn't do any unnecessary computation.
    ///
    /// For matching, the names are percent decoded. So a `%61=a` query parameter is selectable with a `name` of `a`.
    ///
    /// Please note that this returns [`true`] even if the query param has no value.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let url = BetterUrl::parse("https://example.com?a=1&%61=2&a&%61=4").unwrap();
    ///
    /// assert!( url.has_query_param("a", 0));
    /// assert!( url.has_query_param("a", 1));
    /// assert!( url.has_query_param("a", 2));
    /// assert!( url.has_query_param("a", 3));
    /// assert!(!url.has_query_param("a", 4));
    /// ```
    pub fn has_query_param(&self, name: &str, index: usize) -> bool {
        self.raw_query_pairs().is_some_and(|pairs| pairs.filter(|(x, _)| peh(x) == name).nth(index).is_some())
    }

    /// Get the selected query parameter.
    ///
    /// For matching, the names are percent decoded. So a `%61=a` query parameter is selectable with a `name` of `a`.
    ///
    /// First [`Option`] is if there's a query.
    ///
    /// Second [`Option`] is if there's a query paraeter with the specified name.
    ///
    /// Third [`Option`] is if it has a value.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let url = BetterUrl::parse("https://example.com?a=2&b=3&a=4&c").unwrap();
    ///
    /// assert_eq!(url.get_query_param("a", 0), Some(Some(Some("2".into()))));
    /// assert_eq!(url.get_query_param("a", 1), Some(Some(Some("4".into()))));
    /// assert_eq!(url.get_query_param("a", 2), Some(None));
    /// assert_eq!(url.get_query_param("b", 0), Some(Some(Some("3".into()))));
    /// assert_eq!(url.get_query_param("b", 1), Some(None));
    /// assert_eq!(url.get_query_param("c", 0), Some(Some(None)));
    /// assert_eq!(url.get_query_param("c", 1), Some(None));
    ///
    ///
    /// let url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(url.get_query_param("a", 0), None);
    /// assert_eq!(url.get_query_param("a", 1), None);
    ///
    ///
    /// let url = BetterUrl::parse("https://example.com?a=1&%61=2&a=3&b=%41&%62=%42&b=%43").unwrap();
    ///
    /// assert_eq!(url.get_query_param("a", 0), Some(Some(Some("1".into()))));
    /// assert_eq!(url.get_query_param("a", 1), Some(Some(Some("2".into()))));
    /// assert_eq!(url.get_query_param("a", 2), Some(Some(Some("3".into()))));
    /// assert_eq!(url.get_query_param("b", 0), Some(Some(Some("A".into()))));
    /// assert_eq!(url.get_query_param("b", 1), Some(Some(Some("B".into()))));
    /// assert_eq!(url.get_query_param("b", 2), Some(Some(Some("C".into()))));
    /// ```
    pub fn get_query_param<'a>(&'a self, name: &str, index: usize) -> Option<Option<Option<Cow<'a, str>>>> {
        self.get_raw_query_param(name, index).map(|v| v.map(|v| v.map(|v| peh(v))))
    }
    /// Get the selected query paremeter without percent decoding the value.
    ///
    /// For matching, the names are percent decoded. So a `%61=a` query parameter is selectable with a `name` of `a`.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let url = BetterUrl::parse("https://example.com?a=1&%61=2&a=3&b=%41&%62=%42&b=%43").unwrap();
    ///
    /// assert_eq!(url.get_raw_query_param("a", 0), Some(Some(Some("1"))));
    /// assert_eq!(url.get_raw_query_param("a", 1), Some(Some(Some("2"))));
    /// assert_eq!(url.get_raw_query_param("a", 2), Some(Some(Some("3"))));
    /// assert_eq!(url.get_raw_query_param("b", 0), Some(Some(Some("%41"))));
    /// assert_eq!(url.get_raw_query_param("b", 1), Some(Some(Some("%42"))));
    /// assert_eq!(url.get_raw_query_param("b", 2), Some(Some(Some("%43"))));
    /// ```
    pub fn get_raw_query_param<'a>(&'a self, name: &str, index: usize) -> Option<Option<Option<&'a str>>> {
        self.raw_query_pairs().map(|pairs| pairs.filter(|(x, _)| peh(x) == name).nth(index).map(|(_, v)| v))
    }

    /// Set the selected query parameter.
    ///
    /// For matching, the names are percent decoded. So a `%61=a` query parameter is selectable with a `name` of `a`.
    ///
    /// If there are N query parameters named `name` and `index` is N, appends a new query parameter to the end.
    ///
    /// For performance reasons, resulting empty queries are replaced with [`None`].
    /// # Errors
    /// If `index` is above the number of matched query params, returns the error [`SetQueryParamError::QueryParamIndexNotFound`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// url.set_query_param("a", 0, None).unwrap();
    /// assert_eq!(url.query(), None);
    /// url.set_query_param("a", 0, Some(Some("2"))).unwrap();
    /// assert_eq!(url.query(), Some("a=2"));
    /// url.set_query_param("a", 0, Some(Some("3"))).unwrap();
    /// assert_eq!(url.query(), Some("a=3"));
    /// url.set_query_param("a", 1, Some(Some("4"))).unwrap();
    /// assert_eq!(url.query(), Some("a=3&a=4"));
    /// url.set_query_param("a", 3, Some(Some("5"))).unwrap_err();
    /// assert_eq!(url.query(), Some("a=3&a=4"));
    /// url.set_query_param("a", 0, Some(None)).unwrap();
    /// assert_eq!(url.query(), Some("a&a=4"));
    /// url.set_query_param("a", 0, None).unwrap();
    /// assert_eq!(url.query(), Some("a=4"));
    /// url.set_query_param("a", 0, None).unwrap();
    /// assert_eq!(url.query(), None);
    ///
    /// // Inserting adjacent query params
    /// url.set_query_param("a", 0, Some(Some("2&b=3"))).unwrap();
    /// assert_eq!(url.query(), Some("a=2%26b%3D3"));
    ///
    /// // Setting the fragment
    /// url.set_query_param("a", 0, Some(Some("2#123"))).unwrap();
    /// assert_eq!(url.query(), Some("a=2%23123"));
    /// assert_eq!(url.fragment(), None);
    /// url.set_query_param("a", 0, Some(Some("3"))).unwrap();
    /// assert_eq!(url.query(), Some("a=3"));
    /// assert_eq!(url.fragment(), None);
    ///
    ///
    /// // Empty query optimization.
    /// let mut url = BetterUrl::parse("https://example.com?").unwrap();
    ///
    /// assert_eq!(url.query(), Some(""));
    /// url.set_query_param("a", 0, None).unwrap();
    /// assert_eq!(url.query(), None);
    /// ```
    pub fn set_query_param(&mut self, name: &str, index: usize, to: Option<Option<&str>>) -> Result<(), SetQueryParamError> {
        let to = to.map(|to| to.map(|to| form_urlencoded::byte_serialize(to.as_bytes()).collect::<String>()));
        self.set_raw_query_param(&form_urlencoded::byte_serialize(name.as_bytes()).collect::<String>(), index, to.as_ref().map(|to| to.as_deref()))
    }
    /// Sets the selected query parameter, without ensuring either the name or the value are valid.
    ///
    /// For matching, the names are percent decoded. So a `%61=a` query parameter is selectable with a `name` of `a`.
    ///
    /// If there are N query parameters named `name` and `index` is N, appends a new query parameter to the end.
    ///
    /// For performance reasons, resulting empty queries are replaced with [`None`].
    ///
    /// Useful in combination with [`Self::get_raw_query_param`] for transplanting values without decoding then reencoding them.
    ///
    /// PLEASE note that if `name` and/or `value` contain special characters like `=`, `&`, etc. this will give incoherent results! ONLY use this for directly transplanting from and to query params.
    /// # Errors
    /// If `index` is above the number of matched query params, returns the error [`SetQueryParamError::QueryParamIndexNotFound`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// // Normal use
    /// url.set_raw_query_param("a", 0, None).unwrap();
    /// assert_eq!(url.query(), None);
    /// url.set_raw_query_param("a", 0, Some(Some("2"))).unwrap();
    /// assert_eq!(url.query(), Some("a=2"));
    /// url.set_raw_query_param("a", 0, Some(Some("3"))).unwrap();
    /// assert_eq!(url.query(), Some("a=3"));
    /// url.set_raw_query_param("a", 1, Some(Some("4"))).unwrap();
    /// assert_eq!(url.query(), Some("a=3&a=4"));
    /// url.set_raw_query_param("a", 3, Some(Some("5"))).unwrap_err();
    /// assert_eq!(url.query(), Some("a=3&a=4"));
    /// url.set_raw_query_param("a", 0, Some(None)).unwrap();
    /// assert_eq!(url.query(), Some("a&a=4"));
    /// url.set_raw_query_param("a", 0, None).unwrap();
    /// assert_eq!(url.query(), Some("a=4"));
    /// url.set_raw_query_param("a", 0, None).unwrap();
    /// assert_eq!(url.query(), None);
    ///
    /// // Inserting adjacent query params
    /// url.set_raw_query_param("a", 0, Some(Some("2&b=3"))).unwrap();
    /// assert_eq!(url.query(), Some("a=2&b=3"));
    ///
    /// // Setting the fragment
    /// // Exact behavior, while currently identical to [`Self::set_query_param`], is unspecified.
    /// url.set_raw_query_param("a", 0, Some(Some("2#123"))).unwrap();
    /// assert_eq!(url.query(), Some("a=2%23123&b=3"));
    /// assert_eq!(url.fragment(), None);
    /// url.set_raw_query_param("a", 0, Some(Some("3"))).unwrap();
    /// assert_eq!(url.query(), Some("a=3&b=3"));
    /// assert_eq!(url.fragment(), None);
    ///
    ///
    /// // Empty query optimization.
    /// let mut url = BetterUrl::parse("https://example.com?").unwrap();
    ///
    /// assert_eq!(url.query(), Some(""));
    /// url.set_raw_query_param("a", 0, None).unwrap();
    /// assert_eq!(url.query(), None);
    /// ```
    pub fn set_raw_query_param(&mut self, name: &str, index: usize, to: Option<Option<&str>>) -> Result<(), SetQueryParamError> {
        debug!(self, QueryParamSelector::set, name, index, to);

        let mut pairs = match (self.raw_query_pairs(), to) {
            (Some(x), _      ) => x.collect::<Vec<_>>(),
            (None   , None   ) => return Ok(()),
            (None   , Some(_)) => Vec::new()
        };

        let mut found_matches = 0;
        let mut matched_index = None;

        // Find the index of the selected query parameter and store it in `matched_index`.
        for (i, (x, _)) in pairs.iter().enumerate() {
            if peh(x) == name {
                if found_matches == index {
                    matched_index = Some(i);
                    break;
                }
                #[allow(clippy::arithmetic_side_effects, reason = "Requires usize::MAX query pairs, which is obviously more than can exist.")]
                {found_matches += 1;}
            }
        }

        // Set/remove/append the value.
        match (matched_index, to) {
            #[expect(clippy::indexing_slicing, reason = "`i` is always less than `pairs.len()`. If `pairs.len()` is `0`, `matched_index` is `None`.")]
            (Some(i), Some(to)) => pairs[i].1 = to,
            (Some(i), None    ) => {pairs.remove(i);},
            (None   , Some(to)) => if index == found_matches {
                pairs.push((name, to));
            } else {
                Err(SetQueryParamError::QueryParamIndexNotFound)?
            },
            (None, None) => {}
        }

        // Turn the pairs into a query.
        let mut new = String::new();
        for (k, v) in pairs {
            if !new.is_empty() {new.push('&');}
            new.push_str(k);
            if let Some(v) = v {new.push('='); new.push_str(v);}
        }

        self.set_query(Some(&*new).filter(|x| !x.is_empty()));

        Ok(())
    }

    /// Sets the [`UrlPart::Host`] to a domain.
    ///
    /// Please note that this overwrites the presence of the [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period, and is named accordingly.
    /// # Errors
    /// If the call to [`HostDetails::from_host_str`] returns an error, that error is returned.
    ///
    /// If the call to [`HostDetails::from_host_str`] doesn't return a [`HostDetails::Domain`], returns the error [`SetDomainHostError::ProvidedHostIsNotADomain`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_domain_host("example.com").unwrap(); assert_eq!(url.host_str(), Some("example.com"));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_domain_host("example.com").unwrap(); assert_eq!(url.host_str(), Some("example.com"));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_domain_host("example.com").unwrap(); assert_eq!(url.host_str(), Some("example.com"));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_domain_host("example.com").unwrap(); assert_eq!(url.host_str(), Some("example.com"));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_domain_host("example.com").unwrap(); assert_eq!(url.host_str(), Some("example.com"));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_domain_host("example.com").unwrap(); assert_eq!(url.host_str(), Some("example.com"));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_domain_host("example.com").unwrap(); assert_eq!(url.host_str(), Some("example.com"));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_domain_host("example.com").unwrap(); assert_eq!(url.host_str(), Some("example.com"));
    /// ```
    pub fn set_domain_host(&mut self, domain: &str) -> Result<(), SetDomainHostError> {
        debug!(self, BetterUrl::set_domain_host, domain);
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
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com" ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com" ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com" ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com."));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com."));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com."));
    /// ```
    pub fn set_domain(&mut self, to: Option<&str>) -> Result<(), SetDomainError> {
        Ok(match (self.host_details(), to) {
            (Some(HostDetails::Domain(DomainDetails {fqdn_period: Some(_), ..})), Some(to)) => self.set_domain_host(&format!("{to}."))?,
            _ => self.set_host(to)?
        })
    }

    /// Sets [`Self`]'s [`UrlPart::Subdomain`].
    ///
    /// Please note that if `self`'s domain has a [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period, that period is preserved.
    /// # Errors
    /// If [`Self`]'s host isn't a domain, returns the error [`SetSubdomainError::HostIsNotADomain`].
    ///
    /// If [`Self`] doesn't have a [`UrlPart::RegDomain`], returns the error [`SetSubdomainError::MissingRegDomain`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk."));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk."));
    /// ```
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
    /// If [`Self`] doesn't have a [`UrlPart::DomainSuffix`], returns the error [`SetNotDomainSuffixError::MissingDomainSuffix`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk" ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk."));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk."));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk."));
    /// ```
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
    /// If [`Self`] doesn't have a [`UrlPart::DomainSuffix`], returns the error [`SetDomainMiddleError::MissingDomainSuffix`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(             "com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some(    "example2.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(         "www.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some("www.example2.com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(             "co.uk" ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some(    "example2.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(         "www.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some("www.example2.co.uk" ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(             "com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some(    "example2.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(         "www.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some("www.example2.com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(             "co.uk."));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some(    "example2.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(         "www.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some("www.example2.co.uk."));
    /// ```
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
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_reg_domain(None                 ).unwrap_err();
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(            "com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk" ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_reg_domain(None                 ).unwrap(); assert_eq!(url.host_str(), Some(            "www"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(        "www.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(        "www.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_reg_domain(None                 ).unwrap_err();
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(            "com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk" ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_reg_domain(None                 ).unwrap(); assert_eq!(url.host_str(), Some(            "www"   ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(        "www.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(        "www.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_reg_domain(None                 ).unwrap(); assert_eq!(url.host_str(), Some(               "."  ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(            "com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk."));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_reg_domain(None                 ).unwrap(); assert_eq!(url.host_str(), Some(             "www." ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(        "www.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(        "www.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk."));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_reg_domain(None                 ).unwrap(); assert_eq!(url.host_str(), Some(               "."  ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(            "com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk."));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_reg_domain(None                 ).unwrap(); assert_eq!(url.host_str(), Some(            "www."  ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(        "www.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(        "www.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk."));
    /// ```
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
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some(    "example"       ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some("www.example"       ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some(    "example"       ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some("www.example"       ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some(    "example."      ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some("www.example."      ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk."));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some(    "example."      ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some("www.example."      ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk."));
    /// ```
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

    /// Sets the [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period.
    /// # Errors
    /// If `self` doesn't have a host, returns the error [`SetFqdnPeriodError::NoHost`].
    ///
    /// If the host isn't a domain, returns the error [`SetFqdnPeriodError::HostIsNotADomain`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// url.set_fqdn(false).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com"));
    ///
    /// url.set_fqdn(true).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com."));
    ///
    /// url.set_fqdn(true).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com."));
    ///
    /// url.set_fqdn(false).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com"));
    /// ```
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn set_fqdn(&mut self, to: bool) -> Result<(), SetFqdnPeriodError> {
        match (self.host_details(), to) {
            (Some(HostDetails::Domain(DomainDetails {fqdn_period: None   , ..})), false) => {},
            (Some(HostDetails::Domain(DomainDetails {fqdn_period: None   , ..})), true ) => {self.set_host(Some(&format!("{}.", self.host_str().expect("The URL having a DomainDetails means it has a host.")))).expect("Adding a FQDN period to keep the host valid.")},
            #[expect(clippy::unnecessary_to_owned, reason = "It is necessary.")]
            (Some(HostDetails::Domain(DomainDetails {fqdn_period: Some(_), ..})), false) => {self.set_host(Some(&self.host_str().expect("The URL having a DomainDetails means it has a host.").strip_suffix('.').expect("The URL's DomainDetails::fqdn_period being Some means the host ends with a period.").to_string())).expect("Removing a FQDN period to keep the host valid.")},
            (Some(HostDetails::Domain(DomainDetails {fqdn_period: Some(_), ..})), true ) => {},
            (Some(_), _) => Err(SetFqdnPeriodError::HostIsNotADomain)?,
            (None, _) => Err(SetFqdnPeriodError::NoHost)?
        }

        Ok(())
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

impl std::hash::Hash for BetterUrl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.url, state)
    }
}

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

impl From<Url> for BetterUrl {
    fn from(value: Url) -> Self {
        Self {
            host_details: value.host().map(|host| HostDetails::from_host(&host)),
            url: value
        }
    }
}

impl From<BetterUrl> for Url {
    fn from(value: BetterUrl) -> Self {
        value.url
    }
}

impl From<BetterUrl> for String {
    fn from(value: BetterUrl) -> Self {
        value.url.into()
    }
}

impl PartialEq<Url      > for BetterUrl {fn eq(&self, other: &Url      ) -> bool {(&**self)     ==    other }}
impl PartialEq<String   > for BetterUrl {fn eq(&self, other: &String   ) -> bool {self          == &**other }}
impl PartialEq<str      > for BetterUrl {fn eq(&self, other: &str      ) -> bool {self.as_str() ==    other }}
impl PartialEq<&str     > for BetterUrl {fn eq(&self, other: &&str     ) -> bool {self          ==   *other }}
impl PartialEq<BetterUrl> for Url       {fn eq(&self, other: &BetterUrl) -> bool {other         ==    self  }}
impl PartialEq<BetterUrl> for str       {fn eq(&self, other: &BetterUrl) -> bool {other         ==    self  }}
impl PartialEq<BetterUrl> for &str      {fn eq(&self, other: &BetterUrl) -> bool {*self         ==    other }}
impl PartialEq<BetterUrl> for String    {fn eq(&self, other: &BetterUrl) -> bool {other         ==    self  }}

impl std::fmt::Display for BetterUrl {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.url.fmt(formatter)
    }
}
