//! A common API for getting and setting various parts of [`BetterUrl`]s.

use std::borrow::Cow;
use std::str::FromStr;

use thiserror::Error;
use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// A common API for getting and setting various parts of [`BetterUrl`]s.
///
/// For most parts, setting a URL's part to a value then getting that same part returns the same value.
///
/// Exceptions include setting part segments to values containing the split, `After`/`Before`/`Next` variants always returning [`None`], and probably some other things. I'll fix this doc later.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Suitability)]
pub enum UrlPart {
    /// Print debug information about the contained [`Self`].
    #[suitable(never)]
    Debug(Box<Self>),



    /// The whole URL.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    /// assert_eq!(UrlPart::Whole.get(&url), Some("https://example.com/".into()));
    ///
    /// UrlPart::Whole.set(&mut url, Some("https://example2.com")).unwrap();
    /// assert_eq!(UrlPart::Whole.get(&url), Some("https://example2.com/".into()));
    /// assert_eq!(url.as_str(), "https://example2.com/");
    /// ```
    Whole,



    /// The scheme.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    /// assert_eq!(UrlPart::Scheme.get(&url), Some("https".into()));
    /// UrlPart::Scheme.set(&mut url, Some("http")).unwrap();
    /// assert_eq!(UrlPart::Scheme.get(&url), Some("http".into()));
    /// assert_eq!(url.as_str(), "http://example.com/");
    /// ```
    Scheme,
    /// The username.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    /// assert_eq!(UrlPart::Username.get(&url), Some("".into()));
    ///
    /// UrlPart::Username.set(&mut url, Some("admin")).unwrap();
    /// assert_eq!(UrlPart::Username.get(&url), Some("admin".into()));
    /// assert_eq!(url.as_str(), "https://admin@example.com/");
    /// ```
    Username,
    /// The username.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    /// assert_eq!(UrlPart::Password.get(&url), None);
    ///
    /// UrlPart::Password.set(&mut url, Some("password")).unwrap();
    /// assert_eq!(UrlPart::Password.get(&url), Some("password".into()));
    /// assert_eq!(url.as_str(), "https://:password@example.com/");
    /// ```
    Password,



    /// The host.
    ///
    /// Please note that for [fully qualified domain names](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) this includes the ending `.`.
    Host,
    /// [`Self::Host`] but with the `www.` prefix (if it's there) and `.` suffix (if it's there) removed.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://www.www.example.com").unwrap();
    /// assert_eq!(UrlPart::NormalizedHost.get(&url), Some("www.example.com".into()));
    ///
    /// UrlPart::NormalizedHost.set(&mut url, Some("www.example.com")).unwrap();
    /// assert_eq!(UrlPart::NormalizedHost.get(&url), Some("example.com".into()));
    ///
    /// assert_eq!(url.host_str(), Some("example.com"));
    ///
    /// let mut url = BetterUrl::parse("https://www.www.example.com.").unwrap();
    /// assert_eq!(UrlPart::NormalizedHost.get(&url), Some("www.example.com".into()));
    /// ```
    NormalizedHost,



    /// The nth domain segment of the [`Self::Domain`].
    /// # Footguns
    /// While you are able and, per the URL spec, I think allowed, to add empty segments (`Some("")`), this results in weird and unpredictable behavior.
    ///
    /// Thoroughly preventing empty domain segments is a pain so I decided not to.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(UrlPart::DomainSegment(0).get(&url), Some("example".into()));
    /// assert_eq!(UrlPart::DomainSegment(1).get(&url), Some("com".into()));
    /// assert_eq!(UrlPart::DomainSegment(2).get(&url), None);
    ///
    /// UrlPart::DomainSegment(0).set(&mut url, Some("a")).unwrap();
    /// UrlPart::DomainSegment(1).set(&mut url, Some("b")).unwrap();
    /// UrlPart::DomainSegment(2).set(&mut url, Some("c")).unwrap_err();
    ///
    /// assert_eq!(url.host_str(), Some("a.b"));
    ///
    /// // Fully qualified domain names give the same results.
    /// let mut url = BetterUrl::parse("https://example.com.").unwrap();
    ///
    /// assert_eq!(UrlPart::DomainSegment(0).get(&url), Some("example".into()));
    /// assert_eq!(UrlPart::DomainSegment(1).get(&url), Some("com".into()));
    /// assert_eq!(UrlPart::DomainSegment(2).get(&url), None);
    ///
    /// UrlPart::DomainSegment(0).set(&mut url, Some("a")).unwrap();
    /// UrlPart::DomainSegment(1).set(&mut url, Some("b")).unwrap();
    /// UrlPart::DomainSegment(2).set(&mut url, Some("c")).unwrap_err();
    /// ```
    DomainSegment(isize),
    /// The position in [`Self::Domain`] between the nth domain segment and the previous one.
    ///
    /// Allows inserting domain segments between others.
    /// # Footguns
    /// While you are able and, per the URL spec, I think allowed, to add empty segments (`Some("")`), this results in weird and unpredictable behavior.
    ///
    /// Thoroughly preventing empty domain segments is a pain so I decided not to.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(UrlPart::BeforeDomainSegment(0).get(&url), None);
    ///
    /// UrlPart::BeforeDomainSegment(0).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com"));
    /// UrlPart::BeforeDomainSegment(0).set(&mut url, Some("www")).unwrap();
    /// assert_eq!(url.host_str(), Some("www.example.com"));
    /// // If there's no fourth domain segment, it doesn't make sense to set anything before it.
    /// UrlPart::BeforeDomainSegment(3).set(&mut url, Some("www")).unwrap_err();
    /// assert_eq!(url.host_str(), Some("www.example.com"));
    ///
    /// // Fully qualified domain names give the same results.
    /// let mut url = BetterUrl::parse("https://example.com.").unwrap();
    ///
    /// // The FQDN period isn't a domain segment delimiter.
    /// UrlPart::BeforeDomainSegment(3).set(&mut url, Some("abc")).unwrap_err();
    /// assert_eq!(url.host_str(), Some("example.com."));
    /// ```
    BeforeDomainSegment(isize),
    /// The position in [`Self::Domain`] between the nth domain segment and the next one.
    ///
    /// Allows inserting domain segments between others.
    /// # Footguns
    /// While you are able and, per the URL spec, I think allowed, to add empty segments (`Some("")`), this results in weird and unpredictable behavior.
    ///
    /// Thoroughly preventing empty domain segments is a pain so I decided not to.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(UrlPart::AfterDomainSegment(0).get(&url), None);
    ///
    /// UrlPart::AfterDomainSegment(0).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com"));
    /// UrlPart::AfterDomainSegment(0).set(&mut url, Some("www")).unwrap();
    /// assert_eq!(url.host_str(), Some("example.www.com"));
    /// // You can append a segment after the last one.
    /// UrlPart::AfterDomainSegment(2).set(&mut url, Some("www")).unwrap();
    /// assert_eq!(url.host_str(), Some("example.www.com.www"));
    /// // If there's no fourth domain segment, it doesn't make sense to set anything before it.
    /// UrlPart::AfterDomainSegment(4).set(&mut url, Some("www")).unwrap_err();
    /// assert_eq!(url.host_str(), Some("example.www.com.www"));
    ///
    /// // Fully qualified domain names give the same results.
    /// let mut url = BetterUrl::parse("https://example.com.").unwrap();
    ///
    /// // The FQDN period isn't a domain segment delimiter.
    /// UrlPart::AfterDomainSegment(2).set(&mut url, Some("abc")).unwrap_err();
    /// assert_eq!(url.host_str(), Some("example.com."));
    /// ```
    AfterDomainSegment(isize),



