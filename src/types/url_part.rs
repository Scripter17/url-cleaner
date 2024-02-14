use std::borrow::Cow;

use url::{Url, ParseError};
use thiserror::Error;
use serde::{Serialize, Deserialize};

use super::{neg_nth, neg_index};

/// An enum that allows getting and setting various parts of a URL without a bajillion [`crate::rules::conditions::Condition`]s and [`crate::rules::mappers::Mapper`]s.
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
    /// The nth domain segment.
    /// Negative `n` values will get the `-n`'th last item similar to Python's `list[-x]` feature.
    /// # Getting
    /// Can be `None`.
    /// # Setting
    /// Can be `None`, but that's a no-op.
    /// # Set-get identity.
    /// Trying to set an out-of-range segment to anything (even `None`) returns the error [`PartError::SegmentNotFound`].
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
    /// assert_eq!(UrlPart::Subdomain.get(&Url::parse("https://example.com"    ).unwrap(), false), Some(Cow::Borrowed("")));
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
    /// assert!(UrlPart::Domain.set(&mut url, Some("example.co.uk")).is_ok());
    /// assert_eq!(url.as_str(), "https://example.co.uk/");
    /// assert!(UrlPart::Domain.set(&mut url, None).is_err());
    /// ```
    NotSubdomain,
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
    /// Useful only for inserting a path segment inside the a URL's path.
    /// Negative `n` values will get the `-n`'th last item similar to Python's `list[-x]` feature.
    /// Please note that, if a URL has 3 path segments, setting `BeforePathSegment(3)` (the 4th segment) will error even though it's reasonable to expect it to work like [`Self::NextPathSegment`].
    /// This may be changed in the future.
    /// # Getting
    /// Is always `None`.
    /// # Setting
    /// Cannot be `None`.
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
    /// Trying to set an out-of-range segment to anything (even `None`) returns the error [`PartError::SegmentNotFound`].
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
    /// assert!(UrlPart::NextPathSegment.set(&mut url, Some("") ).is_ok());
    /// assert_eq!(url.path(), "/a/b/");
    /// assert!(UrlPart::NextPathSegment.set(&mut url, Some("") ).is_ok());
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
    /// Please note that for URLs that are not cannot-be-a-base, the path is always `Some` and starts with `/`.
    /// If a URL is cannot-be-a-base, getting the path will always return `None`. `Url::path` doesn't but given it's described as "an arbitrary string" in this case I believe returning `None` is less surprising behaviour.
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
    Fragment
}

