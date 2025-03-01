//! Provides [`UrlPart`] which allows for getting and setting various parts of a [`Url`].

use std::borrow::Cow;
use std::str::FromStr;

use url::{Url, Origin};
use thiserror::Error;
use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::util::*;

/// Getters and setters for various parts of a URL.
/// **Some parts may behave in unusual ways. Please check the documentation of parts you use to make sure you understand them.**
/// 
/// [`isize`] is used to allow for Python-style indexing from the end. `-1` is the last element, `-2` is the second last element, etc.
/// 
/// In general (except for domain parts on non-domain URLs and [`Self::PathSegment`]), setting a part to its own value is effectively a no-op.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum UrlPart {
    /// Prints debugging information about the contained [`Self`] and the details of its execution to STDERR.
    /// 
    /// Intended primarily for debugging logic errors.
    /// # Errors
    /// If the call to [`Self::get`]/[`Self::set`] returns an error, that error is returned after the debug info is printed.
    Debug(Box<Self>),
    /// The whole URL. Corresponds to [`Url::as_str`].
    /// # Getting
    /// Is never `None`.
    /// # Setting
    /// Cannot be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Whole.get(&BetterUrl::parse("https://example.com").unwrap()), Some(Cow::Borrowed("https://example.com/")));
    /// 
    /// let mut url=BetterUrl::parse("https://example.com").unwrap();
    /// UrlPart::Whole.set(&mut url, None).unwrap_err();
    /// assert_eq!(url.as_str(), "https://example.com/");
    /// UrlPart::Whole.set(&mut url, Some("https://example2.com")).unwrap();
    /// assert_eq!(url.as_str(), "https://example2.com/");
    /// UrlPart::Whole.set(&mut url, None).unwrap_err();
    /// ```
    Whole,
    /// The scheme. Corresponds to [`Url::scheme`].
    /// # Getting
    /// Is never `None`.
    /// # Setting
    /// Cannot be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Scheme.get(&BetterUrl::parse("https://example.com").unwrap()), Some(Cow::Borrowed("https")));
    /// assert_eq!(UrlPart::Scheme.get(&BetterUrl::parse("http://example.com" ).unwrap()), Some(Cow::Borrowed("http" )));
    /// assert_eq!(UrlPart::Scheme.get(&BetterUrl::parse("ftp://example.com"  ).unwrap()), Some(Cow::Borrowed("ftp"  )));
    /// 
    /// let mut url=BetterUrl::parse("https://example.com").unwrap();
    /// UrlPart::Scheme.set(&mut url, Some("http")).unwrap();
    /// assert_eq!(url.scheme(), "http");
    /// UrlPart::Scheme.set(&mut url, None).unwrap_err();
    /// ```
    Scheme,
    /// The scheme, host, and port
    /// # Getting
    /// Is never `None`.
    /// # Setting
    /// Cannot be `None`.
    /// The port is only set if [`Url::port_or_known_default`] does not equal the port in the origin.
    /// This means setting a URL's origin never adds a redundant port to it.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Origin.get(&BetterUrl::parse("https://example.com"    ).unwrap()), Some(Cow::Borrowed("https://example.com")));
    /// assert_eq!(UrlPart::Origin.get(&BetterUrl::parse("https://example.com:443").unwrap()), Some(Cow::Borrowed("https://example.com")));
    /// assert_eq!(UrlPart::Origin.get(&BetterUrl::parse("https://example.com:80" ).unwrap()), Some(Cow::Borrowed("https://example.com:80")));
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    /// UrlPart::Origin.set(&mut url, Some("https://example.com:443")).unwrap();
    /// assert_eq!(url.as_str(), "https://example.com/");
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    /// UrlPart::Origin.set(&mut url, Some("http://example.com:443")).unwrap();
    /// assert_eq!(url.as_str(), "http://example.com:443/");
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    /// UrlPart::Origin.set(&mut url, Some("http://example.com:80")).unwrap();
    /// assert_eq!(url.as_str(), "http://example.com/");
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    /// UrlPart::Origin.set(&mut url, Some("http://example.com")).unwrap();
    /// assert_eq!(url.as_str(), "http://example.com/");
    /// ```
    Origin,
    /// The username. Corresponds to [`Url::username`].
    /// # Getting
    /// Is never `None`.
    /// # Setting
    /// Cannot be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Username.get(&BetterUrl::parse("https://user:pass@example.com").unwrap()), Some(Cow::Borrowed("user")));
    /// assert_eq!(UrlPart::Username.get(&BetterUrl::parse("http://user:pass@example.com" ).unwrap()), Some(Cow::Borrowed("user")));
    /// assert_eq!(UrlPart::Username.get(&BetterUrl::parse("ftp://user:pass@example.com"  ).unwrap()), Some(Cow::Borrowed("user")));
    /// assert_eq!(UrlPart::Username.get(&BetterUrl::parse("https://example.com").unwrap()), Some(Cow::Borrowed("")));
    /// assert_eq!(UrlPart::Username.get(&BetterUrl::parse("http://example.com" ).unwrap()), Some(Cow::Borrowed("")));
    /// assert_eq!(UrlPart::Username.get(&BetterUrl::parse("ftp://example.com"  ).unwrap()), Some(Cow::Borrowed("")));
    /// 
    /// let mut url=BetterUrl::parse("https://example.com").unwrap();
    /// UrlPart::Username.set(&mut url, Some("test")).unwrap();
    /// assert_eq!(url.username(), "test");
    /// UrlPart::Username.set(&mut url, None).unwrap_err();
    /// ```
    Username,
    /// The password. Corresponds to [`Url::password`].
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Password.get(&BetterUrl::parse("https://user:pass@example.com").unwrap()), Some(Cow::Borrowed("pass")));
    /// assert_eq!(UrlPart::Password.get(&BetterUrl::parse("http://user:pass@example.com" ).unwrap()), Some(Cow::Borrowed("pass")));
    /// assert_eq!(UrlPart::Password.get(&BetterUrl::parse("ftp://user:pass@example.com"  ).unwrap()), Some(Cow::Borrowed("pass")));
    /// assert_eq!(UrlPart::Password.get(&BetterUrl::parse("https://example.com").unwrap()), None);
    /// assert_eq!(UrlPart::Password.get(&BetterUrl::parse("http://example.com" ).unwrap()), None);
    /// assert_eq!(UrlPart::Password.get(&BetterUrl::parse("ftp://example.com"  ).unwrap()), None);
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    /// UrlPart::Password.set(&mut url, Some("xyz")).unwrap();
    /// assert_eq!(url.as_str(), "https://:xyz@example.com/");
    /// ```
    Password,
    /// The host. Either a domain name or IPV4/6 address. Corresponds to [`Url::host`].
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Host.get(&BetterUrl::parse("https://127.0.0.1"      ).unwrap()), Some(Cow::Borrowed("127.0.0.1"      )));
    /// assert_eq!(UrlPart::Host.get(&BetterUrl::parse("https://www.example.com").unwrap()), Some(Cow::Borrowed("www.example.com")));
    /// assert_eq!(UrlPart::Host.get(&BetterUrl::parse("https://a.b.example.com").unwrap()), Some(Cow::Borrowed("a.b.example.com")));
    /// assert_eq!(UrlPart::Host.get(&BetterUrl::parse("https://example.com"    ).unwrap()), Some(Cow::Borrowed("example.com"    )));
    /// ```
    Host,
    /// [`Self::Host`] but with the `www.` at the start, if it exists, removed.
    ///
    /// Currently, for `www.abc.example.com` this returns `abc.example.com`, even though the [`Self::Subdomain`] is `www.abc`.
    /// # Getting
    /// Can be [`None`]
    /// # Setting
    /// Cannot be [`None`]
    /// 
    /// If the URL does not have a host ([`Url::host_str`] returns [`None`]), returns the error [`UrlPartGetError::UrlDoesNotHaveAHost`].
    /// 
    /// If [`Self::Host`] starts with `www.`, replaces the rest of the host.
    /// 
    /// If [`Self::Host`] does not start with `www.`, returns the error [`UrlPartSetError::HostDoesNotStartWithWWWDot`].
    HostWithoutWWWDotPrefix,
    /// The domain segment between segments N-1 and N.
    /// 
    /// Please note that, if a URL has N domain segments, setting `BeforeDomainSegment(N)` (the N+1th segment) will error even though it's reasonable to expect it to work like [`Self::NextDomainSegment`].
    /// 
    /// This may be changed in the future.
    /// # Getting
    /// Is always `None`.
    /// # Setting
    /// Can be `None`, but that's a no-op.
    /// # Set-get identity.
    /// On URLs without a host and URLs with a non-domain host, no guarantees are made regarding this part's set-get identity.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// let mut url=BetterUrl::parse("https://example.com/a/b/c").unwrap();
    /// UrlPart::BeforeDomainSegment(0).get(&url).is_none();
    /// UrlPart::BeforeDomainSegment(1).get(&url).is_none();
    /// UrlPart::BeforeDomainSegment(2).get(&url).is_none();
    ///
    /// UrlPart::BeforeDomainSegment(0).set(&mut url, Some("a")).unwrap();
    /// assert_eq!(url.domain(), Some("a.example.com"));
    /// UrlPart::BeforeDomainSegment(4).set(&mut url, Some("b")).unwrap_err();
    /// assert_eq!(url.domain(), Some("a.example.com"));
    /// UrlPart::BeforeDomainSegment(3).set(&mut url, Some("c")).unwrap();
    /// assert_eq!(url.domain(), Some("a.example.com.c"));
    /// UrlPart::BeforeDomainSegment(100).set(&mut url, Some("e")).unwrap_err();
    /// assert_eq!(url.domain(), Some("a.example.com.c"));
    /// ```
    BeforeDomainSegment(isize),
    /// The nth domain segment.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`, but that's a no-op.
    /// # Set-get identity.
    /// Trying to set an out-of-range segment to anything (even `None`) returns the error [`UrlPartGetError::SegmentNotFound`].
    /// This may be changed to a different error and/or work for some inputs that currently error.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// let mut url=BetterUrl::parse("https://a.b.c.example.com").unwrap();
    /// assert_eq!(UrlPart::DomainSegment(0).get(&url), Some(Cow::Borrowed("a")));
    /// assert_eq!(UrlPart::DomainSegment(1).get(&url), Some(Cow::Borrowed("b")));
    /// assert_eq!(UrlPart::DomainSegment(2).get(&url), Some(Cow::Borrowed("c")));
    /// assert_eq!(UrlPart::DomainSegment(3).get(&url), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::DomainSegment(4).get(&url), Some(Cow::Borrowed("com")));
    /// assert_eq!(UrlPart::DomainSegment(5).get(&url), None);
    ///
    /// UrlPart::DomainSegment(1).set(&mut url, Some("d")).unwrap();
    /// assert_eq!(url.domain().unwrap(), "a.d.c.example.com");
    /// UrlPart::DomainSegment(1).set(&mut url, None).unwrap();
    /// assert_eq!(url.domain().unwrap(), "a.c.example.com");
    /// UrlPart::DomainSegment(4).set(&mut url, Some("e")).unwrap_err();
    /// assert_eq!(url.domain().unwrap(), "a.c.example.com");
    /// ```
    DomainSegment(isize),
    /// The domain segment between segments N-1 and N.
    /// 
    /// Please note that, if a URL has N domain segments, setting `BeforeDomainSegment(N)` (the N+1th segment) will error even though it's reasonable to expect it to work like [`Self::NextDomainSegment`].
    /// 
    /// This may be changed in the future.
    /// # Getting
    /// Is always `None`.
    /// # Setting
    /// Can be `None`, but that's a no-op.
    /// # Set-get identity.
    /// On URLs without a host and URLs with a non-domain host, no guarantees are made regarding this part's set-get identity.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// let mut url=BetterUrl::parse("https://example.com/a/b/c").unwrap();
    /// UrlPart::AfterDomainSegment(0).get(&url).is_none();
    /// UrlPart::AfterDomainSegment(1).get(&url).is_none();
    /// UrlPart::AfterDomainSegment(2).get(&url).is_none();
    ///
    /// UrlPart::AfterDomainSegment(0).set(&mut url, Some("a")).unwrap();
    /// assert_eq!(url.domain(), Some("example.a.com"));
    /// UrlPart::AfterDomainSegment(4).set(&mut url, Some("b")).unwrap_err();
    /// assert_eq!(url.domain(), Some("example.a.com"));
    /// UrlPart::AfterDomainSegment(3).set(&mut url, Some("c")).unwrap_err();
    /// assert_eq!(url.domain(), Some("example.a.com"));
    /// UrlPart::AfterDomainSegment(2).set(&mut url, Some("d")).unwrap();
    /// assert_eq!(url.domain(), Some("example.a.com.d"));
    /// UrlPart::AfterDomainSegment(100).set(&mut url, Some("e")).unwrap_err();
    /// assert_eq!(url.domain(), Some("example.a.com.d"));
    /// ```
    AfterDomainSegment(isize),
    /// The subdomain. If the domain is `a.b.c.co.uk`, the value returned/changed by this is `a.b`.
    /// # Footguns
    /// This uses [`psl::domain_str`] which in turn uses [Mozilla's Public Suffix List](https://publicsuffix.org/) which has some... questionable decisions.
    /// 
    /// For example, the PSL contains `blogspot.com` as a suffix. This means Getting the [`Self::Subdomain`] of `https://blogspot.com` returns `None`, `https://name.blogspot.com` returns also returns `None`, and `https://abc.name.blogspot.com` returns `Some("abc")`.
    /// 
    /// This is stupid, but I can't do anything about it without introducing inconsistencies with other uses of [`psl`], which I currently consider worse.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Set-get identity.
    /// On URLs without a host and URLs with a non-domain host, no guarantees are made regarding this part's set-get identity.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Subdomain.get(&BetterUrl::parse("https://127.0.0.1"      ).unwrap()), None);
    /// assert_eq!(UrlPart::Subdomain.get(&BetterUrl::parse("https://www.example.com").unwrap()), Some(Cow::Borrowed("www")));
    /// assert_eq!(UrlPart::Subdomain.get(&BetterUrl::parse("https://a.b.example.com").unwrap()), Some(Cow::Borrowed("a.b")));
    /// assert_eq!(UrlPart::Subdomain.get(&BetterUrl::parse("https://example.com"    ).unwrap()), None);
    /// assert_eq!(UrlPart::Subdomain.get(&BetterUrl::parse("https://.example.com"   ).unwrap()), Some(Cow::Borrowed("")));
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    /// UrlPart::Subdomain.set(&mut url, Some("abc")).unwrap();
    /// assert_eq!(url.as_str(), "https://abc.example.com/");
    /// UrlPart::Subdomain.set(&mut url, Some("abc.def")).unwrap();
    /// assert_eq!(url.as_str(), "https://abc.def.example.com/");
    /// UrlPart::Subdomain.set(&mut url, Some("")).unwrap();
    /// assert_eq!(url.as_str(), "https://.example.com/");
    /// UrlPart::Subdomain.set(&mut url, None).unwrap();
    /// assert_eq!(url.as_str(), "https://example.com/");
    /// ```
    Subdomain,
    /// The domain's "registerable domain", AKA the middle and suffix. If the domain is `a.b.c.co.uk`, the value returned/changed by this is `c.co.uk`.
    ///
    /// Should always correspond to [`psl::domain_str`]. Guarantees are not made but differences are probably bugs.
    /// # Footguns
    /// This uses [`psl::domain_str`] which in turn uses [Mozilla's Public Suffix List](https://publicsuffix.org/) which has some... questionable decisions.
    /// 
    /// For example, the PSL contains `blogspot.com` as a suffix. This means Getting the [`Self::RegDomain`] of `https://blogspot.com` returns `None` and `https://name.blogspot.com` returns [`Some("name.blogspot.com")`].
    /// 
    /// This is stupid, but I can't do anything about it without introducing inconsistencies with other uses of [`psl`], which I currently consider worse.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Set-get identity.
    /// On URLs without a host and URLs with a non-domain host, no guarantees are made regarding this part's set-get identity.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::RegDomain.get(&BetterUrl::parse("https://127.0.0.1"      ).unwrap()), None);
    /// assert_eq!(UrlPart::RegDomain.get(&BetterUrl::parse("https://www.example.com").unwrap()), Some(Cow::Borrowed("example.com")));
    /// assert_eq!(UrlPart::RegDomain.get(&BetterUrl::parse("https://a.b.example.com").unwrap()), Some(Cow::Borrowed("example.com")));
    /// assert_eq!(UrlPart::RegDomain.get(&BetterUrl::parse("https://example.com"    ).unwrap()), Some(Cow::Borrowed("example.com")));
    ///
    /// let mut url = BetterUrl::parse("https://abc.example.com").unwrap();
    /// UrlPart::RegDomain.set(&mut url, Some("example.co.uk")).unwrap();
    /// assert_eq!(url.as_str(), "https://abc.example.co.uk/");
    /// UrlPart::RegDomain.set(&mut url, None).unwrap();
    /// assert_eq!(url.as_str(), "https://abc/");
    /// ```
    RegDomain,
    /// Similar to [`Self::RegDomain`] but specialized for only when [`Self::Subdomain`] is `"www"` or not present.
    /// 
    /// Exists to allow simulating the old `Rules::HostMap` behavior.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Footguns
    /// This uses [`psl::domain_str`] which in turn uses [Mozilla's Public Suffix List](https://publicsuffix.org/) which has some... questionable decisions.
    /// 
    /// For example, the PSL contains `blogspot.com` as a suffix. This means Getting the [`Self::MaybeWWWRegDomain`] of `https://blogspot.com` returns `None` and `https://www.blogspot.com` returns [`Some("www.blogspot.com")`].
    /// 
    /// This is stupid, but I can't do anything about it without introducing inconsistencies with other uses of [`psl`], which I currently consider worse.
    /// # Getting
    /// Is `None` when the URL's host is a domain with a subdomain that isn't `None` or `Some("www")`.
    /// # Setting
    /// If the URL's subdomain is `None` or `Some("www")`, behaves the same as [`Self::RegDomain`].
    MaybeWWWRegDomain,
    /// The domain's [`Self::Subdomain`] and [`Self::DomainMiddle`].
    ///
    /// Currently, if [`Self::Subdomain`] is [`None`], this is also [`None`].  
    /// Whether or not that's the best design is unclear. It may be changed.
    /// # Footguns
    /// This uses [`psl::domain_str`] which in turn uses [Mozilla's Public Suffix List](https://publicsuffix.org/) which has some... questionable decisions.
    /// 
    /// For example, the PSL contains `blogspot.com` as a suffix. This means Getting the [`Self::NotDomainSuffix`] of `https://blogspot.com` returns `None` and `https://name.blogspot.com` returns [`Some("name")`].
    /// 
    /// This is stupid, but I can't do anything about it without introducing inconsistencies with other uses of [`psl`], which I currently consider worse.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Set-get identity.
    /// On URLs without a host and URLs with a non-domain host, no guarantees are made regarding this part's set-get identity.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::NotDomainSuffix.get(&BetterUrl::parse("https://abc.example.com").unwrap()), Some(Cow::Borrowed("abc.example")));
    /// assert_eq!(UrlPart::NotDomainSuffix.get(&BetterUrl::parse("https://com").unwrap()            ), None);
    /// 
    /// let mut url = BetterUrl::parse("https://abc.example.com").unwrap();
    /// UrlPart::NotDomainSuffix.set(&mut url, Some("a")).unwrap();
    /// assert_eq!(url.as_str(), "https://a.com/");
    /// UrlPart::NotDomainSuffix.set(&mut url, None).unwrap();
    /// assert_eq!(url.as_str(), "https://com/");
    /// ```
    NotDomainSuffix,
    /// The `example` in `abc.example.com`.
    /// # Footguns
    /// This uses [`psl::domain_str`] which in turn uses [Mozilla's Public Suffix List](https://publicsuffix.org/) which has some... questionable decisions.
    /// 
    /// For example, the PSL contains `blogspot.com` as a suffix. This means Getting the [`Self::DomainMiddle`] of `https://blogspot.com` returns `None` and `https://name.blogspot.com` returns [`Some("name")`].
    /// 
    /// This is stupid, but I can't do anything about it without introducing inconsistencies with other uses of [`psl`], which I currently consider worse.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`, though the sensibility of the result is questionable.
    /// # Set-get identity.
    /// On URLs without a host and URLs with a non-domain host, no guarantees are made regarding this part's set-get identity.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::DomainMiddle.get(&BetterUrl::parse("https://example.com"     ).unwrap()), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::DomainMiddle.get(&BetterUrl::parse("https://example.com."    ).unwrap()), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::DomainMiddle.get(&BetterUrl::parse("https://example.co.uk"   ).unwrap()), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::DomainMiddle.get(&BetterUrl::parse("https://example.co.uk."  ).unwrap()), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::DomainMiddle.get(&BetterUrl::parse("https://a.example.com"   ).unwrap()), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::DomainMiddle.get(&BetterUrl::parse("https://a.example.com."  ).unwrap()), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::DomainMiddle.get(&BetterUrl::parse("https://a.example.co.uk" ).unwrap()), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::DomainMiddle.get(&BetterUrl::parse("https://a.example.co.uk.").unwrap()), Some(Cow::Borrowed("example")));
    /// 
    /// let mut url = BetterUrl::parse("https://example.com.").unwrap();
    /// UrlPart::DomainMiddle.set(&mut url, Some("example2")).unwrap();
    /// assert_eq!(url.as_str(), "https://example2.com./");
    /// 
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    /// UrlPart::DomainMiddle.set(&mut url, Some("example2")).unwrap();
    /// assert_eq!(url.as_str(), "https://example2.com/");
    /// 
    /// let mut url = BetterUrl::parse("https://.example.com.").unwrap();
    /// UrlPart::DomainMiddle.set(&mut url, Some("example2")).unwrap();
    /// assert_eq!(url.as_str(), "https://.example2.com./");
    /// 
    /// let mut url = BetterUrl::parse("https://.example.com").unwrap();
    /// UrlPart::DomainMiddle.set(&mut url, Some("example2")).unwrap();
    /// assert_eq!(url.as_str(), "https://.example2.com/");
    /// 
    /// let mut url = BetterUrl::parse("https://a.example.com.").unwrap();
    /// UrlPart::DomainMiddle.set(&mut url, Some("example2")).unwrap();
    /// assert_eq!(url.as_str(), "https://a.example2.com./");
    /// 
    /// let mut url = BetterUrl::parse("https://a.example.com").unwrap();
    /// UrlPart::DomainMiddle.set(&mut url, Some("example2")).unwrap();
    /// assert_eq!(url.as_str(), "https://a.example2.com/");
    /// ```
    DomainMiddle,
    /// Similar to [`Self::DomainMiddle`] but specialized for only when the subdomain is `"www"` or not present.
    /// 
    /// Exists to allow simulating the old `Rules::HostMap` behavior.
    /// # Footguns
    /// This uses [`psl::domain_str`] which in turn uses [Mozilla's Public Suffix List](https://publicsuffix.org/) which has some... questionable decisions.
    /// 
    /// For example, the PSL contains `blogspot.com` as a suffix. This means Getting the [`Self::MaybeWWWDomainMiddle`] of `https://blogspot.com` returns `None` and `https://name.blogspot.com` returns [`Some("name")`].
    /// 
    /// This is stupid, but I can't do anything about it without introducing inconsistencies with other uses of [`psl`], which I currently consider worse.
    /// # Getting
    /// Is `None` when the URL's host is a domain with a subdomain that isn't `None` or `Some("www")`.
    /// # Setting
    /// If the URL's subdomain is `None` or `Some("www")`, behaves the same as [`Self::RegDomain`].
    MaybeWWWDomainMiddle,
    /// The domain. Corresponds to [`Url::domain`].
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Set-get identity.
    /// On URLs without a host and URLs with a non-domain host, no guarantees are made regarding this part's set-get identity.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Domain.get(&BetterUrl::parse("https://127.0.0.1"      ).unwrap()), None);
    /// assert_eq!(UrlPart::Domain.get(&BetterUrl::parse("https://www.example.com").unwrap()), Some(Cow::Borrowed("www.example.com")));
    /// assert_eq!(UrlPart::Domain.get(&BetterUrl::parse("https://a.b.example.com").unwrap()), Some(Cow::Borrowed("a.b.example.com")));
    /// assert_eq!(UrlPart::Domain.get(&BetterUrl::parse("https://example.com"    ).unwrap()), Some(Cow::Borrowed("example.com")));
    ///
    /// let mut url = BetterUrl::parse("https://www.example.com").unwrap();
    /// UrlPart::Domain.set(&mut url, Some("example2.com")).unwrap();
    /// assert_eq!(url.as_str(), "https://example2.com/");
    /// UrlPart::Domain.set(&mut url, None).unwrap_err();
    /// ```
    Domain,
    /// # Footguns
    /// This uses [`psl::domain_str`] which in turn uses [Mozilla's Public Suffix List](https://publicsuffix.org/) which has some... questionable decisions.
    /// 
    /// For example, the PSL contains `blogspot.com` as a suffix. This means Getting the [`Self::DomainSuffix`] of `https://blogspot.com` returns `Some("blogspot.com")` and `https://name.blogspot.com` returns [`Some("blogspot.com")`].
    /// 
    /// This is stupid, but I can't do anything about it without introducing inconsistencies with other uses of [`psl`], which I currently consider worse.
    /// # Set-get identity.
    /// On URLs without a host and URLs with a non-domain host, no guarantees are made regarding this part's set-get identity.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::DomainSuffix.get(&BetterUrl::parse("https://example.com"   ).unwrap()), Some(Cow::Borrowed("com"  )), "https://example.com"   );
    /// assert_eq!(UrlPart::DomainSuffix.get(&BetterUrl::parse("https://example.com."  ).unwrap()), Some(Cow::Borrowed("com"  )), "https://example.com."  );
    /// assert_eq!(UrlPart::DomainSuffix.get(&BetterUrl::parse("https://example.co.uk" ).unwrap()), Some(Cow::Borrowed("co.uk")), "https://example.co.uk" );
    /// assert_eq!(UrlPart::DomainSuffix.get(&BetterUrl::parse("https://example.co.uk.").unwrap()), Some(Cow::Borrowed("co.uk")), "https://example.co.uk.");
    /// 
    /// let mut url = BetterUrl::parse("https://example.com.").unwrap();
    /// UrlPart::DomainSuffix.set(&mut url, Some("co.uk")).unwrap();
    /// assert_eq!(url.as_str(), "https://example.co.uk./");
    /// UrlPart::DomainSuffix.set(&mut url, None).unwrap();
    /// assert_eq!(url.as_str(), "https://example./");
    /// ```
    DomainSuffix,
    /// Useful only for appending a domain segment to a URL as the getter is always `None`.
    /// # Getting
    /// Is always `None`.
    /// # Setting
    /// Can be `None`, but that's a no-op.
    /// # Set-get identity.
    /// On URLs without a host and URLs with a non-domain host, no guarantees are made regarding this part's set-get identity.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::NextDomainSegment.get(&BetterUrl::parse("https://example.com").unwrap()), None);
    /// 
    /// let mut url=BetterUrl::parse("https://example.com").unwrap();
    /// UrlPart::NextDomainSegment.set(&mut url, Some("a")).unwrap();
    /// assert_eq!(url.domain(), Some("example.com.a"));
    /// UrlPart::NextDomainSegment.set(&mut url, Some("b")).unwrap();
    /// assert_eq!(url.domain(), Some("example.com.a.b"));
    /// UrlPart::NextDomainSegment.set(&mut url, Some("") ).unwrap_err();
    /// assert_eq!(url.domain(), Some("example.com.a.b"));
    /// UrlPart::NextDomainSegment.set(&mut url, Some("c")).unwrap();
    /// assert_eq!(url.domain(), Some("example.com.a.b.c"));
    /// UrlPart::NextDomainSegment.set(&mut url, None     ).unwrap();
    /// assert_eq!(url.domain(), Some("example.com.a.b.c"));
    /// ```
    NextDomainSegment,
    /// The port as a string. Corresponds to [`Url::port_or_known_default`].
    /// 
    /// Ports are strings for the sake of a simpler API.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Port.get(&BetterUrl::parse("https://example.com"    ).unwrap()), Some(Cow::Owned("443".to_string())));
    /// assert_eq!(UrlPart::Port.get(&BetterUrl::parse("https://example.com:443").unwrap()), Some(Cow::Owned("443".to_string())));
    /// assert_eq!(UrlPart::Port.get(&BetterUrl::parse("https://example.com:80" ).unwrap()), Some(Cow::Owned("80" .to_string())));
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    /// UrlPart::Port.set(&mut url, Some("80")).unwrap();
    /// assert_eq!(UrlPart::Port.get(&url), Some(Cow::Owned("80".to_string())));
    /// UrlPart::Port.set(&mut url, None).unwrap();
    /// assert_eq!(UrlPart::Port.get(&url), Some(Cow::Owned("443".to_string())));
    /// ```
    Port,
    /// The path segment between segments N-1 and N.
    /// # Getting
    /// Is always `None`.
    /// # Setting
    /// Can be `None`, but that's a no-op.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// let mut url=BetterUrl::parse("https://example.com/a/b/c").unwrap();
    /// UrlPart::BeforePathSegment(0).get(&url).is_none();
    /// UrlPart::BeforePathSegment(1).get(&url).is_none();
    /// UrlPart::BeforePathSegment(2).get(&url).is_none();
    ///
    /// UrlPart::BeforePathSegment(0).set(&mut url, Some("d")).unwrap();
    /// assert_eq!(url.path(), "/d/a/b/c");
    /// UrlPart::BeforePathSegment(5).set(&mut url, Some("e")).unwrap_err();
    /// assert_eq!(url.path(), "/d/a/b/c");
    /// UrlPart::BeforePathSegment(4).set(&mut url, Some("f")).unwrap();
    /// assert_eq!(url.path(), "/d/a/b/c/f");
    /// UrlPart::BeforePathSegment(100).set(&mut url, Some("h")).unwrap_err();
    /// assert_eq!(url.path(), "/d/a/b/c/f");
    /// ```
    BeforePathSegment(isize),
    /// A specific segment of the URL's path.
    /// 
    /// If the path is `"/a/b/c/"`, segment 0 is `"a`"`, 1 is `"b"`, 2 is `"c"`, and 3 is `""`.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Set-get identity.
    /// Trying to set an out-of-range segment to anything (even `None`) returns the error [`UrlPartGetError::SegmentNotFound`].
    /// This may be changed to a different error and/or work for some inputs that currently error.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::PathSegment(0).get(&BetterUrl::parse("https://example.com"     ).unwrap()), Some(Cow::Borrowed("")));
    /// assert_eq!(UrlPart::PathSegment(0).get(&BetterUrl::parse("https://example.com/a"   ).unwrap()), Some(Cow::Borrowed("a")));
    /// assert_eq!(UrlPart::PathSegment(1).get(&BetterUrl::parse("https://example.com/a"   ).unwrap()), None);
    /// assert_eq!(UrlPart::PathSegment(1).get(&BetterUrl::parse("https://example.com/a/"  ).unwrap()), Some(Cow::Borrowed("")));
    /// assert_eq!(UrlPart::PathSegment(1).get(&BetterUrl::parse("https://example.com/a/b" ).unwrap()), Some(Cow::Borrowed("b")));
    /// 
    /// let mut url=BetterUrl::parse("https://example.com/a/b/c/d").unwrap();
    /// UrlPart::PathSegment(1).set(&mut url, Some("e")).unwrap();
    /// assert_eq!(url.path(), "/a/e/c/d");
    /// UrlPart::PathSegment(1).set(&mut url, None).unwrap();
    /// assert_eq!(url.path(), "/a/c/d");
    /// ```
    PathSegment(isize),
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    ///
    /// assert_eq!(UrlPart::PathSegments {start: Some(-2), end: None   }.get(&BetterUrl::parse("https://example.com/0/1/2/3/4").unwrap()), Some(Cow::Owned("3/4".into())));
    /// assert_eq!(UrlPart::PathSegments {start: Some(-1), end: None   }.get(&BetterUrl::parse("https://example.com/0/1/2/3/4").unwrap()), Some(Cow::Owned("4".into())));
    /// assert_eq!(UrlPart::PathSegments {start: None    , end: None   }.get(&BetterUrl::parse("https://example.com/0/1/2/3/4").unwrap()), Some(Cow::Owned("0/1/2/3/4".into())));
    /// assert_eq!(UrlPart::PathSegments {start: Some( 0), end: None   }.get(&BetterUrl::parse("https://example.com/0/1/2/3/4").unwrap()), Some(Cow::Owned("0/1/2/3/4".into())));
    /// assert_eq!(UrlPart::PathSegments {start: Some( 1), end: None   }.get(&BetterUrl::parse("https://example.com/0/1/2/3/4").unwrap()), Some(Cow::Owned("1/2/3/4".into())));
    /// assert_eq!(UrlPart::PathSegments {start: Some( 2), end: None   }.get(&BetterUrl::parse("https://example.com/0/1/2/3/4").unwrap()), Some(Cow::Owned("2/3/4".into())));
    ///
    /// assert_eq!(UrlPart::PathSegments {start: None    , end: Some(1)}.get(&BetterUrl::parse("https://example.com/0/1/2/3/4").unwrap()), Some(Cow::Owned("0".into())));
    /// assert_eq!(UrlPart::PathSegments {start: Some( 0), end: Some(1)}.get(&BetterUrl::parse("https://example.com/0/1/2/3/4").unwrap()), Some(Cow::Owned("0".into())));
    ///
    /// assert_eq!(UrlPart::PathSegments {start: None    , end: Some(2)}.get(&BetterUrl::parse("https://example.com/0/1/2/3/4").unwrap()), Some(Cow::Owned("0/1".into())));
    /// assert_eq!(UrlPart::PathSegments {start: Some( 0), end: Some(2)}.get(&BetterUrl::parse("https://example.com/0/1/2/3/4").unwrap()), Some(Cow::Owned("0/1".into())));
    /// assert_eq!(UrlPart::PathSegments {start: Some( 1), end: Some(2)}.get(&BetterUrl::parse("https://example.com/0/1/2/3/4").unwrap()), Some(Cow::Owned("1".into())));
    ///
    /// let mut url = BetterUrl::parse("https://example.com/0/1/2/3/4").unwrap();
    ///
    /// UrlPart::PathSegments {start: Some(2), end: Some(-1)}.set(&mut url, Some("abc")).unwrap();
    /// assert_eq!(url.path(), "/0/1/abc/4");
    ///
    /// UrlPart::PathSegments {start: Some(1), end: Some(-1)}.set(&mut url, None).unwrap();
    /// assert_eq!(url.path(), "/0/4");
    ///
    /// UrlPart::PathSegments {start: Some(-1), end: None}.set(&mut url, None).unwrap();
    /// assert_eq!(url.path(), "/0");
    /// ```
    PathSegments {
        /// The start of the range of segments to get.
        ///
        /// Defaults to [`None`].
        #[serde(default, skip_serializing_if = "is_default")]
        start: Option<isize>,
        /// The end of the range of segments to get.
        ///
        /// Defaults to [`None`].
        #[serde(default, skip_serializing_if = "is_default")]
        end: Option<isize>
    },
    /// The path segment between segments N and N+1.
    /// # Getting
    /// Is always `None`.
    /// # Setting
    /// Can be `None`, but that's a no-op.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// let mut url=BetterUrl::parse("https://example.com/a/b/c").unwrap();
    /// UrlPart::AfterPathSegment(0).get(&url).is_none();
    /// UrlPart::AfterPathSegment(1).get(&url).is_none();
    /// UrlPart::AfterPathSegment(2).get(&url).is_none();
    ///
    /// UrlPart::AfterPathSegment(0).set(&mut url, Some("d")).unwrap();
    /// assert_eq!(url.path(), "/a/d/b/c");
    /// UrlPart::AfterPathSegment(5).set(&mut url, Some("e")).unwrap_err();
    /// assert_eq!(url.path(), "/a/d/b/c");
    /// UrlPart::AfterPathSegment(4).set(&mut url, Some("f")).unwrap_err();
    /// assert_eq!(url.path(), "/a/d/b/c");
    /// UrlPart::AfterPathSegment(3).set(&mut url, Some("g")).unwrap();
    /// assert_eq!(url.path(), "/a/d/b/c/g");
    /// UrlPart::AfterPathSegment(100).set(&mut url, Some("h")).unwrap_err();
    /// assert_eq!(url.path(), "/a/d/b/c/g");
    /// ```
    AfterPathSegment(isize),
    /// Useful only for appending a path segment to a URL as the getter is always `None`.
    /// Using this with a URL whose path ends in an empty segment (`https://example.com/a/b/`), the setter will overwrite that segment instead of leaving a random empty segment in the middle of the path.
    /// # Getting
    /// Is always `None`.
    /// # Setting
    /// Can be `None`, but that's a no-op.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::NextPathSegment.get(&BetterUrl::parse("https://example.com"   ).unwrap()), None);
    /// assert_eq!(UrlPart::NextPathSegment.get(&BetterUrl::parse("https://example.com/"  ).unwrap()), None);
    /// assert_eq!(UrlPart::NextPathSegment.get(&BetterUrl::parse("https://example.com/a" ).unwrap()), None);
    /// assert_eq!(UrlPart::NextPathSegment.get(&BetterUrl::parse("https://example.com/a/").unwrap()), None);
    /// 
    /// let mut url=BetterUrl::parse("https://example.com").unwrap();
    /// UrlPart::NextPathSegment.set(&mut url, Some("a")).unwrap();
    /// assert_eq!(url.path(), "/a");
    /// UrlPart::NextPathSegment.set(&mut url, Some("b")).unwrap();
    /// assert_eq!(url.path(), "/a/b");
    /// UrlPart::NextPathSegment.set(&mut url, Some("" )).unwrap();
    /// assert_eq!(url.path(), "/a/b/");
    /// UrlPart::NextPathSegment.set(&mut url, Some("" )).unwrap();
    /// assert_eq!(url.path(), "/a/b/");
    /// UrlPart::NextPathSegment.set(&mut url, Some("c")).unwrap();
    /// assert_eq!(url.path(), "/a/b/c");
    /// UrlPart::NextPathSegment.set(&mut url, None     ).unwrap();
    /// assert_eq!(url.path(), "/a/b/c");
    ///
    /// // Note that trailing empty path segments are replaced.
    /// let mut url=BetterUrl::parse("https://example.com/a/b/c/").unwrap();
    /// UrlPart::NextPathSegment.set(&mut url, Some("d")).unwrap();
    /// assert_eq!(url.path(), "/a/b/c/d");
    /// ```
    NextPathSegment,
    /// The path. Corresponds to [`Url::path`].
    /// Please note that all URLs with a path always have the path start with `/`.
    /// This is abstracted away in [`Self::PathSegment`] but not here.
    /// If a URL is [cannot-be-a-base](https://docs.rs/url/latest/url/struct.Url.html#method.cannot_be_a_base), getting the path will always return `None`. `Url::path` doesn't but given it's described as "an arbitrary string" I believe returning `None` is less surprising behaviour.
    /// # Getting
    /// Will be `None` when the URL is [cannot-be-a-base](https://docs.rs/url/latest/url/struct.Url.html#method.cannot_be_a_base).
    /// # Setting
    /// Can only be `None` when the URL is [cannot-be-a-base](https://docs.rs/url/latest/url/struct.Url.html#method.cannot_be_a_base) (always a no-op as it is already `None`).
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Path.get(&BetterUrl::parse("https://example.com"     ).unwrap()), Some(Cow::Borrowed("/"   )));
    /// assert_eq!(UrlPart::Path.get(&BetterUrl::parse("https://example.com/"    ).unwrap()), Some(Cow::Borrowed("/"   )));
    /// assert_eq!(UrlPart::Path.get(&BetterUrl::parse("https://example.com/a"   ).unwrap()), Some(Cow::Borrowed("/a"  )));
    /// assert_eq!(UrlPart::Path.get(&BetterUrl::parse("https://example.com/a"   ).unwrap()), Some(Cow::Borrowed("/a"  )));
    /// assert_eq!(UrlPart::Path.get(&BetterUrl::parse("https://example.com/a/"  ).unwrap()), Some(Cow::Borrowed("/a/" )));
    /// assert_eq!(UrlPart::Path.get(&BetterUrl::parse("https://example.com/a/b" ).unwrap()), Some(Cow::Borrowed("/a/b")));
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    /// UrlPart::Path.set(&mut url, Some("abc")).unwrap();
    /// assert_eq!(url.as_str(), "https://example.com/abc");
    /// UrlPart::Path.set(&mut url, Some("")).unwrap();
    /// assert_eq!(url.as_str(), "https://example.com/");
    /// UrlPart::Path.set(&mut url, None).unwrap_err();
    /// assert_eq!(url.as_str(), "https://example.com/");
    /// ```
    Path,
    /// Makes some internal stuff easier.
    PathWithoutLeadingSlash,
    /// A specific query parameter. The contained string is the parameter's name and the setter sets the parameter's value.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::QueryParam("a".into()).get(&BetterUrl::parse("https://example.com?a=2&b=3").unwrap()), Some(Cow::Borrowed("2")), "1");
    /// assert_eq!(UrlPart::QueryParam("c".into()).get(&BetterUrl::parse("https://example.com?a=2&b=3").unwrap()), None, "2");
    /// 
    /// let mut url=BetterUrl::parse("https://example.com?a=2&b=3").expect("3");
    /// UrlPart::QueryParam("b".into()).set(&mut url, Some("2")).expect("4");
    /// assert_eq!(url.query(), Some("a=2&b=2"), "5");
    /// UrlPart::QueryParam("c".into()).set(&mut url, Some("4")).expect("6");
    /// assert_eq!(url.query(), Some("a=2&b=2&c=4"), "7");
    /// UrlPart::QueryParam("b".into()).set(&mut url, None).expect("8");
    /// assert_eq!(url.query(), Some("a=2&c=4"), "9");
    /// UrlPart::QueryParam("a".into()).set(&mut url, None).expect("10");
    /// assert_eq!(url.query(), Some("c=4"), "11");
    /// UrlPart::QueryParam("c".into()).set(&mut url, None).expect("12");
    /// assert_eq!(url.query(), None, "13");
    /// UrlPart::QueryParam("d".into()).set(&mut url, Some("5")).expect("14");
    /// assert_eq!(url.query(), Some("d=5"), "15");
    ///
    /// UrlPart::QueryParam(QueryParamSelector {name: "d".into(), index: 1}).set(&mut url, Some("6")).expect("16");
    /// assert_eq!(url.query(), Some("d=5&d=6"), "17");
    /// UrlPart::QueryParam("e".into()).set(&mut url, Some("7")).expect("18");
    /// assert_eq!(url.query(), Some("d=5&d=6&e=7"), "19");
    /// UrlPart::QueryParam(QueryParamSelector {name: "d".into(), index: 2}).set(&mut url, Some("8")).expect("20");
    /// assert_eq!(url.query(), Some("d=5&d=6&e=7&d=8"), "21");
    /// UrlPart::QueryParam(QueryParamSelector {name: "d".into(), index: 1}).set(&mut url, None).expect("22");
    /// assert_eq!(url.query(), Some("d=5&e=7&d=8"), "23");
    /// ```
    QueryParam(QueryParamSelector),
    /// The query. Corresponds to [`Url::query`].
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Query.get(&BetterUrl::parse("https://example.com"        ).unwrap()), None);
    /// assert_eq!(UrlPart::Query.get(&BetterUrl::parse("https://example.com?a=2&b=3").unwrap()), Some(Cow::Borrowed("a=2&b=3")));
    /// 
    /// let mut url=BetterUrl::parse("https://example.com?a=2&b=3").unwrap();
    /// UrlPart::Query.set(&mut url, Some("c=4")).unwrap();
    /// assert_eq!(url.query(), Some("c=4"));
    /// UrlPart::Query.set(&mut url, None).unwrap();
    /// assert_eq!(url.query(), None);
    /// ```
    Query,
    /// The fragment. Corresponds to [`Url::fragment`].
    /// Please note that if the query is set to `Some("")`, the resulting URL will look like `https://example.com/?`.
    /// The mappers that filter query parameters will automatically set empty queries to `None`, but this currently does not.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Fragment.get(&BetterUrl::parse("https://example.com"  ).unwrap()), None);
    /// assert_eq!(UrlPart::Fragment.get(&BetterUrl::parse("https://example.com#a").unwrap()), Some(Cow::Borrowed("a")));
    /// 
    /// let mut url=BetterUrl::parse("https://example.com#abc").unwrap();
    /// UrlPart::Fragment.set(&mut url, Some("def")).unwrap();
    /// assert_eq!(url.fragment(), Some("def"));
    /// UrlPart::Fragment.set(&mut url, None).unwrap();
    /// assert_eq!(url.fragment(), None);
    /// ```
    Fragment,
    /// Please note that all URLs with a path always have the path start with `/`.
    /// This is abstracted away in [`Self::PathSegment`] but not here.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// let part = UrlPart::PartSegments {part: Box::new(UrlPart::Path), split: "/".to_string(), start: Some(1), end: Some(-1)};
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c/d/e").unwrap();
    /// 
    /// assert_eq!(part.get(&url), Some(Cow::Owned("a/b/c/d".to_string())));
    /// 
    /// part.set(&mut url, Some("x/y")).unwrap();
    /// assert_eq!(url.as_str(), "https://example.com/x/y/e");
    /// ```
    PartSegments {
        /// The part to get/set.
        part: Box<Self>,
        /// The string to split and join the part on and with.
        split: String,
        /// The start of the range of segments to get.
        /// 
        /// Defaults to `None`.
        #[serde(default, skip_serializing_if = "is_default")]
        start: Option<isize>,
        /// The end of the range of segments to get.
        /// 
        /// Defaults to `None`.
        #[serde(default, skip_serializing_if = "is_default")]
        end: Option<isize>
    },
    /// # Getting
    /// Is always `None`.
    /// # Setting
    /// Can be `None`, but that's a no-op.
    /// # Setting errors
    /// If getting the equivalent [`Self::PartSegment`] would return `None`, returns the error [`UrlPartGetError::SegmentNotFound`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// let mut url = BetterUrl::parse("https://abc.example.com").unwrap();
    /// let part = UrlPart::BeforePartSegment {part: Box::new(UrlPart::Domain), split: ".".to_string(), index: 1};
    /// assert_eq!(part.get(&url),  None);
    /// part.set(&mut url, Some("xyz")).unwrap();
    /// assert_eq!(url.domain(), Some("abc.xyz.example.com"));
    /// 
    /// let part = UrlPart::BeforePartSegment {part: Box::new(UrlPart::Domain), split: ".".to_string(), index: 4};
    /// part.set(&mut url, Some("error")).unwrap();
    /// ```
    BeforePartSegment {
        /// The part to get/set.
        part: Box<Self>,
        /// The value to split the part by.
        split: String,
        /// The index to get/insert before.
        index: isize
    },
    /// # Examples
    /// ```
    /// # use std::borrow::Cow;
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// let mut url = BetterUrl::parse("https://abc.example.com").unwrap();
    /// let part = UrlPart::PartSegment {part: Box::new(UrlPart::Domain), split: ".".to_string(), index: 1};
    /// assert_eq!(part.get(&url),  Some(Cow::Borrowed("example")));
    /// part.set(&mut url, Some("xyz")).unwrap();
    /// assert_eq!(url.domain(), Some("abc.xyz.com"));
    /// ```
    PartSegment {
        /// The part to get/set.
        part: Box<Self>,
        /// The value to split the part by.
        split: String,
        /// The index to get/insert at.
        index: isize
    },
    /// # Getting
    /// Is always `None`.
    /// # Setting
    /// Can be `None`, but that's a no-op.
    /// # Setting errors
    /// If getting the equivalent [`Self::PartSegment`] would return `None`, returns the error [`UrlPartGetError::SegmentNotFound`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// let mut url = BetterUrl::parse("https://abc.example.com").unwrap();
    /// let part = UrlPart::AfterPartSegment {part: Box::new(UrlPart::Domain), split: ".".to_string(), index: 1};
    /// assert_eq!(part.get(&url),  None);
    /// part.set(&mut url, Some("xyz")).unwrap();
    /// assert_eq!(url.domain(), Some("abc.example.xyz.com"));
    /// 
    /// let part = UrlPart::AfterPartSegment {part: Box::new(UrlPart::Domain), split: ".".to_string(), index: 4};
    /// part.set(&mut url, Some("error")).unwrap_err();
    /// ```
    AfterPartSegment {
        /// The part to get/set.
        part: Box<Self>,
        /// The value to split the part by.
        split: String,
        /// The index to get/insert after.
        index: isize
    },
    /// # Getting
    /// If the call to [`Self::get`]/[`Self::set`] returns `None`, instead return `Some(Cow::Borrowed(""))`
    /// # Setting
    /// If the value provided to [`Self::set`] is `None`, it is replaced with `Some("")`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::NoneToEmptyString(Box::new(UrlPart::Fragment)).get(&BetterUrl::parse("https://example.com").unwrap()), Some(Cow::Borrowed("")));
    /// 
    /// let mut url = BetterUrl::parse("https://example.com/abc").unwrap();
    /// UrlPart::NoneToEmptyString(Box::new(UrlPart::Path)).set(&mut url, None).unwrap();
    /// assert_eq!(url.path(), "/");
    /// ```
    NoneToEmptyString(Box<Self>)
}