    /// The nth segment of the [`Self::Subdomain`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://abc.def.example.co.uk").unwrap();
    ///
    /// assert_eq!(UrlPart::SubdomainSegment( 0).get(&url), Some("abc".into()));
    /// assert_eq!(UrlPart::SubdomainSegment( 1).get(&url), Some("def".into()));
    /// assert_eq!(UrlPart::SubdomainSegment( 2).get(&url), None              );
    /// assert_eq!(UrlPart::SubdomainSegment(-1).get(&url), Some("def".into()));
    /// assert_eq!(UrlPart::SubdomainSegment(-2).get(&url), Some("abc".into()));
    /// assert_eq!(UrlPart::SubdomainSegment(-3).get(&url), None              );
    ///
    /// UrlPart::SubdomainSegment(0).set(&mut url, Some("123")).unwrap();
    /// assert_eq!(url.host_str(), Some("123.def.example.co.uk"));
    ///
    /// UrlPart::SubdomainSegment(1).set(&mut url, Some("456")).unwrap();
    /// assert_eq!(url.host_str(), Some("123.456.example.co.uk"));
    ///
    /// UrlPart::SubdomainSegment(2).set(&mut url, Some("789")).unwrap_err();
    /// assert_eq!(url.host_str(), Some("123.456.example.co.uk"));
    ///
    ///
    /// UrlPart::SubdomainSegment(-1).set(&mut url, Some("abc")).unwrap();
    /// assert_eq!(url.host_str(), Some("123.abc.example.co.uk"));
    ///
    /// UrlPart::SubdomainSegment(-2).set(&mut url, Some("def")).unwrap();
    /// assert_eq!(url.host_str(), Some("def.abc.example.co.uk"));
    ///
    /// UrlPart::SubdomainSegment(-3).set(&mut url, Some("ghi")).unwrap_err();
    /// assert_eq!(url.host_str(), Some("def.abc.example.co.uk"));
    ///
    ///
    /// UrlPart::SubdomainSegment(-1).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("def.example.co.uk"));
    ///
    /// UrlPart::SubdomainSegment(0).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("example.co.uk"));
    /// ```
    SubdomainSegment(isize),
    /// The position in [`Self::Subdomain`] between the nth segment and the previous one.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://abc.def.example.co.uk").unwrap();
    ///
    /// assert_eq!(UrlPart::BeforeSubdomainSegment( 0).get(&url), None);
    /// assert_eq!(UrlPart::BeforeSubdomainSegment( 1).get(&url), None);
    /// assert_eq!(UrlPart::BeforeSubdomainSegment( 2).get(&url), None);
    /// assert_eq!(UrlPart::BeforeSubdomainSegment(-1).get(&url), None);
    /// assert_eq!(UrlPart::BeforeSubdomainSegment(-2).get(&url), None);
    /// assert_eq!(UrlPart::BeforeSubdomainSegment(-3).get(&url), None);
    ///
    /// UrlPart::BeforeSubdomainSegment(0).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("abc.def.example.co.uk"));
    /// UrlPart::BeforeSubdomainSegment(1).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("abc.def.example.co.uk"));
    /// UrlPart::BeforeSubdomainSegment(2).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("abc.def.example.co.uk"));
    ///
    /// UrlPart::BeforeSubdomainSegment(0).set(&mut url, Some("ghi")).unwrap();
    /// assert_eq!(url.host_str(), Some("ghi.abc.def.example.co.uk"));
    /// UrlPart::BeforeSubdomainSegment(3).set(&mut url, Some("ghi")).unwrap_err();
    /// assert_eq!(url.host_str(), Some("ghi.abc.def.example.co.uk"));
    /// ```
    BeforeSubdomainSegment(isize),
    /// The position in [`Self::Subdomain`] between the nth segment and the next one.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://abc.def.example.co.uk").unwrap();
    ///
    /// assert_eq!(UrlPart::AfterSubdomainSegment( 0).get(&url), None);
    /// assert_eq!(UrlPart::AfterSubdomainSegment( 1).get(&url), None);
    /// assert_eq!(UrlPart::AfterSubdomainSegment( 2).get(&url), None);
    /// assert_eq!(UrlPart::AfterSubdomainSegment(-1).get(&url), None);
    /// assert_eq!(UrlPart::AfterSubdomainSegment(-2).get(&url), None);
    /// assert_eq!(UrlPart::AfterSubdomainSegment(-3).get(&url), None);
    ///
    /// UrlPart::AfterSubdomainSegment(0).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("abc.def.example.co.uk"));
    /// UrlPart::AfterSubdomainSegment(1).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("abc.def.example.co.uk"));
    /// UrlPart::AfterSubdomainSegment(2).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("abc.def.example.co.uk"));
    ///
    /// UrlPart::AfterSubdomainSegment(0).set(&mut url, Some("ghi")).unwrap();
    /// assert_eq!(url.host_str(), Some("abc.ghi.def.example.co.uk"));
    /// UrlPart::AfterSubdomainSegment(3).set(&mut url, Some("ghi")).unwrap_err();
    /// assert_eq!(url.host_str(), Some("abc.ghi.def.example.co.uk"));
    /// ```
    AfterSubdomainSegment(isize),
    /// The nth segment of the [`Self::DomainSuffix`].
    DomainSuffixSegment(isize),
    /// The position in [`Self::DomainSuffix`] between the nth segment and the previous one.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://abc.def.example.co.uk").unwrap();
    ///
    /// assert_eq!(UrlPart::BeforeDomainSuffixSegment( 0).get(&url), None);
    /// assert_eq!(UrlPart::BeforeDomainSuffixSegment( 1).get(&url), None);
    /// assert_eq!(UrlPart::BeforeDomainSuffixSegment( 2).get(&url), None);
    /// assert_eq!(UrlPart::BeforeDomainSuffixSegment(-1).get(&url), None);
    /// assert_eq!(UrlPart::BeforeDomainSuffixSegment(-2).get(&url), None);
    /// assert_eq!(UrlPart::BeforeDomainSuffixSegment(-3).get(&url), None);
    ///
    /// UrlPart::BeforeDomainSuffixSegment(0).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("abc.def.example.co.uk"));
    /// UrlPart::BeforeDomainSuffixSegment(1).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("abc.def.example.co.uk"));
    /// UrlPart::BeforeDomainSuffixSegment(2).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("abc.def.example.co.uk"));
    ///
    /// UrlPart::BeforeDomainSuffixSegment(0).set(&mut url, Some("ghi")).unwrap();
    /// assert_eq!(url.host_str(), Some("abc.def.example.ghi.co.uk"));
    /// UrlPart::BeforeDomainSuffixSegment(3).set(&mut url, Some("ghi")).unwrap_err();
    /// assert_eq!(url.host_str(), Some("abc.def.example.ghi.co.uk"));
    /// ```
    BeforeDomainSuffixSegment(isize),
    /// The position in [`Self::DomainSuffix`] between the nth segment and the next one.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://abc.def.example.co.uk").unwrap();
    ///
    /// assert_eq!(UrlPart::AfterDomainSuffixSegment( 0).get(&url), None);
    /// assert_eq!(UrlPart::AfterDomainSuffixSegment( 1).get(&url), None);
    /// assert_eq!(UrlPart::AfterDomainSuffixSegment( 2).get(&url), None);
    /// assert_eq!(UrlPart::AfterDomainSuffixSegment(-1).get(&url), None);
    /// assert_eq!(UrlPart::AfterDomainSuffixSegment(-2).get(&url), None);
    /// assert_eq!(UrlPart::AfterDomainSuffixSegment(-3).get(&url), None);
    ///
    /// UrlPart::AfterDomainSuffixSegment(0).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("abc.def.example.co.uk"));
    /// UrlPart::AfterDomainSuffixSegment(1).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("abc.def.example.co.uk"));
    /// UrlPart::AfterDomainSuffixSegment(2).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("abc.def.example.co.uk"));
    ///
    /// UrlPart::AfterDomainSuffixSegment(0).set(&mut url, Some("ghi")).unwrap();
    /// assert_eq!(url.host_str(), Some("abc.def.example.co.ghi.uk"));
    /// UrlPart::AfterDomainSuffixSegment(3).set(&mut url, Some("ghi")).unwrap_err();
    /// assert_eq!(url.host_str(), Some("abc.def.example.co.ghi.uk"));
    /// ```
    AfterDomainSuffixSegment(isize),