impl UrlPart {
    /// Extracts the specified part of the provided URL.
    /// # Errors
    /// See [`Self`]'s documentation for which parts return `None` and when.
    #[must_use]
    pub fn get<'a>(&self, url: &'a Url, none_to_empty_string: bool) -> Option<Cow<'a, str>> {
        let x=match self {
            // Ordered hopefully most used to least used.

            // No shortcut conditions/mappers.

            Self::PathSegment(n)   => neg_nth(url.path_segments()?, *n).map(Cow::Borrowed),
            Self::QueryParam(name) => url.query_pairs().find(|(name2, _)| name==name2).map(|(_, v)| v),

            // Miscelanious.

            Self::Query                => url.query().map(Cow::Borrowed),
            Self::Whole                => Some(Cow::Borrowed(url.as_str())),
            Self::Host                 => url.host_str().map(Cow::Borrowed),
            Self::DomainSegment(n)     => neg_nth(url.domain()?.split('.'), *n).map(Cow::Borrowed),
            Self::Subdomain            => url.domain()
                .and_then(|domain| domain.strip_suffix(psl::domain_str(domain)?).map(|subdomain_dot| Cow::Borrowed(subdomain_dot.strip_suffix('.').unwrap_or(subdomain_dot)))),
            Self::NotSubdomain         => url.domain().and_then(psl::domain_str).map(Cow::Borrowed),
            Self::Domain               => url.domain().map(Cow::Borrowed),
            Self::Port                 => url.port_or_known_default().map(|port| Cow::Owned(port.to_string())), // I cannot be bothered to add number handling.
            Self::Path                 => if url.cannot_be_a_base() {None} else {Some(Cow::Borrowed(url.path()))},

            // The things that are likely very rarely used.

            Self::Scheme               => Some(Cow::Borrowed(url.scheme())),
            Self::Username             => Some(Cow::Borrowed(url.username())),
            Self::Password             => url.password().map(Cow::Borrowed),
            Self::Fragment             => url.fragment().map(Cow::Borrowed),
            Self::BeforePathSegment(_) => None,
            Self::NextPathSegment      => None
        };
        if none_to_empty_string {
            Some(x.unwrap_or(Cow::Borrowed("")))
        } else {
            x
        }
    }

    /// Replaces the specified part of the provided URL with the provided value.
    /// If this method returns an error, `url` is left unchanged.
    /// # Errors
    /// TODO
    pub fn set(&self, url: &mut Url, to: Option<&str>) -> Result<(), PartError> {
        match (self, to) {
            // Ordered hopefully most used to least used.
            (Self::Query, _) => url.set_query(to),
            (Self::Host , _) => url.set_host (to)?,
            (Self::DomainSegment(n), _) => {
                let fixed_n=neg_index(*n, url.domain().ok_or(PartError::HostIsNotADomain)?.split('.').count()).ok_or(PartError::SegmentNotFound)?;
                if fixed_n==url.domain().ok_or(PartError::HostIsNotADomain)?.split('.').count() {Err(PartError::SegmentNotFound)?;}
                match to {
                    Some(to) => url.set_host(Some(&url.domain().ok_or(PartError::HostIsNotADomain)?.split('.').enumerate().       map(|(i, x)| if i==fixed_n {to} else {x}).collect::<Vec<_>>().join(".")))?,
                    None     => url.set_host(Some(&url.domain().ok_or(PartError::HostIsNotADomain)?.split('.').enumerate().filter_map(|(i, x)|   (i!=fixed_n).then_some(x)).collect::<Vec<_>>().join(".")))?
                }
            }
            (Self::Subdomain, _) => {
                match to {
                    Some(to) => {
                        let mut new_domain=to.to_string();
                        new_domain.push('.');
                        new_domain.push_str(&Self::NotSubdomain.get(url, false).ok_or(PartError::HostIsNotADomain)?);
                        url.set_host(Some(&new_domain))?;
                    },
                    None => {
                        #[allow(clippy::unnecessary_to_owned)]
                        url.set_host(Some(&Self::NotSubdomain.get(url, false).ok_or(PartError::HostIsNotADomain)?.into_owned()))?;
                    }
                }
            },
            (Self::NotSubdomain, Some(to)) => {
                let mut new_domain=Self::Subdomain.get(url, false).ok_or(PartError::HostIsNotADomain)?.to_string();
                new_domain.push('.');
                new_domain.push_str(to);
                url.set_host(Some(&new_domain))?;
            },
            (Self::Domain        , _) => url.set_host(to)?,
            (Self::Port          , _) => url.set_port(to.map(|x| x.parse().map_err(|_| PartError::InvalidPort)).transpose()?).map_err(|()| PartError::CannotSetPort)?,
            (Self::BeforePathSegment(n), _) => if let Some(to) = to {
                let fixed_n=neg_index(*n, url.path_segments().ok_or(PartError::UrlDoesNotHavePath)?.count()).ok_or(PartError::SegmentNotFound)?;
                if fixed_n==url.path_segments().ok_or(PartError::UrlDoesNotHavePath)?.count() {Err(PartError::SegmentNotFound)?;}
                url.set_path(&url.path_segments().ok_or(PartError::UrlDoesNotHavePath)?.take(fixed_n).chain([to]).chain(url.path_segments().ok_or(PartError::UrlDoesNotHavePath)?.skip(fixed_n)).collect::<Vec<_>>().join("/"));
            },
            (Self::PathSegment(n), _) => {
                let fixed_n=neg_index(*n, url.path_segments().ok_or(PartError::UrlDoesNotHavePath)?.count()).ok_or(PartError::SegmentNotFound)?;
                match to {
                    // Apparently `Iterator::intersperse` was stabilized but had issues with itertools. Very annoying.
                    Some(to) => url.set_path(&url.path_segments().ok_or(PartError::UrlDoesNotHavePath)?.enumerate().       map(|(i, x)| if i==fixed_n {to} else {x}).collect::<Vec<_>>().join("/")),
                    None     => url.set_path(&url.path_segments().ok_or(PartError::UrlDoesNotHavePath)?.enumerate().filter_map(|(i, x)|   (i!=fixed_n).then_some(x)).collect::<Vec<_>>().join("/")),
                }
            },
            (Self::NextPathSegment, _) => if let Some(to) = to {url.path_segments_mut().map_err(|()| PartError::UrlDoesNotHavePath)?.pop_if_empty().push(to);},
            (Self::Path, _) => match (url.cannot_be_a_base(), to) {
                (false, Some(to)) => url.set_path(to),
                (false, None    ) => Err(PartError::UrlMustHavePath)?,
                (true , Some(_) ) => Err(PartError::UrlCannotHavePath)?,
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

            // The things that are likely very rarely used.

            (Self::Whole   , Some(to)) => *url=Url::parse(to)?,
            (Self::Scheme  , Some(to)) => url.set_scheme  (to).map_err(|()| PartError::CannotSetScheme)?,
            (Self::Username, Some(to)) => url.set_username(to).map_err(|()| PartError::CannotSetUsername)?,
            (Self::Password, _       ) => url.set_password(to).map_err(|()| PartError::CannotSetPassword)?,
            (Self::Fragment, _) => url.set_fragment(to),
            (_, None) => Err(PartError::PartCannotBeNone)?
        }
        Ok(())
    }

    /// Get the part from the provided URL and modify it according to the provided string modification rule.
    /// # Errors
    /// If [`UrlPart::get`] returns an error, that error is returned.
    /// If the string modification returns an error, that error is returned.
    /// If [`UrlPart::set`] returns an error, that error is returned.
    pub fn modify(&self, url: &mut Url, none_to_empty_string: bool, how: &super::StringModification) -> Result<(), PartModificationError> {
        let mut new_part=self.get(url, none_to_empty_string).ok_or(PartModificationError::PartCannotBeNone)?.into_owned();
        how.apply(&mut new_part)?;
        self.set(url, Some(&new_part))?;
        Ok(())
    }
}