/// Selector for the [`Self::index`] occurance of a query parameter named [`Self::name`]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub struct QueryParamSelector {
    /// The name of the query parameter.
    pub name: String,
    /// The index of the query parameter among query parameters named [`Self::name`].
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
    /// Gets the selected query parameter from a [`Url`].
    /// ```
    /// # use std::borrow::Cow;
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(QueryParamSelector {name: "b".into(), index: 1}.get(&Url::parse("https://example.com?a=1&b=2&b=3").unwrap()), Some(Cow::Borrowed("3")));
    /// ```
    pub fn get<'a>(&self, url: &'a Url) -> Option<Cow<'a, str>> {
        self.get_from_iter(url.query_pairs())
    }

    /// Gets the selected query parameter and its absolute index from a [`Url`].
    /// ```
    /// # use std::borrow::Cow;
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(QueryParamSelector {name: "b".into(), index: 1}.get_with_index(&Url::parse("https://example.com?a=1&b=2&b=3").unwrap()), Some((2usize, Cow::Borrowed("3"))));
    /// ```
    pub fn get_with_index<'a>(&self, url: &'a Url) -> Option<(usize, Cow<'a, str>)> {
        self.get_from_iter_with_index(url.query_pairs())
    }

    /// Gets the selected query parameter from an [`Iterator`].
    pub fn get_from_iter<I: IntoIterator<Item = (K, V)>, K: AsRef<str>, V>(&self, pairs: I) -> Option<V> {
        Some(pairs.into_iter().filter(|(name, _)| name.as_ref()==self.name).enumerate().find(|(i, _)| *i==self.index)?.1.1)
    }

    /// Gets the selected query parameter and its absolute index from an [`Iterator`].
    pub fn get_from_iter_with_index<I: IntoIterator<Item = (K, V)>, K: AsRef<str>, V>(&self, pairs: I) -> Option<(usize, V)> {
        pairs.into_iter().enumerate().filter(|(_, (name, _))| name.as_ref()==self.name).enumerate().find_map(|(ni, (ai, (_, v)))| (ni==self.index).then_some((ai, v)))
    }

    /// Sets the selected query parameter.
    /// # Errors
    /// If [`Self::index`] is more than 1 above the number of query params named [`Self::name`], returns the error [`SetQueryParamError::QueryParamIndexNotFound`].
    pub fn set(&self, url: &mut Url, to: Option<&str>) -> Result<(), SetQueryParamError> {
        Ok(match (to, url.query().map(|x| x.len()), self.index) {
            (Some(_ ), None     , 1..) => Err(SetQueryParamError::QueryParamIndexNotFound)?,
            (None    , None     , _  ) => {},
            (Some(to), Some(len), ni  ) => if let Some((ai, _)) = self.get_with_index(url) {
                url.set_query(Some(&form_urlencoded::Serializer::new(String::with_capacity(len))
                    .extend_pairs(url.query_pairs().enumerate().map(|(i, (name, value))| (name, if i==ai {Cow::Borrowed(to)} else {value}))).finish()));
            } else if url.query_pairs().filter(|(name, _)| **name==*self.name).count() == ni {
                url.query_pairs_mut().append_pair(&self.name, to);
            } else {
                Err(SetQueryParamError::QueryParamIndexNotFound)?;
            },
            (None    , Some(len), _) => if let Some((ai, _)) = self.get_with_index(url) {
                let new = form_urlencoded::Serializer::new(String::with_capacity(len)).extend_pairs(url.query_pairs().enumerate().filter_map(|(i, nv)| (i!=ai).then_some(nv))).finish();
                if new.is_empty() {
                    url.set_query(None);
                } else {
                    url.set_query(Some(&new));
                }
            },
            #[allow(clippy::arithmetic_side_effects, reason = "Can't happen.")]
            (Some(to), None     , 0) => url.set_query(Some(&form_urlencoded::Serializer::new(String::with_capacity(self.name.len() + to.len() + 1)).append_pair(&self.name, to).finish()))
        })
    }
}

