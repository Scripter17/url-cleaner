//! A wrapper around [`Url`] that allows for some faster operations.

use std::ops::Bound;
use std::net::IpAddr;
use std::str::FromStr;
use std::ops::Index;

use serde::{Serialize, Deserialize};
use url::{Url, UrlQuery, PathSegmentsMut, ParseError};
use form_urlencoded::Serializer;

use crate::types::*;

/// A wrapper around [`Url`] that allows for some faster operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "Url", into = "Url")]
pub struct BetterUrl {
    /// The contained URL.
    url: Url,
    /// The details of [`Self::url`]'s host.
    host_details: Option<HostDetails>
}

impl BetterUrl {
    /// [`Url::set_fragment`].
    pub fn set_fragment     (&mut self, fragment: Option<&str>)                                    {self.url.set_fragment(fragment)}
    /// [`Url::set_query`].
    pub fn set_query        (&mut self, query   : Option<&str>)                                    {self.url.set_query   (query   )}
    /// [`Url::query_pairs_mut`].
    pub fn query_pairs_mut  (&mut self                        ) -> Serializer<'_, UrlQuery<'_>>    {self.url.query_pairs_mut()     }
    /// [`Url::set_path`].
    pub fn set_path         (&mut self, path    : &str        )                                    {self.url.set_path    (path    )}
    /// [`Url::path_segments_mut`].
    /// # Errors
    /// If the call to [`Url::path_segments_mut`] returns an error, that error is returned.
    #[allow(clippy::result_unit_err, reason = "API compatibility requires this bad API.")]
    pub fn path_segments_mut(&mut self                        ) -> Result<PathSegmentsMut<'_>, ()> {self.url.path_segments_mut()   }
    /// [`Url::set_port`].
    /// # Errors
    /// If the call to [`Url::set_port`] returns an error, that error is returned.
    #[allow(clippy::result_unit_err, reason = "API compatibility requires this bad API.")]
    pub fn set_port         (&mut self, port    : Option<u16> ) -> Result<(), ()>                  {self.url.set_port    (port    )}
    /// [`Url::set_host`].
    /// # Errors
    /// If the call to [`Url::set_host`] returns an error, that error is returned.
    pub fn set_host         (&mut self, host    : Option<&str>) -> Result<(), ParseError>          {self.url.set_host    (host    )?; self.host_details = self.url.host().map(HostDetails::from_host); Ok(())}
    /// [`Url::set_ip_host`].
    /// # Errors
    /// If the call to [`Url::set_ip_host`] returns an error, that error is returned.
    #[allow(clippy::result_unit_err, reason = "API compatibility requires this bad API.")]
    pub fn set_ip_host      (&mut self, address : IpAddr      ) -> Result<(), ()>                  {self.url.set_ip_host (address )?; self.host_details = self.url.host().map(HostDetails::from_host); Ok(())}
    /// [`Url::set_password`].
    /// # Errors
    /// If the call to [`Url::set_password`] returns an error, that error is returned.
    #[allow(clippy::result_unit_err, reason = "API compatibility requires this bad API.")]
    pub fn set_password     (&mut self, password: Option<&str>) -> Result<(), ()>                  {self.url.set_password(password)}
    /// [`Url::set_username`].
    /// # Errors
    /// If the call to [`Url::set_username`] returns an error, that error is returned.
    #[allow(clippy::result_unit_err, reason = "API compatibility requires this bad API.")]
    pub fn set_username     (&mut self, username: &str        ) -> Result<(), ()>                  {self.url.set_username(username)}
    /// [`Url::set_scheme`].
    /// # Errors
    /// If the call to [`Url::set_scheme`] returns an error, that error is returned.
    #[allow(clippy::result_unit_err, reason = "API compatibility requires this bad API.")]
    pub fn set_scheme       (&mut self, scheme  : &str        ) -> Result<(), ()>                  {self.url.set_scheme  (scheme  )}

    /// Sets the [`UrlPart::Subdomain`].
    /// # Errors
    /// If the call to [`Url::set_host`] returns an error, that error is returned.
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't ever happen.")]
    pub fn set_subdomain        (&mut self, to: Option<&str>) -> Result<(), UrlPartSetError> {
        Ok(match self.host_details() {
            #[allow(clippy::useless_format, reason = "Visual consistency.")]
            Some(HostDetails::Domain(domain_details)) => match (to, self.url.host_str().expect(HDA).get(domain_details.not_subdomain_bounds().ok_or(UrlPartSetError::DoesntHaveNotSubdomain)?), domain_details.is_fqdn()) {
                (Some(to), Some(ns), false) => self.set_host(Some(&format!("{to}.{ns}")))?,
                (Some(to), Some(ns), true ) => self.set_host(Some(&format!("{to}.{ns}.")))?,
                (Some(to), None    , false) => self.set_host(Some(&format!("{to}")))?,
                (Some(to), None    , true ) => self.set_host(Some(&format!("{to}.")))?,
                (None    , Some(ns), false) => self.set_host(Some(&format!("{ns}")))?,
                (None    , Some(ns), true ) => self.set_host(Some(&format!("{ns}.")))?,
                (None    , None    , false) => self.set_host(Some(&format!("")))?,
                (None    , None    , true ) => self.set_host(Some(&format!(",")))?,
            }
            _ => Err(UrlPartSetError::HostIsNotADomain)?
        })
    }