    /// The host if it's a domain, *not* including the [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period, if it's present.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://www.example.com").unwrap();
    ///
    /// assert_eq!(url.host_str(), Some("www.example.com"));
    /// assert_eq!(UrlPart::Domain.get(&url), Some("www.example.com".into()));
    /// UrlPart::Domain.set(&mut url, Some("example2.com")).unwrap();
    /// assert_eq!(url.host_str(), Some("example2.com"));
    /// assert_eq!(UrlPart::Domain.get(&url), Some("example2.com".into()));
    ///
    /// // Fully qualified domain names give the same results.
    /// let mut url = BetterUrl::parse("https://www.example.com.").unwrap();
    ///
    /// assert_eq!(url.host_str(), Some("www.example.com."));
    /// assert_eq!(UrlPart::Domain.get(&url), Some("www.example.com".into()));
    /// UrlPart::Domain.set(&mut url, Some("example2.com")).unwrap();
    /// assert_eq!(url.host_str(), Some("example2.com."));
    /// assert_eq!(UrlPart::Domain.get(&url), Some("example2.com".into()));
    /// ```
    Domain,
    /// The subdomain of the [`Self::Domain`].
    ///
    /// Specifically, all domain segments prior to [`Self::RegDomain`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://www.example.com").unwrap();
    ///
    /// assert_eq!(url.host_str(), Some("www.example.com"));
    /// assert_eq!(UrlPart::Subdomain.get(&url), Some("www".into()));
    /// UrlPart::Subdomain.set(&mut url, Some("somethingelse")).unwrap();
    /// assert_eq!(url.host_str(), Some("somethingelse.example.com"));
    /// assert_eq!(UrlPart::Subdomain.get(&url), Some("somethingelse".into()));
    ///
    /// // Fully qualified domain names give the same results.
    /// let mut url = BetterUrl::parse("https://www.example.com.").unwrap();
    ///
    /// assert_eq!(url.host_str(), Some("www.example.com."));
    /// assert_eq!(UrlPart::Subdomain.get(&url), Some("www".into()));
    /// UrlPart::Subdomain.set(&mut url, Some("somethingelse")).unwrap();
    /// assert_eq!(url.host_str(), Some("somethingelse.example.com."));
    /// assert_eq!(UrlPart::Subdomain.get(&url), Some("somethingelse".into()));
    /// ```
    Subdomain,
    /// The registerable domain of the [`Self::Domain`].
    ///
    /// Specifically, the [`Self::DomainMiddle`] plus [`Self::DomainSuffix`].
    ///
    /// Does not include [`Self::FqdnPeriod`], even though the [PSL algorithm specifies it should](https://github.com/publicsuffix/list/wiki/Format#note-3).
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://www.example.com").unwrap();
    ///
    /// assert_eq!(url.host_str(), Some("www.example.com"));
    /// assert_eq!(UrlPart::RegDomain.get(&url), Some("example.com".into()));
    /// UrlPart::RegDomain.set(&mut url, Some("example2.com")).unwrap();
    /// assert_eq!(url.host_str(), Some("www.example2.com"));
    /// assert_eq!(UrlPart::RegDomain.get(&url), Some("example2.com".into()));
    ///
    /// // Fully qualified domain names give the same results.
    /// let mut url = BetterUrl::parse("https://www.example.com.").unwrap();
    ///
    /// assert_eq!(url.host_str(), Some("www.example.com."));
    /// assert_eq!(UrlPart::RegDomain.get(&url), Some("example.com".into()));
    /// UrlPart::RegDomain.set(&mut url, Some("example2.com")).unwrap();
    /// assert_eq!(url.host_str(), Some("www.example2.com."));
    /// assert_eq!(UrlPart::RegDomain.get(&url), Some("example2.com".into()));
    /// ```
    RegDomain,
    /// [`Self::Domain`] without [`Self::DomainSuffix`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://www.example.com").unwrap();
    ///
    /// assert_eq!(url.host_str(), Some("www.example.com"));
    /// assert_eq!(UrlPart::NotDomainSuffix.get(&url), Some("www.example".into()));
    /// UrlPart::NotDomainSuffix.set(&mut url, Some("example2")).unwrap();
    /// assert_eq!(url.host_str(), Some("example2.com"));
    /// assert_eq!(UrlPart::NotDomainSuffix.get(&url), Some("example2".into()));
    ///
    /// // Fully qualified domain names give the same results.
    /// let mut url = BetterUrl::parse("https://www.example.com.").unwrap();
    ///
    /// assert_eq!(url.host_str(), Some("www.example.com."));
    /// assert_eq!(UrlPart::NotDomainSuffix.get(&url), Some("www.example".into()));
    /// UrlPart::NotDomainSuffix.set(&mut url, Some("example2")).unwrap();
    /// assert_eq!(url.host_str(), Some("example2.com."));
    /// assert_eq!(UrlPart::NotDomainSuffix.get(&url), Some("example2".into()));
    /// ```
    NotDomainSuffix,
    /// The domain segment right before [`Self::DomainSuffix`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://www.example.com").unwrap();
    ///
    /// assert_eq!(url.host_str(), Some("www.example.com"));
    /// assert_eq!(UrlPart::DomainMiddle.get(&url), Some("example".into()));
    /// UrlPart::DomainMiddle.set(&mut url, Some("example2")).unwrap();
    /// assert_eq!(url.host_str(), Some("www.example2.com"));
    /// assert_eq!(UrlPart::DomainMiddle.get(&url), Some("example2".into()));
    ///
    /// // Fully qualified domain names give the same results.
    /// let mut url = BetterUrl::parse("https://www.example.com.").unwrap();
    ///
    /// assert_eq!(url.host_str(), Some("www.example.com."));
    /// assert_eq!(UrlPart::DomainMiddle.get(&url), Some("example".into()));
    /// UrlPart::DomainMiddle.set(&mut url, Some("example2")).unwrap();
    /// assert_eq!(url.host_str(), Some("www.example2.com."));
    /// assert_eq!(UrlPart::DomainMiddle.get(&url), Some("example2".into()));
    /// ```
    DomainMiddle,
    /// The suffix of the domain, as defined by Mozilla's [Public Suffix List](https://publicsuffix.org/).
    ///
    /// Basically a standard that treats `.co.uk` the same as `.com`.
    ///
    /// Does not include [`Self::FqdnPeriod`], even though the [PSL algorithm specifies it should](https://github.com/publicsuffix/list/wiki/Format#note-3).
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://www.example.com").unwrap();
    ///
    /// assert_eq!(url.host_str(), Some("www.example.com"));
    /// assert_eq!(UrlPart::DomainSuffix.get(&url), Some("com".into()));
    /// UrlPart::DomainSuffix.set(&mut url, Some("co.uk")).unwrap();
    /// assert_eq!(url.host_str(), Some("www.example.co.uk"));
    /// assert_eq!(UrlPart::DomainSuffix.get(&url), Some("co.uk".into()));
    ///
    /// // Fully qualified domain names give the same results.
    /// let mut url = BetterUrl::parse("https://www.example.com.").unwrap();
    ///
    /// assert_eq!(url.host_str(), Some("www.example.com."));
    /// assert_eq!(UrlPart::DomainSuffix.get(&url), Some("com".into()));
    /// UrlPart::DomainSuffix.set(&mut url, Some("co.uk")).unwrap();
    /// assert_eq!(url.host_str(), Some("www.example.co.uk."));
    /// assert_eq!(UrlPart::DomainSuffix.get(&url), Some("co.uk".into()));
    /// ```
    DomainSuffix,
    /// The [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period.
    /// # Errors
    /// If trying to set [`Self::FqdnPeriod`] to any value other than [`None`] and [`Some`]`(".")`, returns the error [`UrlPartSetError::FqdnPeriodMustBeNoneOrPeriod`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(url.host_str(), Some("example.com"));
    /// assert_eq!(UrlPart::FqdnPeriod.get(&url), None);
    ///
    /// UrlPart::FqdnPeriod.set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com"));
    ///
    /// UrlPart::FqdnPeriod.set(&mut url, Some(".")).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com."));
    ///
    /// UrlPart::FqdnPeriod.set(&mut url, Some("thingelse")).unwrap_err();
    /// assert_eq!(url.host_str(), Some("example.com."));
    ///
    /// UrlPart::FqdnPeriod.set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com"));
    /// ```
    FqdnPeriod,