/// The set of errors [`QueryParamSelector::set`] can return.
#[derive(Debug, Error)]
pub enum SetQueryParamError {
    /// Returned when a query parameter with the specified index cannot be set/created.
    #[error("A query parameter with the specified index could not be set/created.")]
    QueryParamIndexNotFound
}

impl UrlPart {
    /// Extracts the specified part of the provided URL.
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn get<'a>(&self, url: &'a BetterUrl) -> Option<Cow<'a, str>> {
        debug!(UrlPart::get, self, url);
        Some(match self {
            // Ordered hopefully most used to least used.

            // No shortcut conditions/mappers.

            Self::PathSegment(n)   => Cow::Borrowed(neg_nth(url.path_segments()?, *n)?),
            Self::QueryParam(selector) => selector.get(url)?,

            // Miscellaneous.

            Self::Query            => Cow::Borrowed(url.query()?),
            Self::Whole            => Cow::Borrowed(url.as_str()),
            Self::Host             => Cow::Borrowed(url.host_str()?),
            Self::HostWithoutWWWDotPrefix => Cow::Borrowed(url.host_str().map(|x| x.strip_prefix("www.").unwrap_or(x))?),
            Self::DomainSegment(n) => Cow::Borrowed(neg_nth(url.domain()?.split('.'), *n)?),
            Self::Domain           => if let Some(HostDetails::Domain(domain_details)) = url.host_details() {Cow::Borrowed(url.host_str()?.get(domain_details.domain_bounds    () )?)} else {None?},
            Self::Subdomain        => if let Some(HostDetails::Domain(domain_details)) = url.host_details() {Cow::Borrowed(url.host_str()?.get(domain_details.subdomain_bounds ()?)?)} else {None?},
            Self::NotDomainSuffix  => if let Some(HostDetails::Domain(domain_details)) = url.host_details() {Cow::Borrowed(url.host_str()?.get(domain_details.not_suffix_bounds()?)?)} else {None?},
            Self::DomainMiddle     => if let Some(HostDetails::Domain(domain_details)) = url.host_details() {Cow::Borrowed(url.host_str()?.get(domain_details.middle_bounds    ()?)?)} else {None?},
            Self::RegDomain        => if let Some(HostDetails::Domain(domain_details)) = url.host_details() {Cow::Borrowed(url.host_str()?.get(domain_details.reg_domain_bounds()?)?)} else {None?},
            Self::DomainSuffix     => if let Some(HostDetails::Domain(domain_details)) = url.host_details() {Cow::Borrowed(url.host_str()?.get(domain_details.suffix_bounds    ()?)?)} else {None?},
            Self::MaybeWWWDomainMiddle => if Self::Subdomain.get(url).is_none_or(|x| x=="www") {Self::DomainMiddle.get(url)?} else {None?},
            Self::MaybeWWWRegDomain => if Self::Subdomain.get(url).is_none_or(|x| x=="www") {Self::RegDomain.get(url)?} else {None?},
            Self::Port         => Cow::Owned(url.port_or_known_default()?.to_string()), // I cannot be bothered to add number handling.
            Self::Path         => if url.cannot_be_a_base() {None?} else {Cow::Borrowed(url.path())},
            Self::PathWithoutLeadingSlash => Cow::Borrowed((!url.cannot_be_a_base()).then_some(url.path())?.strip_prefix('/')?),

            Self::Origin => Cow::Owned(url.origin().unicode_serialization()),

            Self::PartSegments {part, split, start, end} => {
                // TODO: Change to always borrow when possible.
                Cow::Owned(neg_vec_keep(part.get(url)?.split(split), *start, *end)?.join(split))
            },
            Self::PathSegments {start, end} => {
                // TODO: Change to always borrow.
                Cow::Owned(neg_vec_keep(Self::Path.get(url)?.strip_prefix('/')?.split('/'), *start, *end)?.join("/"))
            },
            Self::PartSegment {part, split, index} => match part.get(url)? {
                Cow::Borrowed(v) => Cow::Borrowed(neg_nth(v.split(split), *index)?),
                Cow::Owned   (v) => Cow::Owned   (neg_nth(v.split(split), *index)?.to_owned())
            },

            // The things that are likely very rarely used.

            Self::Scheme                  => Cow::Borrowed(url.scheme()),
            Self::Username                => Cow::Borrowed(url.username()),
            Self::Password                => Cow::Borrowed(url.password()?),
            Self::Fragment                => Cow::Borrowed(url.fragment()?),
            Self::BeforeDomainSegment(_)  => None?,
            Self::AfterDomainSegment(_)   => None?,
            Self::NextDomainSegment       => None?,
            Self::BeforePathSegment(_)    => None?,
            Self::AfterPathSegment(_)     => None?,
            Self::NextPathSegment         => None?,
            Self::BeforePartSegment{..}   => None?,
            Self::AfterPartSegment{..}    => None?,

            // Miscellaneous.

            Self::NoneToEmptyString(part) => part.get(url).unwrap_or(Cow::Borrowed("")),
            Self::Debug(part) => {
                let ret = part.get(url);
                eprintln!("=== UrlPart::Debug ===\nUrlPart: {part:?}\nValue: {ret:?}");
                ret?
            }
        })
    }

    /// Replaces the specified part of the provided URL with the provided value.
    /// If this method returns an error, `url` is left unchanged.
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn set(&self, url: &mut BetterUrl, to: Option<&str>) -> Result<(), UrlPartSetError> {
        debug!(UrlPart::set, self, url, to);
        match (self, to) {
            (Self::Debug(part), _) => {
                let old = part.get(url).to_owned();
                eprintln!("=== UrlPart::Debug ===\nUrlPart: {part:?}\nOld value: {old:?}\nNew value: {to:?}");
                part.set(url, to)?;
            }
            // Ordered hopefully most used to least used.
            (Self::Query, _) => url.set_query(to),
            (Self::Host , _) => url.set_host (to)?,
            (Self::HostWithoutWWWDotPrefix, Some(to)) => match url.host_str().map(|host| host.starts_with("www.")) {
                Some(true) => url.set_host(Some(&format!("www.{to}")))?,
                Some(false) => Err(UrlPartSetError::HostDoesNotStartWithWWWDot)?,
                None => Err(UrlPartGetError::UrlDoesNotHaveAHost)?
            },
            (Self::Subdomain      , _) => url.set_subdomain        (to)?,
            (Self::NotDomainSuffix, _) => url.set_not_domain_suffix(to)?,
            (Self::DomainMiddle   , _) => url.set_domain_middle    (to)?,
            (Self::RegDomain      , _) => url.set_reg_domain       (to)?,
            (Self::DomainSuffix   , _) => url.set_domain_suffix    (to)?,
            (Self::MaybeWWWDomainMiddle, _) => if Self::Subdomain.get(url).is_none_or(|x| x=="www") {Self::DomainMiddle.set(url, to)?},
            (Self::MaybeWWWRegDomain, _) => if Self::Subdomain.get(url).is_none_or(|x| x=="www") {Self::RegDomain.set(url, to)?},
            (Self::Domain, Some(to)) => set_domain(url, to)?,
            (Self::NextDomainSegment, _) => if let Some(to) = to {
                if to.is_empty() {Err(UrlPartSetError::DomainSegmentCannotBeEmpty)?;}
                Self::Domain.set(url, Some(&match url.host_and_details() {
                    Some((host, HostDetails::Domain(DomainDetails {fqdn_period: Some(_), ..}))) => format!("{host}{to}."),
                    Some((host, HostDetails::Domain(DomainDetails {fqdn_period: None   , ..}))) => format!("{host}.{to}"),
                    _ => Err(UrlPartSetError::HostIsNotADomain)?
                }))?;
            },
            (Self::BeforeDomainSegment(n), _) => if let Some(to) = to {
                let mut segments = url.domain().ok_or(UrlPartGetError::HostIsNotADomain)?.split('.').collect::<Vec<_>>();
                let fixed_n=neg_range_boundary(*n, segments.len()).ok_or(UrlPartGetError::SegmentBoundaryNotFound)?;
                segments.insert(fixed_n, to);
                set_domain(url, &segments.join("."))?;
            },
            (Self::DomainSegment(n), _) => {
                let mut segments = url.domain().ok_or(UrlPartGetError::HostIsNotADomain)?.split('.').collect::<Vec<_>>();
                let fixed_n=neg_index(*n, segments.len()).ok_or(UrlPartGetError::SegmentNotFound)?;
                #[allow(clippy::indexing_slicing, reason = "`fixed_n` is guaranteed to be in bounds.")]
                match to {
                    Some(to) => segments[fixed_n]=to,
                    None     => {let _ = segments.remove(fixed_n);}
                }
                set_domain(url, &segments.join("."))?;
            },
            (Self::AfterDomainSegment(n), _) => if let Some(to) = to {
                let mut segments = url.domain().ok_or(UrlPartGetError::HostIsNotADomain)?.split('.').collect::<Vec<_>>();
                let fixed_n=neg_shifted_range_boundary(*n, segments.len(), 1).ok_or(UrlPartGetError::SegmentBoundaryNotFound)?;
                segments.insert(fixed_n, to);
                set_domain(url, &segments.join("."))?;
            },
            (Self::Port          , _) => url.set_port(to.map(|x| x.parse().map_err(|_| UrlPartSetError::InvalidPort)).transpose()?).map_err(|()| UrlPartSetError::CannotSetPort)?,
            (Self::Origin, Some(to)) => if let Origin::Tuple(scheme, host, port) = Url::parse(to)?.origin() {
                url.set_scheme(&scheme).map_err(|_| UrlPartSetError::CannotSetScheme)?;
                url.set_host(Some(&host.to_string()))?;
                if url.port_or_known_default()!=Some(port) {
                    url.set_port(Some(port)).map_err(|()| UrlPartSetError::CannotSetPort)?;
                }
            },
            (Self::PathSegment(-1), None) => {url.path_segments_mut().map_err(|_| UrlPartGetError::UrlDoesNotHaveAPath)?.pop();},
            (Self::BeforePathSegment(n), _) => if let Some(to) = to {
                let mut segments = url.path_segments().ok_or(UrlPartGetError::UrlDoesNotHaveAPath)?.collect::<Vec<_>>();
                let fixed_n = neg_range_boundary(*n, segments.len()).ok_or(UrlPartGetError::SegmentBoundaryNotFound)?;
                segments.insert(fixed_n, to);
                url.set_path(&segments.join("/"));
            },
            (Self::PathSegment(n), _) => {
                let mut segments = url.path_segments().ok_or(UrlPartGetError::UrlDoesNotHaveAPath)?.collect::<Vec<_>>();
                #[allow(clippy::indexing_slicing, reason = "`fixed_n` is guaranteed to be in bounds.")]
                match (neg_index(*n, segments.len()), to) {
                    (Some(fixed_n), Some(to)) => segments[fixed_n]=to,
                    (Some(fixed_n), None    ) => {let _ = segments.remove(fixed_n);}
                    (None         , Some(_ )) => Err(UrlPartGetError::SegmentNotFound)?,
                    (None         , None    ) => {}
                };
                url.set_path(&segments.join("/"));
            },
            (Self::PathSegments{start, end}, _) => {
                Self::PartSegments{part: Self::PathWithoutLeadingSlash.into(), split: "/".into(), start: *start, end: *end}.set(url, to)?;
            },
            (Self::AfterPathSegment(n), _) => if let Some(to) = to {
                let mut segments = url.path_segments().ok_or(UrlPartGetError::UrlDoesNotHaveAPath)?.collect::<Vec<_>>();
                let fixed_n = neg_shifted_range_boundary(*n, segments.len(), 1).ok_or(UrlPartGetError::SegmentBoundaryNotFound)?;
                segments.insert(fixed_n, to);
                url.set_path(&segments.join("/"));
            },
            (Self::NextPathSegment, _) => if let Some(to) = to {url.path_segments_mut().map_err(|()| UrlPartGetError::UrlDoesNotHaveAPath)?.pop_if_empty().push(to);},
            (Self::Path, _) => match (url.cannot_be_a_base(), to) {
                (false, Some(to)) => url.set_path(to),
                (false, None    ) => Err(UrlPartSetError::UrlMustHaveAPath)?,
                (true , Some(_) ) => Err(UrlPartSetError::UrlCannotHaveAPath)?,
                (true , None    ) => {}
            },
            (Self::PathWithoutLeadingSlash, _) => Self::Path.set(url, to.map(|x| format!("/{x}")).as_deref())?,
            (Self::QueryParam(selector), _) => url.set_query_param(selector, to)?,

            (Self::PartSegments {part, split, start, end}, _) => {
                let temp=part.get(url).ok_or(UrlPartGetError::PartIsNone)?;
                let mut temp2=temp.split(split).collect::<Vec<_>>();
                temp2.splice(neg_range(*start, *end, temp2.len()).ok_or(UrlPartGetError::SegmentRangeNotFound)?, to);
                part.set(url, Some(&temp2.join(split)))?;
            },
            (Self::BeforePartSegment{part, split, index}, _) => if let Some(to) = to {
                let temp = part.get(url).ok_or(UrlPartGetError::PartIsNone)?;
                let mut segments = temp.split(split).collect::<Vec<_>>();
                let fixed_n=neg_range_boundary(*index, segments.len()).ok_or(UrlPartGetError::SegmentNotFound)?;
                segments.insert(fixed_n, to);
                part.set(url, Some(&segments.join(split)))?;
            },
            (Self::PartSegment      {part, split, index}, _) => {
                let temp = part.get(url).ok_or(UrlPartGetError::PartIsNone)?;
                let mut segments = temp.split(split).collect::<Vec<_>>();
                let fixed_n=neg_index(*index, segments.len()).ok_or(UrlPartGetError::SegmentNotFound)?;
                if fixed_n==segments.len() {Err(UrlPartGetError::SegmentNotFound)?;}
                // fixed_n is guaranteed to be in bounds.
                #[allow(clippy::indexing_slicing, reason = "`fixed_n` is guaranteed to be in bounds.")]
                match to {
                    Some(to) => segments[fixed_n]=to,
                    None     => {segments.remove(fixed_n);}
                }
                part.set(url, Some(&segments.join(split)))?;
            },
            (Self::AfterPartSegment {part, split, index}, _) => if let Some(to) = to {
                let temp = part.get(url).ok_or(UrlPartGetError::PartIsNone)?;
                let mut segments = temp.split(split).collect::<Vec<_>>();
                let fixed_n=neg_shifted_range_boundary(*index, segments.len(), 1).ok_or(UrlPartGetError::SegmentNotFound)?;
                segments.insert(fixed_n, to);
                part.set(url, Some(&segments.join(split)))?;
            },
            (Self::NoneToEmptyString(part), _) => part.set(url, to.or(Some("")))?,

            // The things that are likely very rarely used.

            (Self::Whole   , Some(to)) => *url=BetterUrl::parse(to)?,
            (Self::Scheme  , Some(to)) => url.set_scheme  (to).map_err(|()| UrlPartSetError::CannotSetScheme)?,
            (Self::Username, Some(to)) => url.set_username(to).map_err(|()| UrlPartSetError::CannotSetUsername)?,
            (Self::Password, _       ) => url.set_password(to).map_err(|()| UrlPartSetError::CannotSetPassword)?,
            (Self::Fragment, _) => url.set_fragment(to),
            (_, None) => Err(UrlPartSetError::PartCannotBeNone)?
        }
        Ok(())
    }

    /// Internal method to make sure I don't accidentally commit Debug variants and other stuff unsuitable for the default config.
    #[allow(clippy::missing_const_for_fn, reason = "No reason to/consistency.")]
    pub(crate) fn is_suitable_for_release(&self, _config: &Config) -> bool {
        assert!(match self {
            Self::Debug(_) => false,
            _ => true
        }, "Unsuitable UrlPart detected: {self:?}");
        true
    }
}