/// An enum of all possible errors [`UrlPart::set`] can return.
#[derive(Debug, Error)]
pub enum PartError {
    /// Attempted replacement would not produce a valid URL.
    #[error(transparent)]
    ParseError(#[from] ParseError),
    /// [`UrlPart::set`] attempted to set a part that cannot be None to None.
    #[error("UrlPart::set attempted to set a part that cannot be None to None.")]
    PartCannotBeNone,
    /// The provided scheme would not have produced a valid URL.
    #[error("The provided scheme would not have produced a valid URL.")]
    CannotSetScheme,
    /// The provided port is not a number.
    #[error("The provided port is not a number.")]
    InvalidPort,
    /// Cannot set port for this URL. Either because it is cannot-be-a-base, does not have a host, or has the file scheme.
    #[error("Cannot set port for this URL. Either because it is cannot-be-a-base, does not have a host, or has the file scheme.")]
    CannotSetPort,
    /// Cannot set username for this URL. Either because it is cannot-be-a-base or does not have a host.
    #[error("Cannot set username for this URL. Either because it is cannot-be-a-base or does not have a host.")]
    CannotSetUsername,
    /// Cannot set password for this URL. Either because it is cannot-be-a-base or does not have a host.
    #[error("Cannot set password for this URL. Either because it is cannot-be-a-base or does not have a host.")]
    CannotSetPassword,
    /// Returned by `UrlPart::Subdomain.get` when `UrlPart::Domain.get` returns `None`.
    #[error("The URL's host is not a domain.")]
    HostIsNotADomain,
    /// Urls that are cannot-be-a-base don't have a path.
    #[error("Urls that are cannot-be-a-base don't have a path.")]
    UrlDoesNotHavePath,
    /// Returned when setting a [`UrlPart::DomainSegment`], [`UrlPart::PathSegment`], or [`UrlPart::BeforePathSegment`] when the index isn't in the relevant part's segments.
    #[error("The requested segment was not found")]
    SegmentNotFound,
    /// The URL must have a path as it is not cannot-be-a-base.
    #[error("The URL must have a path as it is not cannot-be-a-base.")]
    UrlMustHavePath,
    /// The URL cannot have a path as it is not cannot-be-a-base.
    #[error("The URL cannot have a path as it is not cannot-be-a-base.")]
    UrlCannotHavePath
}

/// The enum of all possible errors that can occur when applying a [`super::StringModification`] to a [`UrlPart`] using [`UrlPart::modify`].
#[derive(Debug, Error)]
pub enum PartModificationError {
    /// The error returned when the call to [`UrlPart::get`] returns `None` and `none_to_empty_string` is `false`
    #[error("Cannot modify the part's string because it doesn't have a string.")]
    PartCannotBeNone,
    /// The error returned when the call to [`super::StringModification::apply`] fails.
    #[error(transparent)]
    StringError(#[from] super::StringError),
    /// The error returned when the call to [`UrlPart::set`] fails.
    #[error(transparent)]
    PartError(#[from] PartError)
}

#[cfg(test)]
mod tests {
    use super::*;

    const URLS: [&str; 1] = [
        "https://example.com"
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