    /// Sets the [`UrlPart::NotDomainSuffix`].
    /// # Errors
    /// If the call to [`Url::set_host`] returns an error, that error is returned.
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't ever happen.")]
    pub fn set_not_domain_suffix(&mut self, to: Option<&str>) -> Result<(), UrlPartSetError> {
        Ok(match self.host_details() {
            #[allow(clippy::useless_format, reason = "Visual consistency.")]
            Some(HostDetails::Domain(domain_details)) => match (to, self.url.host_str().expect(HDA).get(domain_details.suffix_bounds()), domain_details.is_fqdn()) {
                (Some(to), Some(su), false) => self.set_host(Some(&format!("{to}.{su}")))?,
                (Some(to), Some(su), true ) => self.set_host(Some(&format!("{to}.{su}.")))?,
                (Some(to), None    , false) => self.set_host(Some(&format!("{to}")))?,
                (Some(to), None    , true ) => self.set_host(Some(&format!("{to}.")))?,
                (None    , Some(su), false) => self.set_host(Some(&format!("{su}")))?,
                (None    , Some(su), true ) => self.set_host(Some(&format!("{su}.")))?,
                (None    , None    , false) => self.set_host(Some(&format!("")))?,
                (None    , None    , true ) => self.set_host(Some(&format!(".")))?
            },
            _ => Err(UrlPartSetError::HostIsNotADomain)?
        })
    }

    /// Sets the [`UrlPart::DomainMiddle`].
    /// # Errors
    /// If the call to [`Url::set_host`] returns an error, that error is returned.
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't ever happen.")]
    pub fn set_domain_middle    (&mut self, to: Option<&str>) -> Result<(), UrlPartSetError> {
        Ok(match self.host_details() {
            #[allow(clippy::useless_format, reason = "Visual consistency.")]
            Some(HostDetails::Domain(domain_details)) => match (domain_details.subdomain_bounds().and_then(|bounds| self.url.host_str().expect(HDA).get(bounds)), to, self.url.host_str().expect(HDA).get(domain_details.suffix_bounds()), domain_details.is_fqdn()) {
                (Some(sd), Some(to), Some(suffix), false) => self.set_host(Some(&format!("{sd}.{to}.{suffix}")))?,
                (Some(sd), Some(to), Some(suffix), true ) => self.set_host(Some(&format!("{sd}.{to}.{suffix}.")))?,
                (Some(sd), Some(to), None        , false) => self.set_host(Some(&format!("{sd}.{to}")))?,
                (Some(sd), Some(to), None        , true ) => self.set_host(Some(&format!("{sd}.{to}.")))?,
                (Some(sd), None    , Some(suffix), false) => self.set_host(Some(&format!("{sd}.{suffix}")))?,
                (Some(sd), None    , Some(suffix), true ) => self.set_host(Some(&format!("{sd}.{suffix}.")))?,
                (Some(sd), None    , None        , false) => self.set_host(Some(&format!("{sd}")))?,
                (Some(sd), None    , None        , true ) => self.set_host(Some(&format!("{sd}.")))?,
                (None    , Some(to), Some(suffix), false) => self.set_host(Some(&format!("{to}.{suffix}")))?,
                (None    , Some(to), Some(suffix), true ) => self.set_host(Some(&format!("{to}.{suffix}.")))?,
                (None    , Some(to), None        , false) => self.set_host(Some(&format!("{to}")))?,
                (None    , Some(to), None        , true ) => self.set_host(Some(&format!("{to}.")))?,
                (None    , None    , Some(suffix), false) => self.set_host(Some(&format!("{suffix}")))?,
                (None    , None    , Some(suffix), true ) => self.set_host(Some(&format!("{suffix}.")))?,
                (None    , None    , None        , false) => self.set_host(Some(&format!("")))?,
                (None    , None    , None        , true ) => self.set_host(Some(&format!(".")))?
            },
            _ => Err(UrlPartSetError::HostIsNotADomain)?
        })
    }

