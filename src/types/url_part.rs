use std::borrow::Cow;

use url::{Url, ParseError};
use thiserror::Error;

use serde::{Serialize, Deserialize};

/// An enum that makes using the various [`Url`] getters simpler.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum UrlPart {
    /// The whole URL. Corresponds to [`Url::as_str`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Whole.get(&Url::parse("https://example.com").unwrap()), Some(Cow::Borrowed("https://example.com/")));
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
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Scheme.get(&Url::parse("https://example.com").unwrap()), Some(Cow::Borrowed("https")));
    /// assert_eq!(UrlPart::Scheme.get(&Url::parse("http://example.com" ).unwrap()), Some(Cow::Borrowed("http" )));
    /// assert_eq!(UrlPart::Scheme.get(&Url::parse("ftp://example.com"  ).unwrap()), Some(Cow::Borrowed("ftp"  )));
    /// 
    /// let mut url=Url::parse("https://example.com").unwrap();
    /// assert!(UrlPart::Scheme.set(&mut url, Some("http")).is_ok());
    /// assert_eq!(url.scheme(), "http");
    /// assert!(UrlPart::Scheme.set(&mut url, None).is_err());
    /// ```
    Scheme,
    /// The username. Corresponds to [`Url::username`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Username.get(&Url::parse("https://user:pass@example.com").unwrap()), Some(Cow::Borrowed("user")));
    /// assert_eq!(UrlPart::Username.get(&Url::parse("http://user:pass@example.com" ).unwrap()), Some(Cow::Borrowed("user")));
    /// assert_eq!(UrlPart::Username.get(&Url::parse("ftp://user:pass@example.com"  ).unwrap()), Some(Cow::Borrowed("user")));
    /// assert_eq!(UrlPart::Username.get(&Url::parse("https://example.com").unwrap()), Some(Cow::Borrowed("")));
    /// assert_eq!(UrlPart::Username.get(&Url::parse("http://example.com" ).unwrap()), Some(Cow::Borrowed("")));
    /// assert_eq!(UrlPart::Username.get(&Url::parse("ftp://example.com"  ).unwrap()), Some(Cow::Borrowed("")));
    /// 
    /// let mut url=Url::parse("https://example.com").unwrap();
    /// assert!(UrlPart::Username.set(&mut url, Some("test")).is_ok());
    /// assert_eq!(url.username(), "test");
    /// assert!(UrlPart::Username.set(&mut url, None).is_err());
    /// ```
    Username,
    /// The password. Corresponds to [`Url::password`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Password.get(&Url::parse("https://user:pass@example.com").unwrap()), Some(Cow::Borrowed("pass")));
    /// assert_eq!(UrlPart::Password.get(&Url::parse("http://user:pass@example.com" ).unwrap()), Some(Cow::Borrowed("pass")));
    /// assert_eq!(UrlPart::Password.get(&Url::parse("ftp://user:pass@example.com"  ).unwrap()), Some(Cow::Borrowed("pass")));
    /// assert_eq!(UrlPart::Password.get(&Url::parse("https://example.com").unwrap()), None);
    /// assert_eq!(UrlPart::Password.get(&Url::parse("http://example.com" ).unwrap()), None);
    /// assert_eq!(UrlPart::Password.get(&Url::parse("ftp://example.com"  ).unwrap()), None);
    /// ```
    Password,
    /// The host. Either a domain name or IPV4/6 address. Corresponds to [`Url::host`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Host.get(&Url::parse("https://127.0.0.1"      ).unwrap()), Some(Cow::Borrowed("127.0.0.1"      )));
    /// assert_eq!(UrlPart::Host.get(&Url::parse("https://www.example.com").unwrap()), Some(Cow::Borrowed("www.example.com")));
    /// assert_eq!(UrlPart::Host.get(&Url::parse("https://a.b.example.com").unwrap()), Some(Cow::Borrowed("a.b.example.com")));
    /// assert_eq!(UrlPart::Host.get(&Url::parse("https://example.com"    ).unwrap()), Some(Cow::Borrowed("example.com"    )));
    /// ```
    Host,
    /// The subdomain. If the domain is `a.b.c.co.uk`, the value returned/changed by this is `a.b`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Subdomain.get(&Url::parse("https://127.0.0.1"      ).unwrap()), None);
    /// assert_eq!(UrlPart::Subdomain.get(&Url::parse("https://www.example.com").unwrap()), Some(Cow::Borrowed("www")));
    /// assert_eq!(UrlPart::Subdomain.get(&Url::parse("https://a.b.example.com").unwrap()), Some(Cow::Borrowed("a.b")));
    /// assert_eq!(UrlPart::Subdomain.get(&Url::parse("https://example.com"    ).unwrap()), Some(Cow::Borrowed("")));
    /// ```
    Subdomain,
    /// The domain minus the subdomain. If the domain is `a.b.c.co.uk` value returned/changed by this is `c.co.uk`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::NotSubdomain.get(&Url::parse("https://127.0.0.1"      ).unwrap()), None);
    /// assert_eq!(UrlPart::NotSubdomain.get(&Url::parse("https://www.example.com").unwrap()), Some(Cow::Borrowed("example.com")));
    /// assert_eq!(UrlPart::NotSubdomain.get(&Url::parse("https://a.b.example.com").unwrap()), Some(Cow::Borrowed("example.com")));
    /// assert_eq!(UrlPart::NotSubdomain.get(&Url::parse("https://example.com"    ).unwrap()), Some(Cow::Borrowed("example.com")));
    /// ```
    NotSubdomain,
    /// The domain. Corresponds to [`Url::domain`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Domain.get(&Url::parse("https://127.0.0.1"      ).unwrap()), None);
    /// assert_eq!(UrlPart::Domain.get(&Url::parse("https://www.example.com").unwrap()), Some(Cow::Borrowed("www.example.com")));
    /// assert_eq!(UrlPart::Domain.get(&Url::parse("https://a.b.example.com").unwrap()), Some(Cow::Borrowed("a.b.example.com")));
    /// assert_eq!(UrlPart::Domain.get(&Url::parse("https://example.com"    ).unwrap()), Some(Cow::Borrowed("example.com")));
    /// ```
    Domain,
    /// The port as a string. Corresponds to [`Url::port_or_known_default`].
    /// Ports are strings for the sake of a simpler API.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Port.get(&Url::parse("https://example.com"    ).unwrap()), Some(Cow::Owned("443".to_string())));
    /// assert_eq!(UrlPart::Port.get(&Url::parse("https://example.com:443").unwrap()), Some(Cow::Owned("443".to_string())));
    /// assert_eq!(UrlPart::Port.get(&Url::parse("https://example.com:80" ).unwrap()), Some(Cow::Owned("80" .to_string())));
    /// ```
    Port,
    /// A specficic segment of the URL's path.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::PathSegment(0).get(&Url::parse("https://example.com"     ).unwrap()), Some(Cow::Borrowed("")));
    /// assert_eq!(UrlPart::PathSegment(0).get(&Url::parse("https://example.com/a"   ).unwrap()), Some(Cow::Borrowed("a")));
    /// assert_eq!(UrlPart::PathSegment(1).get(&Url::parse("https://example.com/a"   ).unwrap()), None);
    /// assert_eq!(UrlPart::PathSegment(1).get(&Url::parse("https://example.com/a/"  ).unwrap()), Some(Cow::Borrowed("")));
    /// assert_eq!(UrlPart::PathSegment(1).get(&Url::parse("https://example.com/a/b" ).unwrap()), Some(Cow::Borrowed("b")));
    /// 
    /// let mut url=Url::parse("https://example.com/a/b/c/d").unwrap();
    /// assert!(UrlPart::PathSegment(1).set(&mut url, Some("e")).is_ok());
    /// assert_eq!(url.path(), "/a/e/c/d");
    /// assert!(UrlPart::PathSegment(1).set(&mut url, None).is_ok());
    /// assert_eq!(url.path(), "/a/c/d");
    /// ```
    PathSegment(usize),
    /// Useful only for appending a path segment to a URL as the getter is always `None`.
    /// Using this with a URL whose path ends in an empty segment (`https://example.com/a/b/`), the setter will overwrite that segment instead of leaving a random empty segment in the middle of the path.
    /// Why is path manipulation always a pain?
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::NextPathSegment.get(&Url::parse("https://example.com"   ).unwrap()), None);
    /// assert_eq!(UrlPart::NextPathSegment.get(&Url::parse("https://example.com/"  ).unwrap()), None);
    /// assert_eq!(UrlPart::NextPathSegment.get(&Url::parse("https://example.com/a" ).unwrap()), None);
    /// assert_eq!(UrlPart::NextPathSegment.get(&Url::parse("https://example.com/a/").unwrap()), None);
    /// 
    /// let mut url=Url::parse("https://example.com").unwrap();
    /// assert!(UrlPart::NextPathSegment.set(&mut url, Some("a")).is_ok());
    /// assert_eq!(url.path(), "/a");
    /// assert!(UrlPart::NextPathSegment.set(&mut url, Some("b")).is_ok());
    /// assert_eq!(url.path(), "/a/b");
    /// assert!(UrlPart::NextPathSegment.set(&mut url, Some("")).is_ok());
    /// assert_eq!(url.path(), "/a/b/");
    /// assert!(UrlPart::NextPathSegment.set(&mut url, Some("")).is_ok());
    /// assert_eq!(url.path(), "/a/b/");
    /// assert!(UrlPart::NextPathSegment.set(&mut url, Some("c")).is_ok());
    /// assert_eq!(url.path(), "/a/b/c");
    /// assert!(UrlPart::NextPathSegment.set(&mut url, None).is_ok());
    /// assert_eq!(url.path(), "/a/b/c");
    /// ```
    NextPathSegment,
    /// The path. Corresponds to [`Url::path`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Path.get(&Url::parse("https://example.com"     ).unwrap()), Some(Cow::Borrowed("/"   )));
    /// assert_eq!(UrlPart::Path.get(&Url::parse("https://example.com/a"   ).unwrap()), Some(Cow::Borrowed("/a"  )));
    /// assert_eq!(UrlPart::Path.get(&Url::parse("https://example.com/a"   ).unwrap()), Some(Cow::Borrowed("/a"  )));
    /// assert_eq!(UrlPart::Path.get(&Url::parse("https://example.com/a/"  ).unwrap()), Some(Cow::Borrowed("/a/" )));
    /// assert_eq!(UrlPart::Path.get(&Url::parse("https://example.com/a/b" ).unwrap()), Some(Cow::Borrowed("/a/b")));
    /// ```
    Path,
    /// A specific query paramater. The contained string is the paramater's name and the setter sets the paramater's value.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::QueryParam("a".to_string()).get(&Url::parse("https://example.com?a=2&b=3").unwrap()), Some(Cow::Borrowed("2")));
    /// assert_eq!(UrlPart::QueryParam("c".to_string()).get(&Url::parse("https://example.com?a=2&b=3").unwrap()), None);
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
    /// ```
    QueryParam(String),
    /// The query. Corresponds to [`Url::query`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Query.get(&Url::parse("https://example.com"        ).unwrap()), None);
    /// assert_eq!(UrlPart::Query.get(&Url::parse("https://example.com?a=2&b=3").unwrap()), Some(Cow::Borrowed("a=2&b=3")));
    /// 
    /// let mut url=Url::parse("https://example.com?a=2&b=3").unwrap();
    /// assert!(UrlPart::Query.set(&mut url, Some("c=4")).is_ok());
    /// assert_eq!(url.query(), Some("c=4"));
    /// assert!(UrlPart::Query.set(&mut url, None).is_ok());
    /// assert_eq!(url.query(), None);
    /// ```
    Query,
    /// The fragment. Corresponds to [`Url::fragment`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::UrlPart;
    /// # use std::borrow::Cow;
    /// assert_eq!(UrlPart::Fragment.get(&Url::parse("https://example.com"  ).unwrap()), None);
    /// assert_eq!(UrlPart::Fragment.get(&Url::parse("https://example.com#a").unwrap()), Some(Cow::Borrowed("a")));
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
    pub fn get<'a>(&self, url: &'a Url) -> Option<Cow<'a, str>> {
        Some(match self {
            // Ordered hopefully most used to least used.
            Self::Query            => Cow::Borrowed(url.query()?),
            Self::Whole            => Cow::Borrowed(url.as_str()),
            Self::Host             => Cow::Borrowed(url.host_str()?),
            Self::Subdomain        => Cow::Borrowed({
                let domain=url.domain()?;
                // `psl::suffix_str` should never return `None`. Testing required.
                domain.strip_suffix(psl::suffix_str(domain)?)?.strip_suffix('.').unwrap_or("").rsplit_once('.').unwrap_or(("", "")).0
            }),
            Self::NotSubdomain     => Cow::Borrowed({
                let temp=url.domain()?.strip_prefix(&*Self::Subdomain.get(url)?)?;
                temp.strip_prefix('.').unwrap_or(temp)
            }),
            Self::Domain           => Cow::Borrowed(url.domain()?),
            Self::Port             => Cow::Owned   (url.port_or_known_default()?.to_string()), // I cannot be bothered to add number handling.
            Self::PathSegment(n)   => Cow::Borrowed(url.path_segments()?.nth(*n)?), // If the path doesn't start with `'/'` then it's
            Self::Path             => Cow::Borrowed(url.path()),
            Self::QueryParam(name) => url.query_pairs().find_map(|(name2, value)| (name==&name2).then_some(value))?,

            // The things that are likely very rarely used.

            Self::NextPathSegment  => None?,
            Self::Scheme           => Cow::Borrowed(url.scheme()),
            Self::Username         => Cow::Borrowed(url.username()),
            Self::Password         => Cow::Borrowed(url.password()?),
            Self::Fragment         => Cow::Borrowed(url.fragment()?)
        })
    }

    /// Replaces the specified part of the provided URL with the provided value
    /// # Errors
    /// If `with` is `None`, the following part setters will return the error [`ReplaceError::PartCannotBeNone`]:
    /// [`UrlPart::Whole`], [`UrlPart::Scheme`], [`UrlPart::Username`], 
    pub fn set(&self, url: &mut Url, to: Option<&str>) -> Result<(), ReplaceError> {
        match (self, to) {
            // Ordered hopefully most used to least used.
            (Self::Query    , _       ) => url.set_query  (to),
            (Self::Whole    , Some(to)) => *url=Url::parse(to)?,
            (Self::Host     , _       ) => url.set_host   (to)?,
            (Self::Subdomain, Some(to)) => {
                let mut new_domain=to.to_string();
                new_domain.push('.');
                new_domain.push_str(&Self::NotSubdomain.get(url).ok_or(ReplaceError::HostIsNotADomain)?);
                url.set_host(Some(&new_domain))?;
            },
            (Self::NotSubdomain, Some(to))     => {
                let mut new_domain=Self::Subdomain.get(url).ok_or(ReplaceError::HostIsNotADomain)?.to_string();
                new_domain.push('.');
                new_domain.push_str(to);
                url.set_host(Some(&new_domain))?;
            },
            (Self::Domain        , _) => url.set_host(to)?,
            (Self::Port          , _) => url.set_port(to.map(|x| x.parse().map_err(|_| ReplaceError::InvalidPort)).transpose()?).map_err(|_| ReplaceError::CannotSetPort)?,
            (Self::PathSegment(n), _) => match to {
                // Apparently `Iterator::intersperse` was stabilized but had issues with itertools. Very annoying.
                Some(to) => url.set_path(&url.path_segments().ok_or(ReplaceError::CannotBeABase)?.enumerate().       map(|(i, x)| if i==*n {to} else {x}).collect::<Vec<_>>().join("/")),
                None     => url.set_path(&url.path_segments().ok_or(ReplaceError::CannotBeABase)?.enumerate().filter_map(|(i, x)|   (i!=*n).then_some(x)).collect::<Vec<_>>().join("/")),
            },
            (Self::NextPathSegment, _)  => {
                if let Some(to) = to {
                    url.path_segments_mut().map_err(|_| ReplaceError::CannotBeABase)?.pop_if_empty().push(to);
                }
            },
            (Self::Path, Some(to)) => url.set_path(to),
            (Self::QueryParam(name), _) => {
                match to {
                    Some(to) => {
                        if url.query().is_some() {
                            url.set_query(Some(&if url.query_pairs().any(|(name2, _)| name==&name2) {
                                form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().map(|(name2, value)| if name==&name2 {(name2, Cow::Borrowed(to))} else {(name2, value)})).finish()
                            } else {
                                form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().chain([(Cow::Borrowed(name.as_str()), Cow::Borrowed(to))])).finish()
                            }))
                        }
                    },
                    None => {
                        let new_query=form_urlencoded::Serializer::new(String::new()).extend_pairs(url.query_pairs().filter(|(name2, _)| name!=name2)).finish();
                        url.set_query((!new_query.is_empty()).then_some(&new_query))
                    }
                }
            }

            // The things that are likely very rarely used.

            (Self::Scheme   , Some(to)) => url.set_scheme  (to).map_err(|_| ReplaceError::InvalidScheme)?,
            (Self::Username , Some(to)) => url.set_username(to).map_err(|_| ReplaceError::CannotSetUsername)?,
            (Self::Password , _       ) => url.set_password(to).map_err(|_| ReplaceError::CannotSetPassword)?,
            (Self::Fragment, _) => url.set_fragment(to),
            (_, None) => Err(ReplaceError::PartCannotBeNone)?
        }
        Ok(())
    }

    /// Get the part from the provided URL and modify it according to the provided string modification rule.
    pub fn modify(&self, url: &mut Url, none_to_empty_string: bool, how: &super::StringModification) -> Result<(), PartModificationError> {
        let mut new_part=self.get(url).ok_or(PartModificationError::PartCannotBeNone).or(if none_to_empty_string {Ok(Cow::Borrowed(""))} else {Err(PartModificationError::PartCannotBeNone)})?.into_owned();
        how.apply(&mut new_part)?;
        self.set(url, Some(&new_part))?;
        Ok(())
    }
}

/// An enum of all possible errors [`UrlPart::set`] can return.
#[derive(Debug, Error)]
pub enum ReplaceError {
    /// Attempted replacement would not produce a valid URL.
    #[error(transparent)]
    ParseError(#[from] ParseError),
    /// [`UrlPart::set`] attempted to set a part that cannot be None to None.
    #[error("UrlPart::set attempted to set a part that cannot be None to None.")]
    PartCannotBeNone,
    /// The provided scheme would not have produced a valid URL.
    #[error("The provided scheme would not have produced a valid URL.")]
    InvalidScheme,
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
    /// Returned by `UrlPart::PathSegment.set` when the URL is `cannot-be-a-base` because [`Url::path_segments`] returns `None` in that case.
    #[error("The URL cannot be a base and for whatever reason that means `Url::path_segments` returned `None`")]
    CannotBeABase
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
    ReplaceError(#[from] ReplaceError)
}