/// When setting a domain it is generally ideal to make sure it's actually a domain.
/// 
/// Unfortunately [`Url`] doesn't have a `set_domain` method, so this checks if [`url::Host::parse`]ing `domain` returns a [`url::Host::Domain`].
/// # Errors
/// If [`is_valid_domain`] returns [`false`], returns the error [`UrlPartSetError::InvalidDomain`].
/// 
/// If [`is_valid_domain`] returns [`true`] but [`Url::set_host`] somehow returns an error, that error is returned.
fn set_domain(url: &mut BetterUrl, domain: &str) -> Result<(), UrlPartSetError> {
    if let Ok(host_details @ HostDetails::Domain(_)) = HostDetails::from_host_str(domain) {
        url.inner_mut().set_host(Some(domain))?;
        url.set_host_details(Some(host_details));
    } else {
        Err(UrlPartSetError::InvalidDomain)?;
    }
    Ok(())
}

/// The enum of all possible errors [`UrlPart::set`] (not a typo) can return when getting a URL part.
/// 
/// [`UrlPart::get`] returns an [`Option`], but it's still useful to keep this separate from [`UrlPartSetError`] as a kind of sub-error-thing for clarity.
#[derive(Debug, Error)]
pub enum UrlPartGetError {
    /// Returned by `UrlPart::Subdomain.get` when `UrlPart::Domain.get` returns `None`.
    #[error("The URL's host is not a domain.")]
    HostIsNotADomain,
    /// Urls that are [cannot-be-a-base](https://docs.rs/url/latest/url/struct.Url.html#method.cannot_be_a_base) don't have a path.
    #[error("Urls that are cannot-be-a-base don't have a path.")]
    UrlDoesNotHaveAPath,
    /// Returned when the requested segment is not found.
    #[error("The requested segment was not found.")]
    SegmentNotFound,
    /// Returned when the requested segment range is not found.
    #[error("The requested segment range was not found.")]
    SegmentRangeNotFound,
    /// Returned when [`UrlPart::get`] returns `None` where it has to return `Some`.
    #[error("The requested part was None.")]
    PartIsNone,
    /// Returned when the requested segment boundary is not found.
    #[error("The requested segment boundary was not found.")]
    SegmentBoundaryNotFound,
    /// Returned when the URL does not have a host.
    #[error("The URL did not have a host.")]
    UrlDoesNotHaveAHost
}

