use std::borrow::Cow;

use url::{Url, Origin, ParseError};
use thiserror::Error;
use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::config::Params;

/// Getters and setters for various parts of a URL.
/// In general (except for [`Self::DomainSegment`] and [`Self::PathSegment`]), setting a part to its own value is a no-op.
/// __Some parts may behave in unusual ways. Please check the documentation of parts you use to make sure you understand them.__
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum UrlPart {
    /// The whole URL. Corresponds to [`Url::as_str`].
    /// # Getting
    /// Is never `None`.
    /// # Setting
    /// Cannot be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Whole.get(&Url::parse("https://example.com").unwrap(), false), Some(Cow::Borrowed("https://example.com/")));
    /// 
    /// let mut url=Url::parse("https://example.com").unwrap();
    /// assert!(UrlPart::Whole.set(&mut url, None).is_err());
    /// assert_eq!(url.as_str(), "https://example.com/");
    /// assert!(UrlPart::Whole.set(&mut url, Some("https://example2.com")).is_ok());
    /// assert_eq!(url.as_str(), "https://example2.com/");
    /// assert!(UrlPart::Whole.set(&mut url, None).is_err());
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
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Scheme.get(&Url::parse("https://example.com").unwrap(), false), Some(Cow::Borrowed("https")));
    /// assert_eq!(UrlPart::Scheme.get(&Url::parse("http://example.com" ).unwrap(), false), Some(Cow::Borrowed("http" )));
    /// assert_eq!(UrlPart::Scheme.get(&Url::parse("ftp://example.com"  ).unwrap(), false), Some(Cow::Borrowed("ftp"  )));
    /// 
    /// let mut url=Url::parse("https://example.com").unwrap();
    /// assert!(UrlPart::Scheme.set(&mut url, Some("http")).is_ok());
    /// assert_eq!(url.scheme(), "http");
    /// assert!(UrlPart::Scheme.set(&mut url, None).is_err());
    /// ```
    Scheme,
    /// The scheme, host, and port
    /// # Getting
    /// Is never `None`.
    /// # Setting
    /// Cannot be `None`.
    /// The port is only set if [`Url::port_or_known_default`] does not equal the port in the origin.
    /// This means setting a URL's origin never adds a redundant port to it.
    Origin,
    /// The username. Corresponds to [`Url::username`].
    /// # Getting
    /// Is never `None`.
    /// # Setting
    /// Cannot be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Username.get(&Url::parse("https://user:pass@example.com").unwrap(), false), Some(Cow::Borrowed("user")));
    /// assert_eq!(UrlPart::Username.get(&Url::parse("http://user:pass@example.com" ).unwrap(), false), Some(Cow::Borrowed("user")));
    /// assert_eq!(UrlPart::Username.get(&Url::parse("ftp://user:pass@example.com"  ).unwrap(), false), Some(Cow::Borrowed("user")));
    /// assert_eq!(UrlPart::Username.get(&Url::parse("https://example.com").unwrap(), false), Some(Cow::Borrowed("")));
    /// assert_eq!(UrlPart::Username.get(&Url::parse("http://example.com" ).unwrap(), false), Some(Cow::Borrowed("")));
    /// assert_eq!(UrlPart::Username.get(&Url::parse("ftp://example.com"  ).unwrap(), false), Some(Cow::Borrowed("")));
    /// 
    /// let mut url=Url::parse("https://example.com").unwrap();
    /// assert!(UrlPart::Username.set(&mut url, Some("test")).is_ok());
    /// assert_eq!(url.username(), "test");
    /// assert!(UrlPart::Username.set(&mut url, None).is_err());
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
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Password.get(&Url::parse("https://user:pass@example.com").unwrap(), false), Some(Cow::Borrowed("pass")));
    /// assert_eq!(UrlPart::Password.get(&Url::parse("http://user:pass@example.com" ).unwrap(), false), Some(Cow::Borrowed("pass")));
    /// assert_eq!(UrlPart::Password.get(&Url::parse("ftp://user:pass@example.com"  ).unwrap(), false), Some(Cow::Borrowed("pass")));
    /// assert_eq!(UrlPart::Password.get(&Url::parse("https://example.com").unwrap(), false), None);
    /// assert_eq!(UrlPart::Password.get(&Url::parse("http://example.com" ).unwrap(), false), None);
    /// assert_eq!(UrlPart::Password.get(&Url::parse("ftp://example.com"  ).unwrap(), false), None);
    ///
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// assert!(UrlPart::Password.set(&mut url, Some("xyz")).is_ok());
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
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Host.get(&Url::parse("https://127.0.0.1"      ).unwrap(), false), Some(Cow::Borrowed("127.0.0.1"      )));
    /// assert_eq!(UrlPart::Host.get(&Url::parse("https://www.example.com").unwrap(), false), Some(Cow::Borrowed("www.example.com")));
    /// assert_eq!(UrlPart::Host.get(&Url::parse("https://a.b.example.com").unwrap(), false), Some(Cow::Borrowed("a.b.example.com")));
    /// assert_eq!(UrlPart::Host.get(&Url::parse("https://example.com"    ).unwrap(), false), Some(Cow::Borrowed("example.com"    )));
    /// ```
    Host,
    /// The domain segment between segments N-1 and N.
    /// Negative `n` values will get the `-n`'th last item similar to Python's `list[-x]` feature.
    /// Please note that, if a URL has N domain segments, setting `BeforeDomainSegment(N)` (the N+1th segment) will error even though it's reasonable to expect it to work like [`Self::NextDomainSegment`].
    /// This may be changed in the future.
    /// # Getting
    /// Is always `None`.
    /// # Setting
    /// Can be `None`, but that's a no-op.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// let mut url=Url::parse("https://example.com/a/b/c").unwrap();
    /// assert!(UrlPart::BeforeDomainSegment(0).get(&url, false).is_none());
    /// assert!(UrlPart::BeforeDomainSegment(1).get(&url, false).is_none());
    /// assert!(UrlPart::BeforeDomainSegment(2).get(&url, false).is_none());
    ///
    /// assert!(UrlPart::BeforeDomainSegment(0).set(&mut url, Some("a")).is_ok());
    /// assert_eq!(url.domain(), Some("a.example.com"));
    /// assert!(UrlPart::BeforeDomainSegment(4).set(&mut url, Some("b")).is_err());
    /// assert_eq!(url.domain(), Some("a.example.com"));
    /// assert!(UrlPart::BeforeDomainSegment(3).set(&mut url, Some("c")).is_err());
    /// assert_eq!(url.domain(), Some("a.example.com"));
    /// assert!(UrlPart::BeforeDomainSegment(2).set(&mut url, Some("d")).is_ok());
    /// assert_eq!(url.domain(), Some("a.example.d.com"));
    /// assert!(UrlPart::BeforeDomainSegment(100).set(&mut url, Some("e")).is_err());
    /// assert_eq!(url.domain(), Some("a.example.d.com"));
    /// ```
    BeforeDomainSegment(isize),
    /// The nth domain segment.
    /// Negative `n` values will get the `-n`'th last item similar to Python's `list[-x]` feature.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`, but that's a no-op.
    /// # Set-get identity.
    /// Trying to set an out-of-range segment to anything (even `None`) returns the error [`GetPartError::SegmentNotFound`].
    /// This may be changed to a different error and/or work for some inputs that currently error.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// let mut url=Url::parse("https://a.b.c.example.com").unwrap();
    /// assert_eq!(UrlPart::DomainSegment(0).get(&url, false), Some(Cow::Borrowed("a")));
    /// assert_eq!(UrlPart::DomainSegment(1).get(&url, false), Some(Cow::Borrowed("b")));
    /// assert_eq!(UrlPart::DomainSegment(2).get(&url, false), Some(Cow::Borrowed("c")));
    /// assert_eq!(UrlPart::DomainSegment(3).get(&url, false), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::DomainSegment(4).get(&url, false), Some(Cow::Borrowed("com")));
    /// assert_eq!(UrlPart::DomainSegment(5).get(&url, false), None);
    ///
    /// assert!(UrlPart::DomainSegment(1).set(&mut url, Some("d")).is_ok());
    /// assert_eq!(url.domain().unwrap(), "a.d.c.example.com");
    /// assert!(UrlPart::DomainSegment(1).set(&mut url, None).is_ok());
    /// assert_eq!(url.domain().unwrap(), "a.c.example.com");
    /// assert!(UrlPart::DomainSegment(4).set(&mut url, Some("e")).is_err());
    /// assert_eq!(url.domain().unwrap(), "a.c.example.com");
    /// ```
    DomainSegment(isize),
    /// The subdomain. If the domain is `a.b.c.co.uk`, the value returned/changed by this is `a.b`.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Subdomain.get(&Url::parse("https://127.0.0.1"      ).unwrap(), false), None);
    /// assert_eq!(UrlPart::Subdomain.get(&Url::parse("https://www.example.com").unwrap(), false), Some(Cow::Borrowed("www")));
    /// assert_eq!(UrlPart::Subdomain.get(&Url::parse("https://a.b.example.com").unwrap(), false), Some(Cow::Borrowed("a.b")));
    /// assert_eq!(UrlPart::Subdomain.get(&Url::parse("https://example.com"    ).unwrap(), false), None);
    /// assert_eq!(UrlPart::Subdomain.get(&Url::parse("https://.example.com"   ).unwrap(), false), Some(Cow::Borrowed("")));
    ///
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// assert!(UrlPart::Subdomain.set(&mut url, Some("abc")).is_ok());
    /// assert_eq!(url.as_str(), "https://abc.example.com/");
    /// assert!(UrlPart::Subdomain.set(&mut url, Some("abc.def")).is_ok());
    /// assert_eq!(url.as_str(), "https://abc.def.example.com/");
    /// assert!(UrlPart::Subdomain.set(&mut url, Some("")).is_ok());
    /// assert_eq!(url.as_str(), "https://.example.com/");
    /// assert!(UrlPart::Subdomain.set(&mut url, None).is_ok());
    /// assert_eq!(url.as_str(), "https://example.com/");
    /// ```
    Subdomain,
    /// The domain minus the subdomain. If the domain is `a.b.c.co.uk` value returned/changed by this is `c.co.uk`.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::NotSubdomain.get(&Url::parse("https://127.0.0.1"      ).unwrap(), false), None);
    /// assert_eq!(UrlPart::NotSubdomain.get(&Url::parse("https://www.example.com").unwrap(), false), Some(Cow::Borrowed("example.com")));
    /// assert_eq!(UrlPart::NotSubdomain.get(&Url::parse("https://a.b.example.com").unwrap(), false), Some(Cow::Borrowed("example.com")));
    /// assert_eq!(UrlPart::NotSubdomain.get(&Url::parse("https://example.com"    ).unwrap(), false), Some(Cow::Borrowed("example.com")));
    ///
    /// let mut url = Url::parse("https://abc.example.com").unwrap();
    /// assert!(UrlPart::NotSubdomain.set(&mut url, Some("example.co.uk")).is_ok());
    /// assert_eq!(url.as_str(), "https://abc.example.co.uk/");
    /// assert!(UrlPart::NotSubdomain.set(&mut url, None).is_ok());
    /// assert_eq!(url.as_str(), "https://abc/");
    /// ```
    NotSubdomain,
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::NotDomainSuffix.get(&Url::parse("https://abc.example.com").unwrap(), false), Some(Cow::Borrowed("abc.example")));
    /// assert_eq!(UrlPart::NotDomainSuffix.get(&Url::parse("https://com").unwrap()            , false), None);
    /// 
    /// let mut url = Url::parse("https://abc.example.com").unwrap();
    /// assert!(UrlPart::NotDomainSuffix.set(&mut url, Some("a")).is_ok());
    /// assert_eq!(url.as_str(), "https://a.com/");
    /// assert!(UrlPart::NotDomainSuffix.set(&mut url, None).is_ok());
    /// assert_eq!(url.as_str(), "https://com/");
    /// ```
    NotDomainSuffix,
    /// The `example` in `abc.example.com`.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`, though the sensibility of the result is questionable.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::UrlPart;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::NotSubdomainNotSuffix.get(&Url::parse("https://example.com"     ).unwrap(), false), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::NotSubdomainNotSuffix.get(&Url::parse("https://example.com."    ).unwrap(), false), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::NotSubdomainNotSuffix.get(&Url::parse("https://example.co.uk"   ).unwrap(), false), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::NotSubdomainNotSuffix.get(&Url::parse("https://example.co.uk."  ).unwrap(), false), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::NotSubdomainNotSuffix.get(&Url::parse("https://a.example.com"   ).unwrap(), false), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::NotSubdomainNotSuffix.get(&Url::parse("https://a.example.com."  ).unwrap(), false), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::NotSubdomainNotSuffix.get(&Url::parse("https://a.example.co.uk" ).unwrap(), false), Some(Cow::Borrowed("example")));
    /// assert_eq!(UrlPart::NotSubdomainNotSuffix.get(&Url::parse("https://a.example.co.uk.").unwrap(), false), Some(Cow::Borrowed("example")));
    /// 
    /// let mut url = Url::parse("https://example.com.").unwrap();
    /// assert!(UrlPart::NotSubdomainNotSuffix.set(&mut url, Some("example2")).is_ok());
    /// assert_eq!(url.as_str(), "https://example2.com./");
    /// 
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// assert!(UrlPart::NotSubdomainNotSuffix.set(&mut url, Some("example2")).is_ok());
    /// assert_eq!(url.as_str(), "https://example2.com/");
    /// 
    /// let mut url = Url::parse("https://.example.com.").unwrap();
    /// assert!(UrlPart::NotSubdomainNotSuffix.set(&mut url, Some("example2")).is_ok());
    /// assert_eq!(url.as_str(), "https://.example2.com./");
    /// 
    /// let mut url = Url::parse("https://.example.com").unwrap();
    /// assert!(UrlPart::NotSubdomainNotSuffix.set(&mut url, Some("example2")).is_ok());
    /// assert_eq!(url.as_str(), "https://.example2.com/");
    /// 
    /// let mut url = Url::parse("https://a.example.com.").unwrap();
    /// assert!(UrlPart::NotSubdomainNotSuffix.set(&mut url, Some("example2")).is_ok());
    /// assert_eq!(url.as_str(), "https://a.example2.com./");
    /// 
    /// let mut url = Url::parse("https://a.example.com").unwrap();
    /// assert!(UrlPart::NotSubdomainNotSuffix.set(&mut url, Some("example2")).is_ok());
    /// assert_eq!(url.as_str(), "https://a.example2.com/");
    /// ```
    NotSubdomainNotSuffix,
    /// The domain. Corresponds to [`Url::domain`].
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Domain.get(&Url::parse("https://127.0.0.1"      ).unwrap(), false), None);
    /// assert_eq!(UrlPart::Domain.get(&Url::parse("https://www.example.com").unwrap(), false), Some(Cow::Borrowed("www.example.com")));
    /// assert_eq!(UrlPart::Domain.get(&Url::parse("https://a.b.example.com").unwrap(), false), Some(Cow::Borrowed("a.b.example.com")));
    /// assert_eq!(UrlPart::Domain.get(&Url::parse("https://example.com"    ).unwrap(), false), Some(Cow::Borrowed("example.com")));
    ///
    /// let mut url = Url::parse("https://www.example.com").unwrap();
    /// assert!(UrlPart::Domain.set(&mut url, Some("example2.com")).is_ok());
    /// assert_eq!(url.as_str(), "https://example2.com/");
    /// assert!(UrlPart::Domain.set(&mut url, None).is_err());
    /// ```
    Domain,
    /// # Examples
    /// ```
    /// # use url_cleaner::types::UrlPart;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::DomainSuffix.get(&Url::parse("https://example.com"   ).unwrap(), false), Some(Cow::Borrowed("com"  )));
    /// assert_eq!(UrlPart::DomainSuffix.get(&Url::parse("https://example.com."  ).unwrap(), false), Some(Cow::Borrowed("com"  )));
    /// assert_eq!(UrlPart::DomainSuffix.get(&Url::parse("https://example.co.uk" ).unwrap(), false), Some(Cow::Borrowed("co.uk")));
    /// assert_eq!(UrlPart::DomainSuffix.get(&Url::parse("https://example.co.uk.").unwrap(), false), Some(Cow::Borrowed("co.uk")));
    /// 
    /// let mut url = Url::parse("https://example.com.").unwrap();
    /// assert!(UrlPart::DomainSuffix.set(&mut url, Some("co.uk")).is_ok());
    /// assert_eq!(url.as_str(), "https://example.co.uk/");
    /// assert!(UrlPart::DomainSuffix.set(&mut url, None).is_ok());
    /// assert_eq!(url.as_str(), "https://example/");
    /// ```
    DomainSuffix,
    /// Useful only for appending a domain segment to a URL as the getter is always `None`.
    /// # Getting
    /// Is always `None`.
    /// # Setting
    /// Can be `None`, but that's a no-op.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::NextDomainSegment.get(&Url::parse("https://example.com").unwrap(), false), None);
    /// 
    /// let mut url=Url::parse("https://example.com").unwrap();
    /// assert!(UrlPart::NextDomainSegment.set(&mut url, Some("a")).is_ok ());
    /// assert_eq!(url.domain(), Some("example.com.a"));
    /// assert!(UrlPart::NextDomainSegment.set(&mut url, Some("b")).is_ok ());
    /// assert_eq!(url.domain(), Some("example.com.a.b"));
    /// assert!(UrlPart::NextDomainSegment.set(&mut url, Some("") ).is_err());
    /// assert_eq!(url.domain(), Some("example.com.a.b"));
    /// assert!(UrlPart::NextDomainSegment.set(&mut url, Some("c")).is_ok ());
    /// assert_eq!(url.domain(), Some("example.com.a.b.c"));
    /// assert!(UrlPart::NextDomainSegment.set(&mut url, None     ).is_ok ());
    /// assert_eq!(url.domain(), Some("example.com.a.b.c"));
    /// ```
    NextDomainSegment,
    /// The port as a string. Corresponds to [`Url::port_or_known_default`].
    /// Ports are strings for the sake of a simpler API.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Port.get(&Url::parse("https://example.com"    ).unwrap(), false), Some(Cow::Owned("443".to_string())));
    /// assert_eq!(UrlPart::Port.get(&Url::parse("https://example.com:443").unwrap(), false), Some(Cow::Owned("443".to_string())));
    /// assert_eq!(UrlPart::Port.get(&Url::parse("https://example.com:80" ).unwrap(), false), Some(Cow::Owned("80" .to_string())));
    ///
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// assert!(UrlPart::Port.set(&mut url, Some("80")).is_ok());
    /// assert_eq!(UrlPart::Port.get(&url, false), Some(Cow::Owned("80".to_string())));
    /// assert!(UrlPart::Port.set(&mut url, None).is_ok());
    /// assert_eq!(UrlPart::Port.get(&url, false), Some(Cow::Owned("443".to_string())));
    /// ```
    Port,
    /// The path segment between segments N-1 and N.
    /// Negative `n` values will get the `-n`'th last item similar to Python's `list[-x]` feature.
    /// Please note that, if a URL has N path segments, setting `BeforePathSegment(N)` (the N+1th segment) will error even though it's reasonable to expect it to work like [`Self::NextPathSegment`].
    /// This may be changed in the future.
    /// # Getting
    /// Is always `None`.
    /// # Setting
    /// Can be `None`, but that's a no-op.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// let mut url=Url::parse("https://example.com/a/b/c").unwrap();
    /// assert!(UrlPart::BeforePathSegment(0).get(&url, false).is_none());
    /// assert!(UrlPart::BeforePathSegment(1).get(&url, false).is_none());
    /// assert!(UrlPart::BeforePathSegment(2).get(&url, false).is_none());
    ///
    /// assert!(UrlPart::BeforePathSegment(0).set(&mut url, Some("d")).is_ok());
    /// assert_eq!(url.path(), "/d/a/b/c");
    /// assert!(UrlPart::BeforePathSegment(5).set(&mut url, Some("e")).is_err());
    /// assert_eq!(url.path(), "/d/a/b/c");
    /// assert!(UrlPart::BeforePathSegment(4).set(&mut url, Some("f")).is_err());
    /// assert_eq!(url.path(), "/d/a/b/c");
    /// assert!(UrlPart::BeforePathSegment(3).set(&mut url, Some("g")).is_ok());
    /// assert_eq!(url.path(), "/d/a/b/g/c");
    /// assert!(UrlPart::BeforePathSegment(100).set(&mut url, Some("h")).is_err());
    /// assert_eq!(url.path(), "/d/a/b/g/c");
    /// ```
    BeforePathSegment(isize),
    /// A specific segment of the URL's path.
    /// Negative `n` values will get the `-n`'th last item similar to Python's `list[-x]` feature.
    /// Please note that for URLs that aren't cannot-be-a-base, `PathSegemnt(0)` will always be `Some`. On URLs that look like they don't have a path and/or only have a `/`, the value is `Some("")`.
    /// This is potentially unexpected but technically correct.
    /// As far as I know, all cases where this is a problem can be solved using [`crate::types::StringLocation`] on [`Self::Path`] or other combinations of existing tools.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Set-get identity.
    /// Trying to set an out-of-range segment to anything (even `None`) returns the error [`GetPartError::SegmentNotFound`].
    /// This may be changed to a different error and/or work for some inputs that currently error.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::PathSegment(0).get(&Url::parse("https://example.com"     ).unwrap(), false), Some(Cow::Borrowed("")));
    /// assert_eq!(UrlPart::PathSegment(0).get(&Url::parse("https://example.com/a"   ).unwrap(), false), Some(Cow::Borrowed("a")));
    /// assert_eq!(UrlPart::PathSegment(1).get(&Url::parse("https://example.com/a"   ).unwrap(), false), None);
    /// assert_eq!(UrlPart::PathSegment(1).get(&Url::parse("https://example.com/a/"  ).unwrap(), false), Some(Cow::Borrowed("")));
    /// assert_eq!(UrlPart::PathSegment(1).get(&Url::parse("https://example.com/a/b" ).unwrap(), false), Some(Cow::Borrowed("b")));
    /// 
    /// let mut url=Url::parse("https://example.com/a/b/c/d").unwrap();
    /// assert!(UrlPart::PathSegment(1).set(&mut url, Some("e")).is_ok());
    /// assert_eq!(url.path(), "/a/e/c/d");
    /// assert!(UrlPart::PathSegment(1).set(&mut url, None).is_ok());
    /// assert_eq!(url.path(), "/a/c/d");
    /// ```
    PathSegment(isize),
    /// Useful only for appending a path segment to a URL as the getter is always `None`.
    /// Using this with a URL whose path ends in an empty segment (`https://example.com/a/b/`), the setter will overwrite that segment instead of leaving a random empty segment in the middle of the path.
    /// # Getting
    /// Is always `None`.
    /// # Setting
    /// Can be `None`, but that's a no-op.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::NextPathSegment.get(&Url::parse("https://example.com"   ).unwrap(), false), None);
    /// assert_eq!(UrlPart::NextPathSegment.get(&Url::parse("https://example.com/"  ).unwrap(), false), None);
    /// assert_eq!(UrlPart::NextPathSegment.get(&Url::parse("https://example.com/a" ).unwrap(), false), None);
    /// assert_eq!(UrlPart::NextPathSegment.get(&Url::parse("https://example.com/a/").unwrap(), false), None);
    /// 
    /// let mut url=Url::parse("https://example.com").unwrap();
    /// assert!(UrlPart::NextPathSegment.set(&mut url, Some("a")).is_ok());
    /// assert_eq!(url.path(), "/a");
    /// assert!(UrlPart::NextPathSegment.set(&mut url, Some("b")).is_ok());
    /// assert_eq!(url.path(), "/a/b");
    /// assert!(UrlPart::NextPathSegment.set(&mut url, Some("" )).is_ok());
    /// assert_eq!(url.path(), "/a/b/");
    /// assert!(UrlPart::NextPathSegment.set(&mut url, Some("" )).is_ok());
    /// assert_eq!(url.path(), "/a/b/");
    /// assert!(UrlPart::NextPathSegment.set(&mut url, Some("c")).is_ok());
    /// assert_eq!(url.path(), "/a/b/c");
    /// assert!(UrlPart::NextPathSegment.set(&mut url, None     ).is_ok());
    /// assert_eq!(url.path(), "/a/b/c");
    ///
    /// // Note that trailing empty path segments are replaced.
    /// let mut url=Url::parse("https://example.com/a/b/c/").unwrap();
    /// assert!(UrlPart::NextPathSegment.set(&mut url, Some("d")).is_ok());
    /// assert_eq!(url.path(), "/a/b/c/d");
    /// ```
    NextPathSegment,
    /// The path. Corresponds to [`Url::path`].
    /// Please note that all URLs with a path always have the path start with `/`.
    /// This is abstracted away in [`Self::PathSegment`] but not here.
    /// If a URL is cannot-be-a-base, getting the path will always return `None`. `Url::path` doesn't but given it's described as "an arbitrary string" I believe returning `None` is less surprising behaviour.
    /// # Getting
    /// Will be `None` when the URL is cannot-be-a-base.
    /// # Setting
    /// Can only be `None` when the URL is cannot-be-a-base (always a no-op as it is already `None`).
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Path.get(&Url::parse("https://example.com"     ).unwrap(), false), Some(Cow::Borrowed("/"   )));
    /// assert_eq!(UrlPart::Path.get(&Url::parse("https://example.com/"    ).unwrap(), false), Some(Cow::Borrowed("/"   )));
    /// assert_eq!(UrlPart::Path.get(&Url::parse("https://example.com/a"   ).unwrap(), false), Some(Cow::Borrowed("/a"  )));
    /// assert_eq!(UrlPart::Path.get(&Url::parse("https://example.com/a"   ).unwrap(), false), Some(Cow::Borrowed("/a"  )));
    /// assert_eq!(UrlPart::Path.get(&Url::parse("https://example.com/a/"  ).unwrap(), false), Some(Cow::Borrowed("/a/" )));
    /// assert_eq!(UrlPart::Path.get(&Url::parse("https://example.com/a/b" ).unwrap(), false), Some(Cow::Borrowed("/a/b")));
    ///
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// assert!(UrlPart::Path.set(&mut url, Some("abc")).is_ok());
    /// assert_eq!(url.as_str(), "https://example.com/abc");
    /// assert!(UrlPart::Path.set(&mut url, Some("")).is_ok());
    /// assert_eq!(url.as_str(), "https://example.com/");
    /// assert!(UrlPart::Path.set(&mut url, None).is_err());
    /// assert_eq!(url.as_str(), "https://example.com/");
    /// ```
    Path,
    /// A specific query parameter. The contained string is the parameter's name and the setter sets the parameter's value.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::QueryParam("a".to_string()).get(&Url::parse("https://example.com?a=2&b=3").unwrap(), false), Some(Cow::Borrowed("2")));
    /// assert_eq!(UrlPart::QueryParam("c".to_string()).get(&Url::parse("https://example.com?a=2&b=3").unwrap(), false), None);
    /// 
    /// let mut url=Url::parse("https://example.com?a=2&b=3").unwrap();
    /// assert!(UrlPart::QueryParam("b".to_string()).set(&mut url, Some("2")).is_ok());
    /// assert_eq!(url.query(), Some("a=2&b=2"));
    /// assert!(UrlPart::QueryParam("c".to_string()).set(&mut url, Some("4")).is_ok());
    /// assert_eq!(url.query(), Some("a=2&b=2&c=4"));
    /// assert!(UrlPart::QueryParam("b".to_string()).set(&mut url, None).is_ok());
    /// assert_eq!(url.query(), Some("a=2&c=4"));
    /// assert!(UrlPart::QueryParam("a".to_string()).set(&mut url, None).is_ok());
    /// assert_eq!(url.query(), Some("c=4"));
    /// assert!(UrlPart::QueryParam("c".to_string()).set(&mut url, None).is_ok());
    /// assert_eq!(url.query(), None);
    /// assert!(UrlPart::QueryParam("d".to_string()).set(&mut url, Some("5")).is_ok());
    /// assert_eq!(url.query(), Some("d=5"));
    /// ```
    QueryParam(String),
    /// The query. Corresponds to [`Url::query`].
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Query.get(&Url::parse("https://example.com"        ).unwrap(), false), None);
    /// assert_eq!(UrlPart::Query.get(&Url::parse("https://example.com?a=2&b=3").unwrap(), false), Some(Cow::Borrowed("a=2&b=3")));
    /// 
    /// let mut url=Url::parse("https://example.com?a=2&b=3").unwrap();
    /// assert!(UrlPart::Query.set(&mut url, Some("c=4")).is_ok());
    /// assert_eq!(url.query(), Some("c=4"));
    /// assert!(UrlPart::Query.set(&mut url, None).is_ok());
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
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Fragment.get(&Url::parse("https://example.com"  ).unwrap(), false), None);
    /// assert_eq!(UrlPart::Fragment.get(&Url::parse("https://example.com#a").unwrap(), false), Some(Cow::Borrowed("a")));
    /// 
    /// let mut url=Url::parse("https://example.com#abc").unwrap();
    /// assert!(UrlPart::Fragment.set(&mut url, Some("def")).is_ok());
    /// assert_eq!(url.fragment(), Some("def"));
    /// assert!(UrlPart::Fragment.set(&mut url, None).is_ok());
    /// assert_eq!(url.fragment(), None);
    /// ```
    Fragment,
    /// Please note that all URLs with a path always have the path start with `/`.
    /// This is abstracted away in [`Self::PathSegment`] but not here.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::UrlPart;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// let part = UrlPart::PartSegments {part: Box::new(UrlPart::Path), split: "/".to_string(), start: Some(1), end: Some(-1)};
    /// let mut url = Url::parse("https://example.com/a/b/c/d/e").unwrap();
    /// 
    /// assert_eq!(part.get(&url, false), Some(Cow::Owned("a/b/c/d".to_string())));
    /// 
    /// assert!(part.set(&mut url, Some("x/y")).is_ok());
    /// assert_eq!(url.as_str(), "https://example.com/x/y/e");
    /// ```
    PartSegments {
        /// The part to get.
        part: Box<Self>,
        /// The string to split and join the part on and with.
        split: String,
        /// The start of the range of segments to get.
        start: Option<isize>,
        /// The end of the range of segments to get.
        end: Option<isize>
    }
}