    /// Sets the [`UrlPart::NotSubdomain`].
    /// # Errors
    /// If the call to [`Url::set_host`] returns an error, that error is returned.
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't ever happen.")]
    pub fn set_not_subdomain    (&mut self, to: Option<&str>) -> Result<(), UrlPartSetError> {
        Ok(match self.host_details() {
            #[allow(clippy::useless_format, reason = "Visual consistency.")]
            Some(HostDetails::Domain(domain_details)) => match (domain_details.subdomain_bounds().and_then(|bounds| self.url.host_str().expect(HDA).get(bounds)), to, domain_details.is_fqdn()) {
                (Some(sd), Some(to), false) => self.set_host(Some(&format!("{sd}.{to}")))?,
                (Some(sd), Some(to), true ) => self.set_host(Some(&format!("{sd}.{to}.")))?,
                (Some(sd), None    , false) => self.set_host(Some(&format!("{sd}")))?,
                (Some(sd), None    , true ) => self.set_host(Some(&format!("{sd}.")))?,
                (None    , Some(to), false) => self.set_host(Some(&format!("{to}")))?,
                (None    , Some(to), true ) => self.set_host(Some(&format!("{to}.")))?,
                (None    , None    , false) => self.set_host(Some(&format!("")))?,
                (None    , None    , true ) => self.set_host(Some(&format!(".")))?
            },
            _ => Err(UrlPartSetError::HostIsNotADomain)?
        })
    }

    /// Sets the [`UrlPart::DomainSuffix`].
    /// # Errors
    /// If the call to [`Url::set_host`] returns an error, that error is returned.
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't ever happen.")]
    pub fn set_domain_suffix(&mut self, to: Option<&str>) -> Result<(), UrlPartSetError> {
        Ok(match self.host_details() {
            #[allow(clippy::useless_format, reason = "Visual consistency.")]
            Some(HostDetails::Domain(domain_details)) => match (domain_details.not_suffix_bounds().and_then(|bounds| self.url.host_str().expect(HDA).get(bounds)), to, domain_details.is_fqdn()) {
                (Some(ns), Some(to), false) => self.set_host(Some(&format!("{ns}.{to}")))?,
                (Some(ns), Some(to), true ) => self.set_host(Some(&format!("{ns}.{to}.")))?,
                (Some(ns), None    , false) => self.set_host(Some(&format!("{ns}")))?,
                (Some(ns), None    , true ) => self.set_host(Some(&format!("{ns}.")))?,
                (None    , Some(to), false) => self.set_host(Some(&format!("{to}")))?,
                (None    , Some(to), true ) => self.set_host(Some(&format!("{to}.")))?,
                (None    , None    , false) => self.set_host(Some(&format!("")))?,
                (None    , None    , true ) => self.set_host(Some(&format!(".")))?
            },
            _ => Err(UrlPartSetError::HostIsNotADomain)?
        })
    }

    /// Gets the inner [`Url`].
    pub fn url(&self) -> &Url{
        &self.url
    }

    /// Gets the inner [`HostDetails`].
    pub fn host_details(&self) -> Option<&HostDetails> {
        self.host_details.as_ref()
    }

    /// Gets a mutable reference to the inner [`Url`].
    ///
    /// ALWAYS MAKE SURE THE DETAILS FIELD(S) ARE SYNCED.
    pub(crate) fn inner_mut(&mut self) -> &mut Url {
        &mut self.url
    }

    /// Overwrites [`Self::host_details`].
    ///
    /// ALWAYS MAKE SURE THE DETAILS ARE FOR THE CURRENT HOST.
    pub(crate) fn set_host_details(&mut self, details: Option<HostDetails>) {
        self.host_details = details;
    }

    /// Helper function until [`FromStr`] gets its rightful place in the prelude.
    /// # Errors
    /// If the call to [`Url::parse`] returns an error, that error is returned.
    pub fn parse(value: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::from_str(value)
    }
}

impl PartialEq<BetterUrl> for BetterUrl {
    fn eq(&self, other: &BetterUrl) -> bool {
        self.url == other.url
    }
}

impl Eq for BetterUrl {}

impl std::ops::Deref for BetterUrl {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.url
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

impl<T> Index<T> for BetterUrl where Url: Index<T> {
    type Output = <Url as Index<T>>::Output;
    
    fn index(&self, index: T) -> &<Self as Index<T>>::Output {
        self.url.index(index)
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
            host_details: value.host().map(HostDetails::from_host),
            url: value
        }
    }
}

impl std::hash::Hash for BetterUrl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.url, state)
    }
}

/// Thing to yell in hopefully impossible case.
const HDA: &str = "The host_details being HostDetails::Domain meaning the URL has a host that is a domain.";

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

/// Details for a [`BetterUrl`]'s [`Url`]'s host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostDetails {
    /// Details for a host that is a [`url::Host::Domain`].
    Domain(DomainDetails),
    /// Details for a host that is a [`url::Host::Ipv4`].
    Ipv4(Ipv4Details),
    /// Details for a host that is a [`url::Host::Ipv6`].
    Ipv6(Ipv6Details)
}