/// The enum of all possible errors [`UrlPart::set`] can return.
#[derive(Debug, Error)]
pub enum UrlPartSetError {
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`UrlPartGetError`] is encountered.
    #[error(transparent)]
    UrlPartGetError(#[from] UrlPartGetError),
    /// Returned when attempting to set a [`UrlPart`] that cannot be `None` to `None`.
    #[error("Attempted to set a part that cannot be None to None.")]
    PartCannotBeNone,
    /// Returned when a call to [`Url::set_scheme`] returns an error.
    #[error("The provided scheme would not have produced a valid URL.")]
    CannotSetScheme,
    /// Returned when attempting to set a port that is not a value [`u16`].
    #[error("The provided port is not a number between 0 and 65535 (inclusive).")]
    InvalidPort,
    /// Returned when a call to [`Url::set_port`] returns an error.
    #[error("Cannot set port for this URL. Either because it is cannot-be-a-base, does not have a host, or has the file scheme.")]
    CannotSetPort,
    /// Returned when a call to [`Url::set_username`] returns an error.
    #[error("Cannot set username for this URL. Either because it is cannot-be-a-base or does not have a host.")]
    CannotSetUsername,
    /// Returned when a call to [`Url::set_password`] returns an error.
    #[error("Cannot set password for this URL. Either because it is cannot-be-a-base or does not have a host.")]
    CannotSetPassword,
    /// Returned when attempting to remove the path of a URL that has to have a path.
    #[error("The URL must have a path as it is not cannot-be-a-base.")]
    UrlMustHaveAPath,
    /// Returned when attempting to set the part of a URL that cannot have a path.
    #[error("The URL cannot have a path as it is not cannot-be-a-base.")]
    UrlCannotHaveAPath,
    /// Returned when attempting to set a domain segment to `Some("")`.
    #[error("A domain segment cannot be empty.")]
    DomainSegmentCannotBeEmpty,
    /// Returned when setting [`UrlPart::Domain`] to a non-domain value.
    #[error("Attempted to set a URL's domain to an invalid domain. Perhaps trying to set the host instead would help?")]
    InvalidDomain,
    /// Returned when attempting to set a URL's not WWW domain but the URL's subdomain exists and is not www.
    #[error("Attempted to set a URL's not WWW domain but the URL's subdomain exists and is not www.")]
    HostIsNotMaybeWWWDomain,
    /// Returned when Attempting to set a URL's UrlPart::HostWithoutWWWDotPrefix when its UrlPart::Host does not start with \"www.\".
    #[error("Attempted to set a URL's UrlPart::HostWithoutWWWDotPrefix when its UrlPart::Host does not start with \"www.\".")]
    HostDoesNotStartWithWWWDot,
    /// Returned by `UrlPart::Subdomain.get` when `UrlPart::Domain.get` returns `None`.
    #[error("The URL's host is not a domain.")]
    HostIsNotADomain,
    /// Returneed when a [`SetQueryParamError`] is encountered.
    #[error(transparent)]
    SetQueryParamError(#[from] SetQueryParamError),
    /// Returned when trying to set the Subdomain of a domain that doesn't have a RegDomain.
    #[error("Attempted to set the Subdomain of a domain that doesn't have a RegDomain.")]
    DoesntHaveRegDomain,
    /// Returned when trying to set the Subdomain or DomainMiddle of a domain that doesn't have a DomainSuffix.
    #[error("Attempted to set the Subdomain or DomainMiddle of a domain that doesn't have a DomainSuffix")]
    DoesntHaveDomainSuffix
}

#[allow(clippy::unwrap_used, reason = "Panicking tests are easier to write than erroring tests.")]
#[cfg(test)]
mod tests {
    use super::*;

    const DOMAIN_URLS: [&str; 4] = [
        "https://example.com",
        "https://example.com?a=2",
        "https://abc.example.com/d/e?f=g&h=i#j",
        "http://abc.xyz.example.onion.co.uk/awawa/eeeee///AAAAA?l=g&b=t#q+"
    ];

    const IP_URLS: [&str; 2] = [
        "https://127.0.0.1",
        "https://127.0.0.1/awawa/eeeee///AAAAA?l=g&b=t#q+"
    ];

    macro_rules! identity_check {
        ($urls:expr, $($part:ident),+) => {
            identity_check_2!($urls, $(UrlPart::$part),+)
        };
    }

    macro_rules! identity_check_2 {
        ($urls:expr, $($part:expr),+) => {
            $(for mut url in $urls.iter().map(|url| BetterUrl::parse(url).unwrap()) {
                let old = $part.get(&url).map(|x| x.into_owned());
                $part.set(&mut url, old.as_deref()).expect(&format!("Set error: {:?}, {url:?}", $part));
                assert_eq!($part.get(&url).as_deref(), old.as_deref(), "Identity error: {:?}, {old:?}, {url:?}", $part);
            })+
        };
    }

    #[test]
    fn set_to_get_identity() {
        identity_check!(
            DOMAIN_URLS,
            Whole, Scheme, Username, Password, Host,
            Subdomain, RegDomain, NotDomainSuffix, DomainMiddle, Domain, DomainSuffix, NextDomainSegment,
            Port, NextPathSegment, Path, Query, Fragment
        );
        identity_check!(
            IP_URLS, 
            Whole, Scheme, Username, Password, Host,

            Port, NextPathSegment, Path, Query, Fragment
        );
        identity_check_2!(
            DOMAIN_URLS,
            UrlPart::BeforeDomainSegment(0),
            UrlPart::BeforeDomainSegment(1),
            UrlPart::DomainSegment(0),
            UrlPart::DomainSegment(1),
            UrlPart::BeforePathSegment(0),
            UrlPart::BeforePathSegment(1),
            UrlPart::PathSegment(0),
            UrlPart::PathSegment(1),
            UrlPart::QueryParam("a".into())
        );
        identity_check_2!(
            IP_URLS,
            UrlPart::BeforePathSegment(0),
            UrlPart::BeforePathSegment(1),
            UrlPart::PathSegment(0),
            UrlPart::PathSegment(1),
            UrlPart::QueryParam("a".into())
        );
    }
}
