//! A common API for getting and setting various parts of [`BetterUrl`]s.

use std::borrow::Cow;
use std::str::FromStr;

use url::Url;
use thiserror::Error;
use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::util::*;

/// A common API for getting and setting various parts of [`BetterUrl`]s.
///
/// For most parts, setting a URL's part to a value then getting that same part returns the same value.
///
/// Exceptions include setting part segments to values containing the split, `After`/`Before`/`Next` vairants always returning [`None`], and probably some other things. I'll fix this doc later.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Suitability)]
pub enum UrlPart {
    /// Print debug information about the contained [`Self`].
    #[suitable(never)]
    Debug(Box<Self>),



    /// The whole URL.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
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
    /// # use url_cleaner::types::*;
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
    /// # use url_cleaner::types::*;
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
    /// # use url_cleaner::types::*;
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
    /// [`Self::Host`] but with the `www.` at the start, if it's there, removed.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// let mut url = BetterUrl::parse("https://www.www.example.com").unwrap();
    /// assert_eq!(UrlPart::HostWithoutWWWDotPrefixAndFqdnPeriod.get(&url), Some("www.example.com".into()));
    ///
    /// UrlPart::HostWithoutWWWDotPrefixAndFqdnPeriod.set(&mut url, Some("www.example.com")).unwrap();
    /// assert_eq!(UrlPart::HostWithoutWWWDotPrefixAndFqdnPeriod.get(&url), Some("example.com".into()));
    ///
    /// assert_eq!(url.host_str(), Some("example.com"));
    ///
    /// let mut url = BetterUrl::parse("https://www.www.example.com.").unwrap();
    /// assert_eq!(UrlPart::HostWithoutWWWDotPrefixAndFqdnPeriod.get(&url), Some("www.example.com".into()));
    /// ```
    HostWithoutWWWDotPrefixAndFqdnPeriod,



    /// The nth domain segment of the [`Self::Domain`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
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
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(UrlPart::BeforeDomainSegment(0).get(&url), None);
    ///
    /// UrlPart::BeforeDomainSegment(0).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com"));
    /// UrlPart::BeforeDomainSegment(0).set(&mut url, Some("")).unwrap_err();
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
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(UrlPart::AfterDomainSegment(0).get(&url), None);
    ///
    /// UrlPart::AfterDomainSegment(0).set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com"));
    /// UrlPart::AfterDomainSegment(0).set(&mut url, Some("")).unwrap_err();
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
    /// The position in [`Self::Domain`] after the last domain segment.
    ///
    /// Allows appending domain segments.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(UrlPart::NextDomainSegment.get(&url), None);
    ///
    /// UrlPart::NextDomainSegment.set(&mut url, None).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com"));
    /// UrlPart::NextDomainSegment.set(&mut url, Some("")).unwrap_err();
    /// assert_eq!(url.host_str(), Some("example.com"));
    /// UrlPart::NextDomainSegment.set(&mut url, Some("com")).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com.com"));
    ///
    /// // Fully qualified domain names give the same results.
    /// let mut url = BetterUrl::parse("https://example.com.").unwrap();
    ///
    /// UrlPart::NextDomainSegment.set(&mut url, Some("com")).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com.com."));
    /// ```
    NextDomainSegment,