    /// The port.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(UrlPart::Port.get(&url), None);
    ///
    /// UrlPart::Port.set(&mut url, Some("443")).unwrap();
    /// assert_eq!(UrlPart::Port.get(&url), None);
    ///
    /// UrlPart::Port.set(&mut url, Some("80")).unwrap();
    /// assert_eq!(UrlPart::Port.get(&url), Some("80".into()));
    /// ```
    Port,



    /// The path.
    /// # Getting
    /// If the URL is a [cannot-be-a-base](https://docs.rs/url/latest/url/struct.Url.html#method.cannot_be_a_base) URL, this returns "an arbitrary string that doesn't start with `/`".
    ///
    /// For other (most) URLs, this returns the path as expected with the leading `/`.
    /// # Setting
    /// If the URL is a [cannot-be-a-base](https://docs.rs/url/latest/url/struct.Url.html#method.cannot_be_a_base) URL, a leading `/` is turned into `%2F`.
    ///
    /// For other (most) URLs, this first ensures the value starts with `/` (`abc` -> `/abc`, `/def` -> `/def`) then sets the URL's path to that value.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c").unwrap();
    ///
    /// assert_eq!(UrlPart::Path.get(&url), Some("/a/b/c".into()));
    ///
    /// UrlPart::Path.set(&mut url, Some("abc")).unwrap();
    /// assert_eq!(UrlPart::Path.get(&url), Some("/abc".into()));
    ///
    /// UrlPart::Path.set(&mut url, Some("/def")).unwrap();
    /// assert_eq!(UrlPart::Path.get(&url), Some("/def".into()));
    /// ```
    Path,
    /// The nth path segment, ignoring the leading slash.
    ///
    /// Please note that a path like `/a/b/c/` has the path segments `["a", "b", "c", ""]`.
    /// # Getting
    /// If the URL is a [cannot-be-a-base](https://docs.rs/url/latest/url/struct.Url.html#method.cannot_be_a_base) URL, this always returns [`None`].
    /// # Setting
    /// If the URL is a [cannot-be-a-base](https://docs.rs/url/latest/url/struct.Url.html#method.cannot_be_a_base) URL, this always returns the error [`UrlPartSetError::UrlDoesNotHavePathSegments`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c").unwrap();
    ///
    /// assert_eq!(UrlPart::PathSegment(0).get(&url), Some("a".into()));
    /// assert_eq!(UrlPart::PathSegment(1).get(&url), Some("b".into()));
    /// assert_eq!(UrlPart::PathSegment(2).get(&url), Some("c".into()));
    /// assert_eq!(UrlPart::PathSegment(3).get(&url), None);
    ///
    /// UrlPart::PathSegment(0).set(&mut url, Some("A")).unwrap();
    /// UrlPart::PathSegment(1).set(&mut url, Some("B")).unwrap();
    /// UrlPart::PathSegment(2).set(&mut url, Some("C")).unwrap();
    /// UrlPart::PathSegment(3).set(&mut url, Some("D")).unwrap_err();
    /// UrlPart::PathSegment(3).set(&mut url, None     ).unwrap();
    ///
    /// UrlPart::PathSegment(0).set(&mut url, None).unwrap();
    ///
    /// assert_eq!(url.path(), "/B/C");
    /// ```
    PathSegment(isize),
    /// The path segment between the nth and the previous one.
    ///
    /// Please note that a path like `/a/b/c/` has the path segments `["a", "b", "c", ""]`.
    /// # Getting
    /// Always [`None`].
    /// # Setting
    /// If set to [`None`], does nothing.
    ///
    /// If set to [`Some`], inserts a new path segment between the nth and the previous one.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c").unwrap();
    ///
    /// assert_eq!(UrlPart::BeforePathSegment(0).get(&url), None);
    ///
    /// UrlPart::BeforePathSegment(0).set(&mut url, None).unwrap();
    /// assert_eq!(url.path(), "/a/b/c");
    ///
    /// UrlPart::BeforePathSegment(0).set(&mut url, Some("d")).unwrap();
    /// assert_eq!(url.path(), "/d/a/b/c");
    ///
    /// UrlPart::BeforePathSegment(4).set(&mut url, Some("e")).unwrap_err();
    /// ```
    BeforePathSegment(isize),
    /// The path segment between the nth and the next one.
    ///
    /// Please note that a path like `/a/b/c/` has the path segments `["a", "b", "c", ""]`.
    ///
    /// If you want to insert a new path segment at the end but replace the trailing empty segment, use [`Self::NextPathSegment`].
    /// # Getting
    /// Always [`None`].
    /// # Setting
    /// If set to [`None`], does nothing.
    ///
    /// If set to [`Some`], inserts a new path segment between the nth and the next one.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c").unwrap();
    ///
    /// assert_eq!(UrlPart::AfterPathSegment(0).get(&url), None);
    ///
    /// UrlPart::AfterPathSegment(0).set(&mut url, None).unwrap();
    /// assert_eq!(url.path(), "/a/b/c");
    ///
    /// UrlPart::AfterPathSegment(0).set(&mut url, Some("d")).unwrap();
    /// assert_eq!(url.path(), "/a/d/b/c");
    ///
    /// UrlPart::AfterPathSegment(4).set(&mut url, Some("e")).unwrap_err();
    /// assert_eq!(url.path(), "/a/d/b/c");
    /// ```
    AfterPathSegment(isize),
    /// Effectively [`Self::AfterPathSegment`] with the [`Self::AfterPathSegment::0`] being the index of the last [`Self::PathSegment`].
    ///
    /// Please note that a path like `/a/b/c/` has the path segments `["a", "b", "c", ""]`.
    ///
    /// Despite this, setting the [`Self::NextPathSegment`] *overwrites* the last segment instead of appending it.
    ///
    /// If you truly must append a path segment after an empty trailing path segment, use [`Self::AfterPathSegment`] with a value of `-1`.
    /// # Getting
    /// Always [`None`].
    /// # Setting
    /// If set to [`None`], does nothing.
    ///
    /// If set to [`Some`],
    ///
    /// 1. If the last path segment is empty, remove it.
    /// 2. Append the new path segment.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c").unwrap();
    ///
    /// assert_eq!(UrlPart::NextPathSegment.get(&url), None);
    ///
    /// UrlPart::NextPathSegment.set(&mut url, None).unwrap();
    /// assert_eq!(url.path(), "/a/b/c");
    ///
    /// UrlPart::NextPathSegment.set(&mut url, Some("d")).unwrap();
    /// assert_eq!(url.path(), "/a/b/c/d");
    ///
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c/").unwrap();
    ///
    /// // Note that trailing empty path segment was removed.
    /// UrlPart::NextPathSegment.set(&mut url, Some("d")).unwrap();
    /// assert_eq!(url.path(), "/a/b/c/d");
    ///
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c//").unwrap();
    ///
    /// // Note that empty path segment before the trailing empty path segment wasn't removed.
    /// UrlPart::NextPathSegment.set(&mut url, Some("d")).unwrap();
    /// assert_eq!(url.path(), "/a/b/c//d");
    /// ```
    NextPathSegment,