impl UrlPart {
    /// Extracts the specified part of the provided URL.
    /// # Errors
    /// See [`Self`]'s documentation for which parts return `None` and when.
    /// If `none_to_empty_string` is `true`, this always returns `Some`.
    #[must_use]
    pub fn get<'a>(&self, url: &'a Url, none_to_empty_string: bool) -> Option<Cow<'a, str>> {
        #[cfg(feature = "debug")]
        println!("PartGet: {self:?}");
        let ret=self._get(url);
        if none_to_empty_string {
            ret.or(Some(Cow::Borrowed("")))
        } else {
            ret
        }
    }

    fn _get<'a>(&self, url: &'a Url) -> Option<Cow<'a, str>> {
        #[cfg(feature = "debug")]
        println!("_PartGet: {self:?}");
        match self {
            // Ordered hopefully most used to least used.

            // No shortcut conditions/mappers.

            Self::PathSegment(n)   => neg_nth(url.path_segments()?, *n).map(Cow::Borrowed),
            Self::QueryParam(name) => url.query_pairs().find(|(name2, _)| name==name2).map(|(_, v)| v),

            // Miscellaneous.

            Self::Query                  => url.query().map(Cow::Borrowed),
            Self::Whole                  => Some(Cow::Borrowed(url.as_str())),
            Self::Host                   => url.host_str().map(Cow::Borrowed),
            Self::DomainSegment(n)       => neg_nth(url.domain()?.split('.'), *n).map(Cow::Borrowed),
            Self::Subdomain              => {
                let url_domain=url.domain().map(|x| x.strip_suffix('.').unwrap_or(x))?;
                Some(Cow::Borrowed(url_domain.strip_suffix(psl::domain_str(url_domain)?)?.rsplit_once('.')?.0))
            },
            Self::NotSubdomain           => url.domain().and_then(psl::domain_str).map(Cow::Borrowed),
            Self::NotDomainSuffix        => {
                let domain=url.domain().map(|x| x.strip_suffix('.').unwrap_or(x))?;
                match domain.strip_suffix(psl::suffix_str(domain)?).map(|x| Cow::Borrowed(x.strip_suffix('.').unwrap_or(x))) {
                    Some(Cow::Borrowed("")) => None,
                    nds => nds
                }
            },
            Self::NotSubdomainNotSuffix  => {
                url.domain().map(|x| x.strip_suffix('.').unwrap_or(x).strip_suffix(psl::suffix_str(x)?))??
                    .rsplit('.').nth(1).map(Cow::Borrowed)
            },
            Self::Domain                 => url.domain().map(Cow::Borrowed),
            Self::DomainSuffix           => url.domain().and_then(psl::suffix_str).map(Cow::Borrowed),
            Self::Port                   => url.port_or_known_default().map(|port| Cow::Owned(port.to_string())), // I cannot be bothered to add number handling.
            Self::Path                   => if url.cannot_be_a_base() {None} else {Some(Cow::Borrowed(url.path()))},

            Self::Origin => Some(Cow::Owned(url.origin().unicode_serialization())),

            Self::PartSegments {part, split, start, end} => {
                Some(Cow::Owned(neg_vec_keep(part._get(url)?.split(split), *start, *end)?.join(split)))
            },

            // The things that are likely very rarely used.

            Self::Scheme                 => Some(Cow::Borrowed(url.scheme())),
            Self::Username               => Some(Cow::Borrowed(url.username())),
            Self::Password               => url.password().map(Cow::Borrowed),
            Self::Fragment               => url.fragment().map(Cow::Borrowed),
            Self::BeforeDomainSegment(_) => None,
            Self::NextDomainSegment      => None,
            Self::BeforePathSegment(_)   => None,
            Self::NextPathSegment        => None
        }
    }

    /// Replaces the specified part of the provided URL with the provided value.
    /// If this method returns an error, `url` is left unchanged.
    /// # Errors
    /// TODO
    #[allow(clippy::missing_panics_doc)]
    pub fn set(&self, url: &mut Url, to: Option<&str>) -> Result<(), SetPartError> {
        #[cfg(feature = "debug")]
        println!("PartSet: {self:?}");
        match (self, to) {
            // Ordered hopefully most used to least used.
            (Self::Query, _) => url.set_query(to),
            (Self::Host , _) => url.set_host (to)?,
            (Self::BeforeDomainSegment(n), _) => if let Some(to) = to {
                let fixed_n=neg_index(*n, url.domain().ok_or(GetPartError::HostIsNotADomain)?.split('.').count()).ok_or(GetPartError::SegmentNotFound)?;
                if fixed_n==url.domain().ok_or(GetPartError::HostIsNotADomain)?.split('.').count() {Err(GetPartError::SegmentNotFound)?;}
                url.set_host(Some(&url.domain().ok_or(GetPartError::HostIsNotADomain)?.split('.').take(fixed_n).chain([to]).chain(url.domain().ok_or(GetPartError::HostIsNotADomain)?.split('.').skip(fixed_n)).collect::<Vec<_>>().join(".")))?;
            },
            (Self::DomainSegment(n), _) => {
                let fixed_n=neg_index(*n, url.domain().ok_or(GetPartError::HostIsNotADomain)?.split('.').count()).ok_or(GetPartError::SegmentNotFound)?;
                if fixed_n==url.domain().ok_or(GetPartError::HostIsNotADomain)?.split('.').count() {Err(GetPartError::SegmentNotFound)?;}
                match to {
                    Some(to) => url.set_host(Some(&url.domain().ok_or(GetPartError::HostIsNotADomain)?.split('.').enumerate().       map(|(i, x)| if i==fixed_n {to} else {x}).collect::<Vec<_>>().join(".")))?,
                    None     => url.set_host(Some(&url.domain().ok_or(GetPartError::HostIsNotADomain)?.split('.').enumerate().filter_map(|(i, x)|   (i!=fixed_n).then_some(x)).collect::<Vec<_>>().join(".")))?
                }
            },
            (Self::Subdomain, _) => {
                match to {
                    Some(to) => {
                        let mut new_domain=to.to_string();
                        new_domain.push('.');
                        new_domain.push_str(&Self::NotSubdomain.get(url, false).ok_or(GetPartError::HostIsNotADomain)?);
                        url.set_host(Some(&new_domain))?;
                    },
                    None => {
                        #[allow(clippy::unnecessary_to_owned)]
                        url.set_host(Some(&Self::NotSubdomain.get(url, false).ok_or(GetPartError::HostIsNotADomain)?.into_owned()))?;
                    }
                }
            },
            (Self::NotSubdomain, _) => match to {
                Some(to) => {
                    let mut new_domain=Self::Subdomain.get(url, true).ok_or(GetPartError::HostIsNotADomain)?.to_string();
                    if !new_domain.is_empty() {
                        new_domain.push('.');
                    }
                    new_domain.push_str(to);
                    url.set_host(Some(&new_domain))?;
                },
                None => {
                    #[allow(clippy::unnecessary_to_owned)]
                    url.set_host(Some(&Self::Subdomain.get(url, true).ok_or(GetPartError::HostIsNotADomain)?.to_string()))?;
                }
            },
            #[allow(clippy::unnecessary_to_owned)]
            (Self::NotDomainSuffix, _) => match to {
                Some(to) => url.set_host(Some(&format!("{}.{}", to, Self::DomainSuffix.get(url, false).ok_or(GetPartError::HostIsNotADomain)?)))?,
                None     => url.set_host(Some(&Self::DomainSuffix.get(url, false).ok_or(GetPartError::HostIsNotADomain)?.to_string()))?
            },
            (Self::NotSubdomainNotSuffix, _) => {
                #[allow(clippy::useless_format)]
                url.set_host(Some(&match (Self::Subdomain.get(url, false), to, Self::DomainSuffix.get(url, false), Self::Domain.get(url, false).ok_or(GetPartError::HostIsNotADomain)?.ends_with('.')) {
                    // I do not know or care if any of these are impossible.
                    // Future me here: I care slightly. I'm pretty sure `(Some(_), None, Some(_), _)` can't happen.
                    (Some(subdomain), Some(to), Some(suffix), true ) => format!("{subdomain}.{to}.{suffix}."),
                    (Some(subdomain), Some(to), Some(suffix), false) => format!("{subdomain}.{to}.{suffix}"),
                    (Some(subdomain), Some(to), None        , true ) => format!("{subdomain}.{to}."),
                    (Some(subdomain), Some(to), None        , false) => format!("{subdomain}.{to}"),
                    (Some(subdomain), None    , Some(suffix), true ) => format!("{subdomain}.{suffix}."),
                    (Some(subdomain), None    , Some(suffix), false) => format!("{subdomain}.{suffix}"),
                    (Some(subdomain), None    , None        , true ) => format!("{subdomain}."),
                    (Some(subdomain), None    , None        , false) => format!("{subdomain}"),
                    (None           , Some(to), Some(suffix), true ) => format!("{to}.{suffix}."),
                    (None           , Some(to), Some(suffix), false) => format!("{to}.{suffix}"),
                    (None           , Some(to), None        , true ) => format!("{to}."),
                    (None           , Some(to), None        , false) => format!("{to}"),
                    (None           , None    , Some(suffix), true ) => format!("{suffix}."),
                    (None           , None    , Some(suffix), false) => format!("{suffix}"),
                    (None           , None    , None        , true ) => format!("."),
                    (None           , None    , None        , false) => format!("")
                }))?;
            },
            (Self::Domain        , _) => url.set_host(to)?,
            (Self::DomainSuffix  , _) => {
                let not_suffix=Self::NotDomainSuffix.get(url, false).ok_or(GetPartError::PartIsNone)?;
                let fqdn=url.domain().ok_or(GetPartError::HostIsNotADomain)?.ends_with('.');
                match &*match (to, fqdn) {
                    (Some(to), true ) => format!("{}{}.", not_suffix, to),
                    (Some(to), false) => format!("{}{}" , not_suffix, to),
                    (None, _) => not_suffix.to_string()
                } {
                    "" => url.set_host(None),
                    d => url.set_host(Some(d))
                }?;
            },
            (Self::NextDomainSegment, _) => if let Some(to) = to {
                url.set_host(Some(&url.domain().ok_or(GetPartError::HostIsNotADomain)?.split('.').chain([to]).collect::<Vec<_>>().join(".")))?
            },
            (Self::Port          , _) => url.set_port(to.map(|x| x.parse().map_err(|_| SetPartError::InvalidPort)).transpose()?).map_err(|()| SetPartError::CannotSetPort)?,
            (Self::Origin, Some(to)) => if let Origin::Tuple(scheme, host, port) = Url::parse(to)?.origin() {
                url.set_scheme(&scheme).map_err(|_| SetPartError::CannotSetScheme)?;
                #[allow(clippy::unnecessary_to_owned)]
                url.set_host(Some(&host.to_string()))?;
                if url.port_or_known_default()!=Some(port) {
                    url.set_port(Some(port)).map_err(|()| SetPartError::CannotSetPort)?;
                }
            },
            (Self::BeforePathSegment(n), _) => if let Some(to) = to {
                let fixed_n=neg_index(*n, url.path_segments().ok_or(GetPartError::UrlDoesNotHaveAPath)?.count()).ok_or(GetPartError::SegmentNotFound)?;
                if fixed_n==url.path_segments().ok_or(GetPartError::UrlDoesNotHaveAPath)?.count() {Err(GetPartError::SegmentNotFound)?;}
                url.set_path(&url.path_segments().ok_or(GetPartError::UrlDoesNotHaveAPath)?.take(fixed_n).chain([to]).chain(url.path_segments().ok_or(GetPartError::UrlDoesNotHaveAPath)?.skip(fixed_n)).collect::<Vec<_>>().join("/"));
            },
            (Self::PathSegment(n), _) => {
                let fixed_n=neg_index(*n, url.path_segments().ok_or(GetPartError::UrlDoesNotHaveAPath)?.count()).ok_or(GetPartError::SegmentNotFound)?;
                match to {
                    // Apparently `Iterator::intersperse` was stabilized but had issues with itertools. Very annoying.
                    Some(to) => url.set_path(&url.path_segments().ok_or(GetPartError::UrlDoesNotHaveAPath)?.enumerate().       map(|(i, x)| if i==fixed_n {to} else {x}).collect::<Vec<_>>().join("/")),
                    None     => url.set_path(&url.path_segments().ok_or(GetPartError::UrlDoesNotHaveAPath)?.enumerate().filter_map(|(i, x)|   (i!=fixed_n).then_some(x)).collect::<Vec<_>>().join("/")),
                }
            },
            (Self::NextPathSegment, _) => if let Some(to) = to {url.path_segments_mut().map_err(|()| GetPartError::UrlDoesNotHaveAPath)?.pop_if_empty().push(to);},
            (Self::Path, _) => match (url.cannot_be_a_base(), to) {
                (false, Some(to)) => url.set_path(to),
                (false, None    ) => Err(SetPartError::UrlMustHaveAPath)?,
                (true , Some(_) ) => Err(SetPartError::UrlCannotHaveAPath)?,
                (true , None    ) => {}
            },
            (Self::QueryParam(name), _) => {
                if let Some(to) = to {
                    if url.query().is_some() {
                        url.set_query(Some(&if url.query_pairs().any(|(name2, _)| name==&name2) {
                            form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().map(|(name2, value)| if name==&name2 {(name2, Cow::Borrowed(to))} else {(name2, value)})).finish()
                        } else {
                            form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().chain([(Cow::Borrowed(name.as_str()), Cow::Borrowed(to))])).finish()
                        }));
                    } else {
                        url.set_query(Some(&form_urlencoded::Serializer::new(String::new()).append_pair(name, to).finish()));
                    }
                } else {
                    let new_query=form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().filter(|(name2, _)| name!=name2)).finish();
                    url.set_query((!new_query.is_empty()).then_some(&new_query));
                }
            }

            (Self::PartSegments {part, split, start, end}, _) => {
                let temp=part._get(url).ok_or(GetPartError::PartIsNone)?;
                let mut temp2=temp.split(split).collect::<Vec<_>>();
                temp2.splice(neg_range(*start, *end, temp2.len()).ok_or(GetPartError::SegmentRangeNotFound)?, to);
                part.set(url, Some(&temp2.join(split)))?;
            },

            // The things that are likely very rarely used.

            (Self::Whole   , Some(to)) => *url=Url::parse(to)?,
            (Self::Scheme  , Some(to)) => url.set_scheme  (to).map_err(|()| SetPartError::CannotSetScheme)?,
            (Self::Username, Some(to)) => url.set_username(to).map_err(|()| SetPartError::CannotSetUsername)?,
            (Self::Password, _       ) => url.set_password(to).map_err(|()| SetPartError::CannotSetPassword)?,
            (Self::Fragment, _) => url.set_fragment(to),
            (_, None) => Err(SetPartError::PartCannotBeNone)?
        }
        Ok(())
    }

    /// Get the part from the provided URL and modify it according to the provided string modification rule.
    /// # Errors
    /// If [`UrlPart::get`] returns an error, that error is returned.
    /// If the string modification returns an error, that error is returned.
    /// If [`UrlPart::set`] returns an error, that error is returned.
    #[cfg(feature = "string-modification")]
    pub fn modify(&self, url: &mut Url, none_to_empty_string: bool, how: &StringModification, params: &Params) -> Result<(), PartModificationError> {
        let mut new_part=self.get(url, none_to_empty_string).ok_or(PartModificationError::PartIsNone)?.into_owned();
        how.apply(&mut new_part, params)?;
        self.set(url, Some(&new_part))?;
        Ok(())
    }
}