    /// The host if it's a domain, *not* including the [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period, if it's present.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
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
    /// # use url_cleaner::types::*;
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
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
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
    /// # use url_cleaner::types::*;
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
    /// # use url_cleaner::types::*;
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
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
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
    /// # use url_cleaner::types::*;
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
    /// # use url_cleaner::types::*;
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
    /// If the URL is a [cannot-be-a-base](https://docs.rs/url/latest/url/struct.Url.html#method.cannot_be_a_base) URL, this probably does something. The [`Url::set_path`] docs don't say.
    ///
    /// For other (most) URLs, this first ensures the value starts with `/` (`abc` -> `/abc`, `/def` -> `/def`) then sets the URL's path to that value.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
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
    /// # use url_cleaner::types::*;
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
    /// A range of [`Self::PathSegment`]s.
    ///
    /// Please note that a path like `/a/b/c/` has the path segments `["a", "b", "c", ""]`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c").unwrap();
    ///
    /// assert_eq!(UrlPart::PathSegments {start: 0, end: Some(0)}.get(&url), Some("".into()));
    /// assert_eq!(UrlPart::PathSegments {start: 0, end: Some(1)}.get(&url), Some("a".into()));
    /// assert_eq!(UrlPart::PathSegments {start: 0, end: Some(2)}.get(&url), Some("a/b".into()));
    /// assert_eq!(UrlPart::PathSegments {start: 0, end: Some(3)}.get(&url), Some("a/b/c".into()));
    /// assert_eq!(UrlPart::PathSegments {start: 0, end: Some(4)}.get(&url), None);
    /// assert_eq!(UrlPart::PathSegments {start: 0, end: None   }.get(&url), Some("a/b/c".into()));
    ///
    /// UrlPart::PathSegments {start: 0, end: Some(2)}.set(&mut url, Some("A")).unwrap();
    /// assert_eq!(url.path(), "/A/c");
    ///
    /// UrlPart::PathSegments {start: 0, end: Some(2)}.set(&mut url, Some("yes/this/works/and/is/weird")).unwrap();
    /// assert_eq!(url.path(), "/yes/this/works/and/is/weird");
    /// assert_eq!(UrlPart::PathSegments {start: 0, end: Some(2)}.get(&url), Some("yes/this".into()));
    /// UrlPart::PathSegments {start: 0, end: Some(2)}.set(&mut url, None).unwrap();
    /// assert_eq!(url.path(), "/works/and/is/weird");
    ///
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c/").unwrap();
    ///
    /// assert_eq!(UrlPart::PathSegments {start: 0, end: Some(0)}.get(&url), Some("".into()));
    /// assert_eq!(UrlPart::PathSegments {start: 0, end: Some(1)}.get(&url), Some("a".into()));
    /// assert_eq!(UrlPart::PathSegments {start: 0, end: Some(2)}.get(&url), Some("a/b".into()));
    /// assert_eq!(UrlPart::PathSegments {start: 0, end: Some(3)}.get(&url), Some("a/b/c".into()));
    /// assert_eq!(UrlPart::PathSegments {start: 0, end: Some(4)}.get(&url), Some("a/b/c/".into()));
    /// assert_eq!(UrlPart::PathSegments {start: 0, end: Some(5)}.get(&url), None);
    /// assert_eq!(UrlPart::PathSegments {start: 0, end: None   }.get(&url), Some("a/b/c/".into()));
    /// ```
    PathSegments {
        /// The path segment to start the range at.
        ///
        /// Defaults to `0`.
        #[serde(default, skip_serializing_if = "is_default")]
        start: isize,
        /// The path segment to end the range just before.
        ///
        /// If [`None`], gets all path segments after [`Self::PathSegments::start`].
        ///
        /// Defaults to [`None`].
        #[serde(default, skip_serializing_if = "is_default")]
        end: Option<isize>
    },
    /// The path segment between the nth and the previous one.
    ///
    /// Please note that a path like `/a/b/c/` has the path segments `["a", "b", "c", ""]`.
    /// # Getting
    /// Always [`None`].
    /// # Setting
    /// If set to [`None`], does nothing.
    ///
    /// If set to [`Some`], inserts a new path segment between the nth and the previous one.
    ///
    /// If [`Self::BeforePathSegment::0`] is 1 above the last path segment, this currently still works. Whether or not this is a good design is unclear to me.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
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
    /// UrlPart::BeforePathSegment(4).set(&mut url, Some("e")).unwrap();
    /// assert_eq!(url.path(), "/d/a/b/c/e");
    /// ```
    BeforePathSegment(isize),
    /// The path segment between the nth and the next one.
    ///
    /// Please note that a path like `/a/b/c/` has the path segments `["a", "b", "c", ""]`.
    /// # Getting
    /// Always [`None`].
    /// # Setting
    /// If set to [`None`], does nothing.
    ///
    /// If set to [`Some`], inserts a new path segment between the nth and the next one.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
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
    /// # use url_cleaner::types::*;
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
    /// # use url_cleaner::types::*;
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
    /// Setting a query parameter with a [`QueryParamSelector::index`] of exactly one more than the current count of query parameters with the matching [`QueryParamSelector::name`] will append a new query paramter.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
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