    /// The query. Does not include the `?`.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(UrlPart::Query.get(&url), None);
    ///
    /// UrlPart::Query.set(&mut url, Some("")).unwrap();
    /// assert_eq!(UrlPart::Query.get(&url), Some("".into()));
    ///
    /// UrlPart::Query.set(&mut url, Some("abc")).unwrap();
    /// assert_eq!(UrlPart::Query.get(&url), Some("abc".into()));
    ///
    /// UrlPart::Query.set(&mut url, Some("abc=def")).unwrap();
    /// assert_eq!(UrlPart::Query.get(&url), Some("abc=def".into()));
    ///
    /// UrlPart::Query.set(&mut url, Some("")).unwrap();
    /// assert_eq!(UrlPart::Query.get(&url), Some("".into()));
    ///
    /// UrlPart::Query.set(&mut url, None).unwrap();
    /// assert_eq!(UrlPart::Query.get(&url), None);
    /// ```
    Query,
    /// The selected query parameter.
    ///
    /// Setting a query parameter with a [`QueryParamSelector::index`] of exactly one more than the current count of query parameters with the matching [`QueryParamSelector::name`] will append a new query parameter.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(UrlPart::QueryParam(QueryParamSelector {name: "a".into(), index: 0}).get(&url), None);
    ///
    /// UrlPart::QueryParam(QueryParamSelector {name: "a".into(), index: 0}).set(&mut url, Some("2")).unwrap();
    /// assert_eq!(url.query(), Some("a=2"));
    /// assert_eq!(UrlPart::QueryParam(QueryParamSelector {name: "a".into(), index: 0}).get(&url), Some("2".into()));
    ///
    /// UrlPart::QueryParam(QueryParamSelector {name: "a".into(), index: 1}).set(&mut url, Some("3")).unwrap();
    /// assert_eq!(url.query(), Some("a=2&a=3"));
    /// assert_eq!(UrlPart::QueryParam(QueryParamSelector {name: "a".into(), index: 0}).get(&url), Some("2".into()));
    /// assert_eq!(UrlPart::QueryParam(QueryParamSelector {name: "a".into(), index: 1}).get(&url), Some("3".into()));
    ///
    /// UrlPart::QueryParam(QueryParamSelector {name: "a".into(), index: 0}).set(&mut url, None).unwrap();
    /// assert_eq!(url.query(), Some("a=3"));
    /// assert_eq!(UrlPart::QueryParam(QueryParamSelector {name: "a".into(), index: 0}).get(&url), Some("3".into()));
    /// assert_eq!(UrlPart::QueryParam(QueryParamSelector {name: "a".into(), index: 1}).get(&url), None);
    /// ```
    QueryParam(QueryParamSelector),
    /// [`Self::QueryParam`] without doing any percent decoding.
    ///
    /// Useful for directly transplanting query parameters.
    RawQueryParam(QueryParamSelector),