/// The enum of all possible errors [`UrlPart::set`] can return when getting a URL part.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum GetPartError {
    /// Returned by `UrlPart::Subdomain.get` when `UrlPart::Domain.get` returns `None`.
    #[error("The URL's host is not a domain.")]
    HostIsNotADomain,
    /// Urls that are cannot-be-a-base don't have a path.
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
    PartIsNone
}

/// The enum of all possible errors [`UrlPart::set`] can return.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SetPartError {
    /// Returned when a [`ParseError`] is encountered.
    #[error(transparent)]
    ParseError(#[from] ParseError),
    /// Returned when a [`GetPartError`] is encountered.
    #[error(transparent)]
    GetPartError(#[from] GetPartError),
    /// Returned when attempting to set a [`UrlPart`] that cannot be `None` to `None`.
    #[error("Attempted to set a part that cannot be None to None.")]
    PartCannotBeNone,
    /// Returned when a call to [`Url::set_scheme`] returns an error.
    #[error("The provided scheme would not have produced a valid URL.")]
    CannotSetScheme,
    /// Returned when attempting to set a port that is not a value [`u16`].
    #[error("The provided port is not a number, less than 0, or greater than 65535.")]
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
    UrlCannotHaveAPath
}

/// The enum of all possible errors [`UrlPart::modify`] can return.
#[cfg(feature = "string-modification")]
#[derive(Debug, Error)]
pub enum PartModificationError {
    /// Returned when the requested part is `None`.
    #[error("Cannot modify the requested part because it doesn't have a value.")]
    PartIsNone,
    /// Returned when a [`StringError`] is encountered.
    #[error(transparent)]
    StringError(#[from] StringError),
    /// Returned ehen a [`SetPartError`] is encountered.
    #[error(transparent)]
    SetPartError(#[from] SetPartError),
    /// Returned when a [`StringModificationError`] is encountered.
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError)
}

#[cfg(test)]
mod tests {
    use super::*;

    const URLS: [&str; 3] = [
        "https://example.com",
        "https://example.com?a=2",
        "https://abc.example.com/d/e?f=g&h=i#j"
    ];

    macro_rules! math_thing {
        ($part:ident) => {
            for mut url in URLS.iter().map(|url| Url::parse(url).unwrap()) {
                let old=UrlPart::$part.get(&url, false).map(Cow::into_owned);
                assert!(UrlPart::$part.set(&mut url, old.as_deref()).is_ok());
                assert_eq!(UrlPart::$part.get(&url, false).as_deref(), old.as_deref());
            }
        };
        ($part:ident, $($parts:ident),+) => {{
            math_thing!($part);
            math_thing!($($parts),+);
        }};
    }

    macro_rules! math_thing_2 {
        ($expr:expr) => {
            for mut url in URLS.iter().map(|url| Url::parse(url).unwrap()) {
                let old=$expr.get(&url, false).map(Cow::into_owned);
                assert!($expr.set(&mut url, old.as_deref()).is_ok());
                assert_eq!($expr.get(&url, false).as_deref(), old.as_deref());
            }
        };
        ($expr:expr, $($exprs:expr),+) => {{
            math_thing_2!($expr);
            math_thing_2!($($exprs),+);
        }};
    }
    
    #[test]
    fn set_to_get_identity() {
        math_thing!(Whole, Scheme, Username, Password, Host, Subdomain, NotSubdomain, Domain, Port, NextPathSegment, Path, Query, Fragment);
        math_thing_2!(
            // UrlPart::DomainSegment(0), UrlPart::DomainSegment(1), UrlPart::DomainSegment(2),
            // UrlPart::PathSegment(0), UrlPart::PathSegment(1), UrlPart::PathSegment(2),
            UrlPart::QueryParam("a".to_string())
        );
    }
}