    /// The fragment. Does not include the `#`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
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
    Fragment
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

impl QueryParamSelector {
    /// Get the selected query parameter.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// let url = BetterUrl::parse("https://example.com?a=2&b=3&a=4").unwrap();
    ///
    /// assert_eq!(QueryParamSelector {name: "a".into(), index: 0}.get(&url), Some("2".into()));
    /// assert_eq!(QueryParamSelector {name: "a".into(), index: 1}.get(&url), Some("4".into()));
    /// assert_eq!(QueryParamSelector {name: "a".into(), index: 2}.get(&url), None);
    /// assert_eq!(QueryParamSelector {name: "b".into(), index: 0}.get(&url), Some("3".into()));
    /// assert_eq!(QueryParamSelector {name: "b".into(), index: 1}.get(&url), None);
    /// ```
    pub fn get<'a>(&self, url: &'a Url) -> Option<Cow<'a, str>> {
        self.get_from_iter(url.query_pairs())
    }

    /// Get the selected query parameter and its absolute index in the list of query parameters.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// let url = BetterUrl::parse("https://example.com?a=2&b=3&a=4").unwrap();
    ///
    /// assert_eq!(QueryParamSelector {name: "a".into(), index: 0}.get_with_index(&url), Some((0, "2".into())));
    /// assert_eq!(QueryParamSelector {name: "a".into(), index: 1}.get_with_index(&url), Some((2, "4".into())));
    /// assert_eq!(QueryParamSelector {name: "a".into(), index: 2}.get_with_index(&url), None);
    /// assert_eq!(QueryParamSelector {name: "b".into(), index: 0}.get_with_index(&url), Some((1, "3".into())));
    /// assert_eq!(QueryParamSelector {name: "b".into(), index: 1}.get_with_index(&url), None);
    /// ```
    pub fn get_with_index<'a>(&self, url: &'a Url) -> Option<(usize, Cow<'a, str>)> {
        self.get_from_iter_with_index(url.query_pairs())
    }

    /// Get the selected query parameter from an [`Iterator`] of query parameters.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// let query_pairs = [("a", "2"), ("b", "3"), ("a", "4")];
    ///
    /// assert_eq!(QueryParamSelector {name: "a".into(), index: 0}.get_from_iter(query_pairs), Some("2".into()));
    /// assert_eq!(QueryParamSelector {name: "a".into(), index: 1}.get_from_iter(query_pairs), Some("4".into()));
    /// assert_eq!(QueryParamSelector {name: "a".into(), index: 2}.get_from_iter(query_pairs), None);
    /// assert_eq!(QueryParamSelector {name: "b".into(), index: 0}.get_from_iter(query_pairs), Some("3".into()));
    /// assert_eq!(QueryParamSelector {name: "b".into(), index: 1}.get_from_iter(query_pairs), None);
    /// ```
    pub fn get_from_iter<I: IntoIterator<Item = (K, V)>, K: AsRef<str>, V>(&self, pairs: I) -> Option<V> {
        Some(pairs.into_iter().filter(|(name, _)| name.as_ref()==self.name).enumerate().find(|(i, _)| *i==self.index)?.1.1)
    }

    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// let query_pairs = [("a", "2"), ("b", "3"), ("a", "4")];
    ///
    /// assert_eq!(QueryParamSelector {name: "a".into(), index: 0}.get_from_iter_with_index(query_pairs), Some((0, "2".into())));
    /// assert_eq!(QueryParamSelector {name: "a".into(), index: 1}.get_from_iter_with_index(query_pairs), Some((2, "4".into())));
    /// assert_eq!(QueryParamSelector {name: "a".into(), index: 2}.get_from_iter_with_index(query_pairs), None);
    /// assert_eq!(QueryParamSelector {name: "b".into(), index: 0}.get_from_iter_with_index(query_pairs), Some((1, "3".into())));
    /// assert_eq!(QueryParamSelector {name: "b".into(), index: 1}.get_from_iter_with_index(query_pairs), None);
    /// ```
    pub fn get_from_iter_with_index<I: IntoIterator<Item = (K, V)>, K: AsRef<str>, V>(&self, pairs: I) -> Option<(usize, V)> {
        pairs.into_iter().enumerate().filter(|(_, (name, _))| name.as_ref()==self.name).enumerate().find_map(|(ni, (ai, (_, v)))| (ni==self.index).then_some((ai, v)))
    }

    /// Set the selected query parameter.
    ///
    /// Note that if [`Self::index`] is equal to the number of matched query params, this appends a new query parameter.
    /// # Errors
    /// If [`Self::index`] is  above the number of matched query params, returns the error [`SetQueryParamError::QueryParamIndexNotFound`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// QueryParamSelector {name: "a".into(), index: 0}.set(&mut url, Some("2")).unwrap();
    /// assert_eq!(url.query(), Some("a=2"));
    /// QueryParamSelector {name: "a".into(), index: 0}.set(&mut url, Some("3")).unwrap();
    /// assert_eq!(url.query(), Some("a=3"));
    /// QueryParamSelector {name: "a".into(), index: 1}.set(&mut url, Some("4")).unwrap();
    /// assert_eq!(url.query(), Some("a=3&a=4"));
    /// QueryParamSelector {name: "a".into(), index: 3}.set(&mut url, Some("5")).unwrap_err();
    /// assert_eq!(url.query(), Some("a=3&a=4"));
    /// QueryParamSelector {name: "a".into(), index: 0}.set(&mut url, None).unwrap();
    /// assert_eq!(url.query(), Some("a=4"));
    /// QueryParamSelector {name: "a".into(), index: 0}.set(&mut url, None).unwrap();
    /// assert_eq!(url.query(), None);
    /// ```
    pub fn set(&self, url: &mut BetterUrl, to: Option<&str>) -> Result<(), SetQueryParamError> {
        let mut pairs = url.query_pairs().collect::<Vec<_>>();

        let mut found_matches = 0;
        let mut matched_index = None;

        // Find the index of the selected query parameter and store it in `matched_index`.
        for (i, (name, _)) in pairs.iter_mut().enumerate() {
            if *name == self.name {
                if found_matches == self.index {
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
            (Some(i), Some(to)) => pairs[i].1 = to.into(),
            (Some(i), None    ) => {pairs.remove(i);},
            (None   , Some(to)) => if self.index == found_matches {
                pairs.push((self.name.clone().into(), to.into()));
            } else {
                Err(SetQueryParamError::QueryParamIndexNotFound)?
            },
            (None, None) => {}
        }

        // Turn the pairs into a query.
        let serialized_query = if pairs.is_empty() {
            None
        } else {
            Some(form_urlencoded::Serializer::new(String::with_capacity(url.query().unwrap_or_default().len())).extend_pairs(pairs).finish())
        };

        url.set_query(serialized_query.as_deref());

        Ok(())
    }
}

/// The enum of errors [`QueryParamSelector::set`] can return.
#[derive(Debug, Error)]
pub enum SetQueryParamError {
    /// Returned when a query parameter with the specified index can't be set/created.
    #[error("A query parameter with the specified index could not be set/created.")]
    QueryParamIndexNotFound
}

impl UrlPart {
    /// Gets the value.
    pub fn get<'a>(&self, url: &'a BetterUrl) -> Option<Cow<'a, str>> {
        debug!(UrlPart::get, self, url);
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



            Self::Host => Cow::Borrowed(url.host_str()?),
            Self::HostWithoutWWWDotPrefixAndFqdnPeriod => Cow::Borrowed(url.host_str().map(|x| x.strip_prefix("www.").unwrap_or(x)).map(|x| x.strip_suffix(".").unwrap_or(x))?),



            Self::DomainSegment(n) => Cow::Borrowed(neg_nth(url.domain()?.split('.'), *n)?),
            Self::BeforeDomainSegment(_) => None?,
            Self::AfterDomainSegment(_)  => None?,
            Self::NextDomainSegment      => None?,



            Self::Domain          => Cow::Borrowed(url.domain           ()?),
            Self::Subdomain       => Cow::Borrowed(url.subdomain        ()?),
            Self::RegDomain       => Cow::Borrowed(url.reg_domain       ()?),
            Self::NotDomainSuffix => Cow::Borrowed(url.not_domain_suffix()?),
            Self::DomainMiddle    => Cow::Borrowed(url.domain_middle    ()?),
            Self::DomainSuffix    => Cow::Borrowed(url.domain_suffix    ()?),
            Self::FqdnPeriod      => Cow::Borrowed(url.host_str()?.get(url.domain_details()?.fqdn_period?..)?),



            Self::Port => Cow::Owned(url.port()?.to_string()),



            Self::Path => Cow::Borrowed(url.path()),
            Self::PathSegment(n)   => Cow::Borrowed(neg_nth(url.path_segments().ok()?, *n)?),
            Self::PathSegments {start, end} => {
                // TODO: Change to always borrow.
                Cow::Owned(neg_vec_keep(Self::Path.get(url)?.strip_prefix('/')?.split('/'), *start, *end)?.join("/"))
            },
            Self::BeforePathSegment(_)    => None?,
            Self::AfterPathSegment(_)     => None?,
            Self::NextPathSegment         => None?,



            Self::Query => Cow::Borrowed(url.query()?),
            Self::QueryParam(selector) => selector.get(url)?,



            Self::Fragment                => Cow::Borrowed(url.fragment()?),
        })
    }

    /// Sets the value.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn set(&self, url: &mut BetterUrl, to: Option<&str>) -> Result<(), UrlPartSetError> {
        debug!(UrlPart::set, self, url, to);
        match (self, to) {
            (Self::Debug(part), _) => {
                let old = part.get(url).to_owned();
                eprintln!("=== UrlPart::Debug ===\nUrlPart: {part:?}\nOld value: {old:?}\nNew value: {to:?}");
                part.set(url, to)?;
            },



            (Self::Whole   , Some(to)) => *url=BetterUrl::parse(to)?,
            (Self::Scheme  , Some(to)) => url.set_scheme  (to)?,
            (Self::Username, Some(to)) => url.set_username(to)?,
            (Self::Password, _       ) => url.set_password(to)?,



            (Self::Host , _) => url.set_host(to)?,
            (Self::HostWithoutWWWDotPrefixAndFqdnPeriod, _) => url.set_host(to.map(|to| to.strip_prefix("www.").unwrap_or(to)).map(|x| x.strip_suffix(".").unwrap_or(x)))?,


            (Self::DomainSegment(n), _) => {
                let mut segments = url.domain().ok_or(UrlPartSetError::HostIsNotADomain)?.split('.').collect::<Vec<_>>();
                let fixed_n=neg_index(*n, segments.len()).ok_or(UrlPartSetError::SegmentNotFound)?;
                #[allow(clippy::indexing_slicing, reason = "`fixed_n` is guaranteed to be in bounds.")]
                match to {
                    Some(to) => if to.is_empty() {
                        Err(UrlPartSetError::DomainSegmentCannotBeEmpty)?
                    } else {
                        segments[fixed_n]=to
                    },
                    None => {let _ = segments.remove(fixed_n);}
                }
                Self::Domain.set(url, Some(&segments.join(".")))?;
            },
            (Self::BeforeDomainSegment(n), _) => if let Some(to) = to {
                if to.is_empty() {Err(UrlPartSetError::DomainSegmentCannotBeEmpty)?;}
                let mut segments = url.domain().ok_or(UrlPartSetError::HostIsNotADomain)?.split('.').collect::<Vec<_>>();
                let fixed_n=neg_index(*n, segments.len()).ok_or(UrlPartSetError::SegmentBoundaryNotFound)?;
                segments.insert(fixed_n, to);
                Self::Domain.set(url, Some(&segments.join(".")))?;
            },
            (Self::AfterDomainSegment(n), _) => if let Some(to) = to {
                if to.is_empty() {Err(UrlPartSetError::DomainSegmentCannotBeEmpty)?;}
                let mut segments = url.domain().ok_or(UrlPartSetError::HostIsNotADomain)?.split('.').collect::<Vec<_>>();
                #[expect(clippy::arithmetic_side_effects, reason = "Can't happen.")]
                let fixed_n=neg_index(*n, segments.len()).ok_or(UrlPartSetError::SegmentBoundaryNotFound)? + 1;
                segments.insert(fixed_n, to);
                Self::Domain.set(url, Some(&segments.join(".")))?;
            },
            (Self::NextDomainSegment, _) => if let Some(to) = to {
                if to.is_empty() {Err(UrlPartSetError::DomainSegmentCannotBeEmpty)?;}
                Self::Domain.set(url, Some(&url.domain().ok_or(UrlPartSetError::HostIsNotADomain)?.split('.').chain([to]).collect::<Vec<_>>().join(".")))?;
                // Self::Domain.set(url, Some(&match url.host_str().zip(url.domain_details()) {
                //     Some((host, DomainDetails {fqdn_period: Some(_), ..})) => format!("{host}{to}."),
                //     Some((host, DomainDetails {fqdn_period: None   , ..})) => format!("{host}.{to}"),
                //     _ => Err(UrlPartSetError::HostIsNotADomain)?
                // }))?;
            },
            (Self::FqdnPeriod, _) => match (url.domain_details(), to) {
                (Some(DomainDetails {fqdn_period: None   , ..}), None     ) => {},
                (Some(DomainDetails {fqdn_period: None   , ..}), Some(".")) => {url.set_host(url.host_str().map(|host| format!("{host}.")).as_deref())?},
                (Some(DomainDetails {fqdn_period: Some(_), ..}), None     ) => {url.set_host(url.host_str().and_then(|host| host.strip_suffix(".").map(|host| host.to_string())).as_deref())?},
                (Some(DomainDetails {fqdn_period: Some(_), ..}), Some(".")) => {},
                (Some(_)                                       , Some(_  )) => Err(UrlPartSetError::FqdnPeriodMustBeNoneOrPeriod)?,
                (None                                          , Some(_  )) => Err(UrlPartSetError::HostIsNotADomain)?,
                (None                                          , None     ) => {}
            },



            (Self::Domain         , _) => url.set_domain           (to)?,
            (Self::Subdomain      , _) => url.set_subdomain        (to)?,
            (Self::RegDomain      , _) => url.set_reg_domain       (to)?,
            (Self::NotDomainSuffix, _) => url.set_not_domain_suffix(to)?,
            (Self::DomainMiddle   , _) => url.set_domain_middle    (to)?,
            (Self::DomainSuffix   , _) => url.set_domain_suffix    (to)?,



            (Self::Port, _) => url.set_port(to.map(|x| x.parse().map_err(|_| UrlPartSetError::InvalidPort)).transpose()?)?,



            (Self::Path, Some(to)) => url.set_path(to),
            (Self::PathSegment(-1), None) => {url.path_segments_mut()?.pop();},
            (Self::PathSegment(n), _) => {
                let mut segments = url.path_segments()?.collect::<Vec<_>>();
                #[allow(clippy::indexing_slicing, reason = "`fixed_n` is guaranteed to be in bounds.")]
                match (neg_index(*n, segments.len()), to) {
                    (Some(fixed_n), Some(to)) => segments[fixed_n]=to,
                    (Some(fixed_n), None    ) => {let _ = segments.remove(fixed_n);}
                    (None         , Some(_ )) => Err(UrlPartSetError::SegmentNotFound)?,
                    (None         , None    ) => {}
                };
                url.set_path(&segments.join("/"));
            },
            (Self::PathSegments{start, end}, _) => {
                let mut segments = url.path_segments()?.collect::<Vec<_>>();
                set_segment_range(&mut segments, *start, *end, to).or(Err(UrlPartSetError::SegmentRangeNotFound))?;
                url.set_path(&format!("/{}", segments.join("/")));
            },
            (Self::BeforePathSegment(n), _) => if let Some(to) = to {
                let mut segments = url.path_segments()?.collect::<Vec<_>>();
                let fixed_n = neg_range_boundary(*n, segments.len()).ok_or(UrlPartSetError::SegmentBoundaryNotFound)?;
                segments.insert(fixed_n, to);
                url.set_path(&segments.join("/"));
            },
            (Self::AfterPathSegment(n), _) => if let Some(to) = to {
                let mut segments = url.path_segments()?.collect::<Vec<_>>();
                let fixed_n = neg_shifted_range_boundary(*n, segments.len(), 1).ok_or(UrlPartSetError::SegmentBoundaryNotFound)?;
                segments.insert(fixed_n, to);
                url.set_path(&segments.join("/"));
            },
            (Self::NextPathSegment, _) => if let Some(to) = to {url.path_segments_mut()?.pop_if_empty().push(to);},



            (Self::Query, _) => url.set_query(to),
            (Self::QueryParam(selector), _) => selector.set(url, to)?,



            (Self::Fragment, _) => url.set_fragment(to),



            (_, None) => Err(UrlPartSetError::PartCannotBeNone)?
        }
        Ok(())
    }
}