    /// The fragment. Does not include the `#`.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(UrlPart::Fragment.get(&url), None);
    ///
    /// UrlPart::Fragment.set(&mut url, Some("a")).unwrap();
    /// assert_eq!(UrlPart::Fragment.get(&url), Some("a".into()));
    ///
    /// UrlPart::Fragment.set(&mut url, None).unwrap();
    /// assert_eq!(UrlPart::Fragment.get(&url), None);
    /// ```
    Fragment,

    /// Uses [`url::Position`]s to handle multiple adjacent parts at the same time.
    ///
    /// Getting uses the range `start..end` and setting joins the range `..start`, the value to set (or the empty string if [`None`]), and `end..`.
    /// # Errors
    /// If the call to [`BetterUrl::parse`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::glue::UrlPosition;
    ///
    /// // Note that the `#1` at the end is the fragment, so just getting the query gives the wrong answer.
    /// let mut url = BetterUrl::parse("https://href.li/?https://example.com/?abc=123&def=456#1").unwrap();
    /// assert_eq!(
    ///     UrlPart::PositionRange {start: UrlPosition::BeforeQuery, end: UrlPosition::AfterFragment}.get(&url),
    ///     Some("https://example.com/?abc=123&def=456#1".into())
    /// );
    ///
    /// UrlPart::PositionRange {start: UrlPosition::AfterPath, end: UrlPosition::AfterFragment}.set(&mut url, None).unwrap();
    /// assert_eq!(url, "https://href.li/");
    /// ```
    PositionRange {
        /// The start of the range to get/set.
        start: UrlPosition,
        /// The end of the range to get/set.
        end: UrlPosition
    }
}

/// Allows getting and setting specific instances of a query parameter.
///
/// For example, it allows getting and setting the second `a` in `https://example.com?a=1&a=2`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub struct QueryParamSelector {
    /// The name of the query parameter to get.
    pub name: String,
    /// The index of matching query parameters to get.
    ///
    /// Defaults to `0`.
    #[serde(default, skip_serializing_if = "is_default")]
    pub index: usize
}

string_or_struct_magic!(QueryParamSelector);

impl FromStr for QueryParamSelector {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for QueryParamSelector {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for QueryParamSelector {
    fn from(value: String) -> Self {
        Self {
            name: value,
            index: Default::default()
        }
    }
}

impl UrlPart {
    /// Gets the value.
    pub fn get<'a>(&self, url: &'a BetterUrl) -> Option<Cow<'a, str>> {
        debug!(self, UrlPart::get, url);
        Some(match self {
            Self::Debug(part) => {
                let ret = part.get(url);
                eprintln!("=== UrlPart::Debug ===\nUrlPart: {part:?}\nValue: {ret:?}");
                ret?
            },



            Self::Whole => Cow::Borrowed(url.as_str()),



            Self::Scheme   => Cow::Borrowed(url.scheme()),
            Self::Username => Cow::Borrowed(url.username()),
            Self::Password => Cow::Borrowed(url.password()?),



            Self::Host           => Cow::Borrowed(url.host_str()?),
            Self::NormalizedHost => Cow::Borrowed(url.host_str().map(|x| x.strip_prefix("www.").unwrap_or(x)).map(|x| x.strip_suffix(".").unwrap_or(x))?),



            Self::DomainSegment(n @ 0..) => Cow::Borrowed(url.domain()?.split('.').nth((*n) as usize)?),
            #[allow(clippy::arithmetic_side_effects, reason = "n is always below zero.")]
            Self::DomainSegment(n @ ..0) => Cow::Borrowed(url.domain()?.split('.').nth_back((n + 1).unsigned_abs())?),
            Self::BeforeDomainSegment(_) => None?,
            Self::AfterDomainSegment(_)  => None?,



            Self::SubdomainSegment(n @ 0..)    => Cow::Borrowed(url.subdomain()?.split('.').nth((*n) as usize)?),
            #[allow(clippy::arithmetic_side_effects, reason = "n is always below zero.")]
            Self::SubdomainSegment(n @ ..0)    => Cow::Borrowed(url.subdomain()?.split('.').nth_back((n + 1).unsigned_abs())?),
            Self::BeforeSubdomainSegment(_)    => None?,
            Self::AfterSubdomainSegment(_)     => None?,
            Self::DomainSuffixSegment(n @ 0..) => Cow::Borrowed(url.domain_suffix()?.split('.').nth((*n) as usize)?),
            #[allow(clippy::arithmetic_side_effects, reason = "n is always below zero.")]
            Self::DomainSuffixSegment(n @ ..0) => Cow::Borrowed(url.domain_suffix()?.split('.').nth_back((n + 1).unsigned_abs())?),
            Self::BeforeDomainSuffixSegment(_) => None?,
            Self::AfterDomainSuffixSegment(_)  => None?,



            Self::Domain          => Cow::Borrowed(url.domain           ()?),
            Self::Subdomain       => Cow::Borrowed(url.subdomain        ()?),
            Self::RegDomain       => Cow::Borrowed(url.reg_domain       ()?),
            Self::NotDomainSuffix => Cow::Borrowed(url.not_domain_suffix()?),
            Self::DomainMiddle    => Cow::Borrowed(url.domain_middle    ()?),
            Self::DomainSuffix    => Cow::Borrowed(url.domain_suffix    ()?),
            Self::FqdnPeriod      => Cow::Borrowed(url.fqdn_period      ()?),



            Self::Port => Cow::Owned(url.port()?.to_string()),



            Self::Path                 => Cow::Borrowed(url.path()),
            Self::PathSegment(n @ 0..) => Cow::Borrowed(url.path_segments().ok()?.nth((*n) as usize)?),
            #[allow(clippy::arithmetic_side_effects, reason = "n is always below zero.")]
            Self::PathSegment(n @ ..0) => Cow::Borrowed(url.path_segments().ok()?.nth_back((n + 1).unsigned_abs())?),
            Self::BeforePathSegment(_) => None?,
            Self::AfterPathSegment(_)  => None?,
            Self::NextPathSegment      => None?,



            Self::Query => Cow::Borrowed(url.query()?),
            Self::QueryParam   (QueryParamSelector {name, index}) => url.get_query_param(name, *index)???,
            Self::RawQueryParam(QueryParamSelector {name, index}) => Cow::Borrowed(url.get_raw_query_param(name, *index)???),



            Self::Fragment => Cow::Borrowed(url.fragment()?),



            Self::PositionRange {start, end} => Cow::Borrowed(&url[start.0..end.0])
        })
    }