impl HostDetails {
    /// Creates a [`Self`] from a [`str`].
    /// # Errors
    /// If the call to [`url::Host::parse`] returns an error, that error is returned.
    pub fn from_host_str(host: &str) -> Result<Self, url::ParseError> {
        url::Host::parse(host).map(Self::from_host)
    }

    /// Creates a [`Self`] from a [`url::Host`] so long as its domain variant is [`AsRef<str>`].
    pub fn from_host<T: AsRef<str>>(host: url::Host<T>) -> Self {
        match host {
            url::Host::Domain(domain) => Self::Domain(DomainDetails::from_domain_str(domain.as_ref())),
            url::Host::Ipv4(_) => Self::Ipv4(Ipv4Details {}),
            url::Host::Ipv6(_) => Self::Ipv6(Ipv6Details {})
        }
    }
}

/// Details of a domain.
/// ```
/// # use url_cleaner::types::*;
///
/// assert_eq!(DomainDetails::from_domain_str("example.com"     ), DomainDetails {subdomain_period: None   , suffix_period: Some( 7), fqdn_period: None    });
/// assert_eq!(DomainDetails::from_domain_str("abc.example.com" ), DomainDetails {subdomain_period: Some(3), suffix_period: Some(11), fqdn_period: None    });
/// assert_eq!(DomainDetails::from_domain_str("abc.example.com."), DomainDetails {subdomain_period: Some(3), suffix_period: Some(11), fqdn_period: Some(15)});
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainDetails {
    /// The location of the `'.'` separating the [`UrlPart::Subdomain`] and [`UrlPart::NotSubdomain`].
    pub subdomain_period: Option<usize>,
    /// The location of the `'.'` separating the [`UrlPart::NotDomainSuffix`] and [`UrlPart::DomainSuffix`].
    pub suffix_period: Option<usize>,
    /// The location of the `'.'` at the end of the domain, marking it as a [fully qualified domain names](https://en.wikipedia.org/wiki/Fully_qualified_domain_name).
    pub fqdn_period: Option<usize>
}

impl DomainDetails {
    /// Creates a [`Self`] from a domain [`str`].
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible")]
    pub fn from_domain_str(domain: &str) -> Self {
        match psl::suffix_str(domain) {
            Some(suffix) => match domain.strip_suffix('.').unwrap_or(domain).strip_suffix(suffix).expect("The domain suffix to be a suffix of the host string.").strip_suffix('.') {
                #[allow(clippy::arithmetic_side_effects, reason = "Shouldn't be possible.")]
                Some(not_suffix) => Self {
                    fqdn_period: domain.ends_with('.').then_some(domain.len() - 1),
                    subdomain_period: not_suffix.rsplit_once('.').map(|(_, middle)| domain.len() - suffix.len() - 1 - middle.len() - 1 - domain.ends_with('.') as usize),
                    suffix_period: Some(domain.len() - suffix.len() - 1 - domain.ends_with('.') as usize)
                },
                None => Self {
                    fqdn_period: domain.ends_with('.').then_some(domain.len()),
                    subdomain_period: None,
                    suffix_period: None
                }
            },
            None => Self {
                fqdn_period: domain.ends_with('.').then_some(domain.len()),
                subdomain_period: None,
                suffix_period: None
            }
        }
    }

    /// Gets the range in the domapin corresponding to [`UrlPart::Subdomain`].
    pub fn subdomain_bounds    (&self) -> Option<(Bound<usize>, Bound<usize>)> {self.subdomain_period.map(|x| (Bound::Unbounded, Bound::Excluded(x)))}
    /// Gets the range in the domapin corresponding to [`UrlPart::NotDomainSuffix`].
    pub fn not_suffix_bounds   (&self) -> Option<(Bound<usize>, Bound<usize>)> {self.suffix_period.map(|x| (Bound::Unbounded, Bound::Excluded(x)))}
    /// Gets the range in the domapin corresponding to [`UrlPart::DomainMiddle`].
    pub fn middle_bounds       (&self) -> Option<(Bound<usize>, Bound<usize>)> {self.suffix_period.map(|x| (exorub(self.subdomain_period), Bound::Excluded(x)))}
    /// Gets the range in the domapin corresponding to [`UrlPart::NotSubdomain`].
    pub fn not_subdomain_bounds(&self) -> Option<(Bound<usize>, Bound<usize>)> {self.suffix_period.map(|_| (exorub(self.subdomain_period), exorub(self.fqdn_period)))}
    /// Gets the range in the domapin corresponding to [`UrlPart::DomainSuffix`].
    pub fn suffix_bounds       (&self) ->        (Bound<usize>, Bound<usize>)  {(exorub(self.suffix_period), exorub(self.fqdn_period))}

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

/// Details of an IPv4 address.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ipv4Details {}

/// Details of an IPv4 address.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ipv6Details {}