/// The enum of errors [`UrlPart::set`] can return.
#[derive(Debug, Error)]
pub enum UrlPartSetError {
    /// Returned when a requested segment isn't found.
    #[error("The requested segment was not found.")]
    SegmentNotFound,
    /// Returned when a requested segment range isn't found.
    #[error("The requested segment range was not found.")]
    SegmentRangeNotFound,
    /// Returned when the requested part is [`None`].
    #[error("The requested part was None.")]
    PartIsNone,
    /// Returned when the requested segment boundary isn't found.
    #[error("The requested segment boundary was not found.")]
    SegmentBoundaryNotFound,
    /// Returned when the URL doesn't have a host.
    #[error("The URL did not have a host.")]
    UrlDoesNotHaveAHost,
    /// Returned when attempting to set a part that cannot be [`None`] to [`None`].
    #[error("Attempted to set a part that cannot be None to None.")]
    PartCannotBeNone,
    /// Returned when trying to set a port to a value that isn't a number between 0 and 65535 (inclusive).
    #[error("The provided port is not a number between 0 and 65535 (inclusive).")]
    InvalidPort,
    /// Returned when the URL's host is not a domain.
    #[error("The URL's host is not a domain.")]
    HostIsNotADomain,
    /// Returned when attempting to set a URL's domain to an invalid domain.
    #[error("Attempted to set a URL's domain to an invalid domain. Perhaps trying to set the host instead would help?")]
    InvalidDomain,
    /// Returned when attempting to set a domain segment to an empty string.
    #[error("A domain segment cannot be empty.")]
    DomainSegmentCannotBeEmpty,
    /// Returned when attempting to set a domain's [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period to a value other than [`None`] and `.`.
    #[error("Attempted to set a domain's FQDN period to a value other than None and \".\".")]
    FqdnPeriodMustBeNoneOrPeriod,
    /// Returned when a [`url::ParseError)`] is encountered.
    #[error(transparent)] UrlParseError             (#[from] url::ParseError),
    /// Returned when a [`SetQueryParamError)`] is encountered.
    #[error(transparent)] SetQueryParamError        (#[from] SetQueryParamError),
    /// Returned when a [`SetIpHostError)`] is encountered.
    #[error(transparent)] SetIpHostError            (#[from] SetIpHostError),
    /// Returned when a [`SetSchemeError)`] is encountered.
    #[error(transparent)] SetSchemeError            (#[from] SetSchemeError),
    /// Returned when a [`SetUsernameError)`] is encountered.
    #[error(transparent)] SetUsernameError          (#[from] SetUsernameError),
    /// Returned when a [`SetPasswordError)`] is encountered.
    #[error(transparent)] SetPasswordError          (#[from] SetPasswordError),
    /// Returned when a [`SetSubdomainError)`] is encountered.
    #[error(transparent)] SetSubdomainError         (#[from] SetSubdomainError),
    /// Returned when a [`SetDdomainError)`] is encountered.
    #[error(transparent)] SetDomainError            (#[from] SetDomainError),
    /// Returned when a [`SetNotDomainSuffixError)`] is encountered.
    #[error(transparent)] SetNotDomainSuffixError   (#[from] SetNotDomainSuffixError),
    /// Returned when a [`SetDomainMiddleError)`] is encountered.
    #[error(transparent)] SetDomainMiddleError      (#[from] SetDomainMiddleError),
    /// Returned when a [`SetRegDomainError)`] is encountered.
    #[error(transparent)] SetRegDomainError         (#[from] SetRegDomainError),
    /// Returned when a [`SetDomainSuffixError)`] is encountered.
    #[error(transparent)] SetDomainSuffixError      (#[from] SetDomainSuffixError),
    /// Returned when a [`UrlDoesNotHavePathSegments)`] is encountered.
    #[error(transparent)] UrlDoesNotHavePathSegments(#[from] UrlDoesNotHavePathSegments),
    /// Returned when a [`SetPortError)`] is encountered.
    #[error(transparent)] SetPortError              (#[from] SetPortError),
    /// Returned when a [`SetDomainHostError`] is encountered.
    #[error(transparent)] SetDomainHostError        (#[from] SetDomainHostError)
}