    /// Sets the value.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn set(&self, url: &mut BetterUrl, to: Option<&str>) -> Result<(), UrlPartSetError> {
        debug!(self, UrlPart::set, url, to);
        match (self, to) {
            (Self::Debug(part), _) => {
                let old = part.get(url).to_owned();
                eprintln!("=== UrlPart::Debug ===\nUrlPart: {part:?}\nOld value: {old:?}\nNew value: {to:?}");
                part.set(url, to)?;
            },



            (Self::Whole   , Some(to)) => *url=BetterUrl::parse(to)?,
            (Self::Whole   , None    ) => Err(UrlPartSetError::WholeCannotBeNone)?,
            (Self::Scheme  , Some(to)) => url.set_scheme(to)?,
            (Self::Scheme  , None    ) => Err(UrlPartSetError::SchemeCannotBeNone)?,
            (Self::Username, Some(to)) => url.set_username(to)?,
            (Self::Username, None    ) => Err(UrlPartSetError::UsernameCannotBeNone)?,
            (Self::Password, _       ) => url.set_password(to)?,



            (Self::Host , _) => url.set_host(to)?,
            (Self::NormalizedHost, _) => url.set_host(to.map(|to| to.strip_prefix("www.").unwrap_or(to)).map(|x| x.strip_suffix(".").unwrap_or(x)))?,



            (Self::BeforeDomainSegment      (n), _) => url.set_domain       (Some(set_rel_segment(url.domain       ().ok_or(UrlPartSetError::NoDomain      )?.split('.'), *n, SegRel::Before, to)?).filter(|x| !x.is_empty()).map(|x| x.join(".")).as_deref())?,
            (Self::DomainSegment            (n), _) => url.set_domain       (Some(set_rel_segment(url.domain       ().ok_or(UrlPartSetError::NoDomain      )?.split('.'), *n, SegRel::At    , to)?).filter(|x| !x.is_empty()).map(|x| x.join(".")).as_deref())?,
            (Self::AfterDomainSegment       (n), _) => url.set_domain       (Some(set_rel_segment(url.domain       ().ok_or(UrlPartSetError::NoDomain      )?.split('.'), *n, SegRel::After , to)?).filter(|x| !x.is_empty()).map(|x| x.join(".")).as_deref())?,
            (Self::BeforeSubdomainSegment   (n), _) => url.set_subdomain    (Some(set_rel_segment(url.subdomain    ().ok_or(UrlPartSetError::NoSubdomain   )?.split('.'), *n, SegRel::Before, to)?).filter(|x| !x.is_empty()).map(|x| x.join(".")).as_deref())?,
            (Self::SubdomainSegment         (n), _) => url.set_subdomain    (Some(set_rel_segment(url.subdomain    ().ok_or(UrlPartSetError::NoSubdomain   )?.split('.'), *n, SegRel::At    , to)?).filter(|x| !x.is_empty()).map(|x| x.join(".")).as_deref())?,
            (Self::AfterSubdomainSegment    (n), _) => url.set_subdomain    (Some(set_rel_segment(url.subdomain    ().ok_or(UrlPartSetError::NoSubdomain   )?.split('.'), *n, SegRel::After , to)?).filter(|x| !x.is_empty()).map(|x| x.join(".")).as_deref())?,
            (Self::BeforeDomainSuffixSegment(n), _) => url.set_domain_suffix(Some(set_rel_segment(url.domain_suffix().ok_or(UrlPartSetError::NoDomainSuffix)?.split('.'), *n, SegRel::Before, to)?).filter(|x| !x.is_empty()).map(|x| x.join(".")).as_deref())?,
            (Self::DomainSuffixSegment      (n), _) => url.set_domain_suffix(Some(set_rel_segment(url.domain_suffix().ok_or(UrlPartSetError::NoDomainSuffix)?.split('.'), *n, SegRel::At    , to)?).filter(|x| !x.is_empty()).map(|x| x.join(".")).as_deref())?,
            (Self::AfterDomainSuffixSegment (n), _) => url.set_domain_suffix(Some(set_rel_segment(url.domain_suffix().ok_or(UrlPartSetError::NoDomainSuffix)?.split('.'), *n, SegRel::After , to)?).filter(|x| !x.is_empty()).map(|x| x.join(".")).as_deref())?,



            (Self::Domain         , _        ) => url.set_domain           (to)?,
            (Self::Subdomain      , _        ) => url.set_subdomain        (to)?,
            (Self::RegDomain      , _        ) => url.set_reg_domain       (to)?,
            (Self::NotDomainSuffix, _        ) => url.set_not_domain_suffix(to)?,
            (Self::DomainMiddle   , _        ) => url.set_domain_middle    (to)?,
            (Self::DomainSuffix   , _        ) => url.set_domain_suffix    (to)?,
            (Self::FqdnPeriod     , Some(".")) => url.set_fqdn(true)?,
            (Self::FqdnPeriod     , None     ) => url.set_fqdn(false)?,
            (Self::FqdnPeriod     , Some(_)  ) => Err(UrlPartSetError::FqdnPeriodMustBeNoneOrPeriod)?,



            (Self::Port, _) => url.set_port(to.map(|x| x.parse().map_err(|_| UrlPartSetError::InvalidPort)).transpose()?)?,



            (Self::Path, Some(to)) => url.set_path(to),
            (Self::Path, None    ) => Err(UrlPartSetError::PathCannotBeNone)?,
            (Self::BeforePathSegment(n), _) => url.set_path(&set_rel_segment(url.path_segments()?, *n, SegRel::Before, to)?.join("/")),
            (Self::PathSegment      (n), _) => url.set_path(&set_rel_segment(url.path_segments()?, *n, SegRel::At    , to)?.join("/")),
            (Self::AfterPathSegment (n), _) => url.set_path(&set_rel_segment(url.path_segments()?, *n, SegRel::After , to)?.join("/")),
            (Self::NextPathSegment     , _) => if let Some(to) = to {url.path_segments_mut()?.pop_if_empty().push(to);},



            (Self::Query, _) => url.set_query(to),
            (Self::QueryParam   (QueryParamSelector {name, index}), _) => url.set_query_param(name, *index, to.map(Some))?,
            (Self::RawQueryParam(QueryParamSelector {name, index}), _) => url.set_raw_query_param(name, *index, to.map(Some))?,



            (Self::Fragment, _) => url.set_fragment(to),



            (Self::PositionRange {start, end}, _) => *url = BetterUrl::parse(&format!("{}{}{}", &url[..start.0], to.unwrap_or_default(), &url[end.0..]))?
        }
        Ok(())
    }
}

/// The enum of errors [`UrlPart::set`] can return.
#[derive(Debug, Error)]
pub enum UrlPartSetError {
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)] UrlParseError(#[from] url::ParseError),
    /// Returned when attempting to set [`UrlPart::Whole`] to [`None`].
    #[error("Attempted to set a whole URL to None.")]
    WholeCannotBeNone,

    // Pre-host stuff.

    /// Returned when attempting to set a URL's scheme to [`None`].
    #[error("Attempted to set a URL's scheme to None.")]
    SchemeCannotBeNone,
    /// Returned when attempting to set a URL's scheme to an invalid value.
    #[error("Attempted to set a URL's scheme to an invalid value.")]
    SetSchemeError,
    /// Returned when attempting to set a URL's username to [`None`].
    #[error("Attempted to set a URL's username to None.")]
    UsernameCannotBeNone,
    /// Returned when attempting to set a URL's username to an invalid value.
    #[error("Attempted to set a URL's username to an invalid value.")]
    SetUsernameError,
    /// Returned when attempting to set a URL's password to an invalid value.
    #[error("Attempted to set a URL's password to an invalid value.")]
    SetPasswordError,

    // Host stuff.

    /// Returned when a [`SetHostError`] is encountered.
    #[error(transparent)]
    SetHostError(#[from] SetHostError),
    /// Returned when attempting to set the host of a URL with no host to an IP.
    #[error("Attempted to set the host of a URL with no host to an IP.")]
    SetIpHostError,
    /// Returned when a [`SetSubdomainError)`] is encountered.
    #[error(transparent)] SetSubdomainError      (#[from] SetSubdomainError),
    /// Returned when a [`SetDdomainError)`] is encountered.
    #[error(transparent)] SetDomainError         (#[from] SetDomainError),
    /// Returned when a [`SetNotDomainSuffixError)`] is encountered.
    #[error(transparent)] SetNotDomainSuffixError(#[from] SetNotDomainSuffixError),
    /// Returned when a [`SetDomainMiddleError)`] is encountered.
    #[error(transparent)] SetDomainMiddleError   (#[from] SetDomainMiddleError),
    /// Returned when a [`SetRegDomainError)`] is encountered.
    #[error(transparent)] SetRegDomainError      (#[from] SetRegDomainError),
    /// Returned when a [`SetDomainSuffixError)`] is encountered.
    #[error(transparent)] SetDomainSuffixError   (#[from] SetDomainSuffixError),
    /// Returned when a [`SetDomainHostError`] is encountered.
    #[error(transparent)] SetDomainHostError     (#[from] SetDomainHostError),
    /// Returned when a [`SetFqdnPeriodError`] is encountered.
    #[error(transparent)] SetFqdnPeriodError     (#[from] SetFqdnPeriodError),
    /// Returned when attempting to set a domain's [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period to a value other than [`None`] and `.`.
    #[error("Attempted to set a domain's FQDN period to a value other than None and \".\".")]
    FqdnPeriodMustBeNoneOrPeriod,



    /// Returned when attempting to set a [`UrlPart::DomainSegment`] of a URL with no [`UrlPart::Domain`].
    #[error("Attempted to set a domain segment of a URL with no domain.")]
    NoDomain,
    /// Returned when attempting to set a [`UrlPart::SubdomainSegment`] of a URL with no [`UrlPart::Subdomain`].
    #[error("Attempted to set a subdomain segment of a URL with no subdomain.")]
    NoSubdomain,
    /// Returned when attempting to set a [`UrlPart::DomainSuffixSegment`] of a URL with no [`UrlPart::DomainSuffix`].
    #[error("Attempted to set a domain suffix segment of a URL with no domain suffix.")]
    NoDomainSuffix,

    // Post-host stuff.

    /// Returned when attempting to set a port to a value that isn't a number between 0 and 65535 (inclusive).
    #[error("Attempted to set a port to a value that isn't a number between 0 and 65535 (inclusive).")]
    InvalidPort,
    /// Returned when attempting to set a URL's port fails.
    #[error("Attempting to set the URL's port failed.")]
    SetPortError,
    /// Returned when attempting to set a URL's path to [`None`].
    #[error("Attempted to set the URL's path to None.")]
    PathCannotBeNone,
    /// Returned when attempting to get/set a URL's path segments when it doesn't have any.
    #[error("Attempted to manipulate a URL's path segments when it doesn't have any.")]
    UrlDoesNotHavePathSegments,
    /// Returned when a [`SetQueryParamError)`] is encountered.
    #[error(transparent)] SetQueryParamError(#[from] SetQueryParamError),

    // General stuff.

    /// Returned when a requested segment isn't found.
    #[error("The requested segment was not found.")]
    SegmentNotFound,
}

from_units!{UrlPartSetError, SegmentNotFound, SetPortError, SetIpHostError, SetPasswordError, SetUsernameError, SetSchemeError, UrlDoesNotHavePathSegments}
