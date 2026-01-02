//! Implementing query stuff for [`BetterUrl`].

use std::borrow::Cow;

use form_urlencoded::Serializer;
use thiserror::Error;
use url::UrlQuery;
#[expect(unused_imports, reason = "Used in doc comments.")]
use url::Url;

use crate::prelude::*;

/// The enum of errors [`BetterUrl::set_query_param`] can return.
#[derive(Debug, Error)]
pub enum SetQueryParamError {
    /// Returned when a query parameter with the specified index can't be set/created.
    #[error("A query parameter with the specified index could not be set/created.")]
    QueryParamIndexNotFound
}

/// The enum of errors [`BetterUrl::set_query_segment`] and [`BetterUrl::set_raw_query_segment`] can return.
#[derive(Debug, Error)]
pub enum SetQuerySegmentError {
    /// Returned when there is no query.
    #[error("There is no query.")]
    NoQuery,
    /// Returned when the query segment isn't found.
    #[error("The query segment wasn't found.")]
    QuerySegmentNotFound
}

/// The enum of errors [`BetterUrl::insert_query_segment`] and [`BetterUrl::insert_raw_query_segment`] can return.
#[derive(Debug, Error)]
pub enum InsertQuerySegmentError {
    /// Returned when there is no query.
    #[error("There is no query.")]
    NoQuery,
    /// Returned when the query segment isn't found.
    #[error("The query segment wasn't found.")]
    QuerySegmentNotFound
}

/// The enum of errors [`BetterUrl::rename_query_param`] can return.
#[derive(Debug, Error)]
pub enum RenameQueryParamError {
    /// Returned when attempting to rename a query param to a name containing a `&`, `=`, or `#`.
    #[error("Attempted to rename a query param to a name containing a &, =, or #.")]
    InvalidName,
    /// Returned when attempting to rename a query param in a URL with no query.
    #[error("Attempted to rename a query param in a URL with no query.")]
    NoQuery,
    /// Returned when the specified query param isn't found.
    #[error("The specified query param was not found.")]
    QueryParamNotFound
}

impl BetterUrl {
    /// [`Url::set_query`].
    pub fn set_query(&mut self, query: Option<&str>) {
        if self.query() != query {
            self.url.set_query(query)
        }
    }

    /// [`Url::query_pairs_mut`].
    pub fn query_pairs_mut(&mut self) -> Serializer<'_, UrlQuery<'_>> {
        self.url.query_pairs_mut()
    }

    /// An iterator over query parameters without percent decoding anything.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
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
    pub fn raw_query_pairs(&self) -> Option<impl Iterator<Item = (&str, Option<&str>)>> {
        Some(self.query()?
            .split('&')
            .map(|kev| kev.split_once('=')
                .map_or(
                    (kev, None),
                    |(k, v)| (k, Some(v))
                )
            ))
    }

    /// Get the selected query parameter without percent decoding the value.
    ///
    /// For matching, the names are percent decoded. So a `%61=a` query parameter is selectable with a `name` of `a`.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let url = BetterUrl::parse("https://example.com?a=1&%61=2&a=3&b=%41&%62=%42&b=%43").unwrap();
    ///
    /// assert_eq!(url.raw_query_param("a", 0), Some(Some(Some("1"))));
    /// assert_eq!(url.raw_query_param("a", 1), Some(Some(Some("2"))));
    /// assert_eq!(url.raw_query_param("a", 2), Some(Some(Some("3"))));
    /// assert_eq!(url.raw_query_param("b", 0), Some(Some(Some("%41"))));
    /// assert_eq!(url.raw_query_param("b", 1), Some(Some(Some("%42"))));
    /// assert_eq!(url.raw_query_param("b", 2), Some(Some(Some("%43"))));
    /// ```
    pub fn raw_query_param<'a>(&'a self, name: &str, index: usize) -> Option<Option<Option<&'a str>>> {
        self.raw_query_pairs().map(|pairs| pairs.filter(|(x, _)| decode_query_part(x) == name).nth(index).map(|(_, v)| v))
    }

    /// Return [`true`] if [`Self::raw_query_param`] would return `Some(Some(_))`, but doesn't do any unnecessary computation.
    ///
    /// Please note that this returns [`true`] even if the query param has no value.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let url = BetterUrl::parse("https://example.com?a=1&%61=2&a&%61=4").unwrap();
    ///
    /// assert!( url.has_raw_query_param("a", 0));
    /// assert!( url.has_raw_query_param("a", 1));
    /// assert!(!url.has_raw_query_param("a", 2));
    /// assert!(!url.has_raw_query_param("a", 3));
    /// assert!(!url.has_raw_query_param("a", 4));
    /// ```
    pub fn has_raw_query_param(&self, name: &str, index: usize) -> bool {
        self.raw_query_pairs().is_some_and(|pairs| pairs.filter(|(key, _)| *key == name).nth(index).is_some())
    }

    /// Return [`true`] if [`Self::query_param`] would return `Some(Some(_))`, but doesn't do any unnecessary computation.
    ///
    /// For matching, the names are percent decoded. So a `%61=a` query parameter is selectable with a `name` of `a`.
    ///
    /// Please note that this returns [`true`] even if the query param has no value.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
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
        self.raw_query_pairs().is_some_and(|pairs| pairs.filter(|(key, _)| decode_query_part(key) == name).nth(index).is_some())
    }

    /// Get the selected query parameter.
    ///
    /// For matching, the names are percent decoded. So a `%61=a` query parameter is selectable with a `name` of `a`.
    ///
    /// First [`Option`] is if there's a query.
    ///
    /// Second [`Option`] is if there's a query parameter with the specified name.
    ///
    /// Third [`Option`] is if it has a value.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let url = BetterUrl::parse("https://example.com?a=2&b=3&a=4&c").unwrap();
    ///
    /// assert_eq!(url.query_param("a", 0), Some(Some(Some("2".into()))));
    /// assert_eq!(url.query_param("a", 1), Some(Some(Some("4".into()))));
    /// assert_eq!(url.query_param("a", 2), Some(None));
    /// assert_eq!(url.query_param("b", 0), Some(Some(Some("3".into()))));
    /// assert_eq!(url.query_param("b", 1), Some(None));
    /// assert_eq!(url.query_param("c", 0), Some(Some(None)));
    /// assert_eq!(url.query_param("c", 1), Some(None));
    ///
    ///
    /// let url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(url.query_param("a", 0), None);
    /// assert_eq!(url.query_param("a", 1), None);
    ///
    ///
    /// let url = BetterUrl::parse("https://example.com?a=1&%61=2&a=3&b=%41&%62=%42&b=%43").unwrap();
    ///
    /// assert_eq!(url.query_param("a", 0), Some(Some(Some("1".into()))));
    /// assert_eq!(url.query_param("a", 1), Some(Some(Some("2".into()))));
    /// assert_eq!(url.query_param("a", 2), Some(Some(Some("3".into()))));
    /// assert_eq!(url.query_param("b", 0), Some(Some(Some("A".into()))));
    /// assert_eq!(url.query_param("b", 1), Some(Some(Some("B".into()))));
    /// assert_eq!(url.query_param("b", 2), Some(Some(Some("C".into()))));
    ///
    ///
    /// let url = BetterUrl::parse("https://example.com?a+b=2+3").unwrap();
    ///
    /// assert_eq!(url.query_param("a b", 0), Some(Some(Some("2 3".into()))));
    /// ```
    pub fn query_param<'a>(&'a self, name: &str, index: usize) -> Option<Option<Option<Cow<'a, str>>>> {
        self.raw_query_param(name, index).map(|v| v.map(|v| v.map(|v| decode_query_part(v))))
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
    /// use better_url::prelude::*;
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
    /// Useful in combination with [`Self::raw_query_param`] for transplanting values without decoding then re-encoding them.
    ///
    /// PLEASE note that if `name` and/or `value` contain special characters like `=`, `&`, etc. this will give incoherent results! ONLY use this for directly transplanting from and to query params.
    /// # Errors
    /// If `index` is above the number of matched query params, returns the error [`SetQueryParamError::QueryParamIndexNotFound`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
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
    /// // Failing to set the fragment because [`Url::set_query`] refuses to.
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
        let mut ret = String::with_capacity(self.query().map_or(0, str::len) + name.len() + 1 + to.flatten().map_or(0, str::len));
        let mut found = 0;

        if let Some(query) = self.query() {
            for param in query.split('&') {
                if param.strip_prefix(name).is_some_and(|x| x.is_empty() || x.starts_with('=')) {
                    if found == index {
                        if let Some(x) = to {
                            if !ret.is_empty() {ret.push('&');}
                            ret.push_str(name);
                            if let Some(x) = x {
                                ret.push('=');
                                ret.push_str(x);
                            }
                        }
                    } else {
                        if !ret.is_empty() {ret.push('&');}
                        ret.push_str(param);
                    }
                    found += 1;
                } else {
                    if !ret.is_empty() {ret.push('&');}
                    ret.push_str(param);
                }
            }
        }
        if found == index {
            if let Some(x) = to {
                if !ret.is_empty() {ret.push('&');}
                ret.push_str(name);
                if let Some(x) = x {
                    ret.push('=');
                    ret.push_str(x);
                }
            }
        } else if found < index {
            Err(SetQueryParamError::QueryParamIndexNotFound)?
        }

        self.set_query(Some(&*ret).filter(|x| !x.is_empty()));

        Ok(())
    }

    /// Rename the specified query parameter to the specified name.
    /// # Errors
    /// If `to` contains a `&`, `=`, or `#`, returns the error [`RenameQueryParamError::InvalidName`].
    ///
    /// If [`Url::query`] is [`None`], returns the error [`RenameQueryParamError::NoQuery`].
    ///
    /// If the specified query param isn't found, returns the error [`RenameQueryParamError::QueryParamNotFound`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com?a=1&%61=2&a=3").unwrap();
    ///
    /// url.rename_query_param("a", 1, "b").unwrap();
    /// assert_eq!(url.query(), Some("a=1&b=2&a=3"));
    ///
    /// url.rename_query_param("a", 1, "b").unwrap();
    /// assert_eq!(url.query(), Some("a=1&b=2&b=3"));
    ///
    /// url.rename_query_param("a", 1, "b").unwrap_err();
    /// assert_eq!(url.query(), Some("a=1&b=2&b=3"));
    /// ```
    pub fn rename_query_param(&mut self, name: &str, index: usize, to: &str) -> Result<(), RenameQueryParamError> {
        if to.contains(['&', '=', '#']) {
            Err(RenameQueryParamError::InvalidName)?
        }
        let query = self.query().ok_or(RenameQueryParamError::NoQuery)?;
        let mut new = String::with_capacity(query.len() + to.len());
        let mut found = 0;
        for param in query.split('&') {
            if !new.is_empty() {new.push('&');}
            let (k, v) = match param.split_once('=') {
                Some((k, v)) => (k, Some(v)),
                None => (param, None)
            };
            if decode_query_part(k) == name {
                if found == index {
                    new.push_str(to);
                    if let Some(v) = v {
                        new.push('=');
                        new.push_str(v);
                    }
                } else {
                    new.push_str(param);
                }
                found += 1;
            } else {
                new.push_str(param);
            }
        }
        if found <= index {
            Err(RenameQueryParamError::QueryParamNotFound)?;
        }
        self.set_query(Some(&new));
        Ok(())
    }

    /// Set the specified query segment without percent encoding.
    ///
    /// If `value` is [`Some`], sets the segment to `{key}={value}`.
    ///
    /// If `value` is [`None`], sets the segment to `{key}`.
    /// # Errors
    /// If the call to [`Url::query`] returns [`None`], returns the error [`SetQuerySegmentError::NoQuery`].
    ///
    /// If the index is out of range, returns the error [`SetQuerySegmentError::QuerySegmentNotFound`].
    pub fn set_raw_query_segment(&mut self, index: isize, key: &str, value: Option<&str>) -> Result<(), SetQuerySegmentError> {
        self.set_query(Some(&set_key_value(
            self.query().ok_or(SetQuerySegmentError::NoQuery)?,
            "&",
            index,
            &qpeh(key),
            value.map(qpeh).as_deref(),
            SetQuerySegmentError::QuerySegmentNotFound
        )?));
        Ok(())
    }

    /// Inserts the specified query segment without percent encoding.
    ///
    /// If `value` is [`Some`], sets the segment to `{key}={value}`.
    ///
    /// If `value` is [`None`], sets the segment to `{key}`.
    /// # Errors
    /// If the call to [`Url::query`] returns [`None`], returns the error [`InsertQuerySegmentError::NoQuery`].
    ///
    /// If the index is out of range, returns the error [`InsertQuerySegmentError::QuerySegmentNotFound`].
    pub fn insert_raw_query_segment(&mut self, index: isize, key: &str, value: Option<&str>) -> Result<(), InsertQuerySegmentError> {
        self.set_query(Some(&insert_key_value(
            self.query().ok_or(InsertQuerySegmentError::NoQuery)?,
            "&",
            index,
            key,
            value,
            InsertQuerySegmentError::QuerySegmentNotFound
        )?));
        Ok(())
    }

    /// Set the specified query segment.
    ///
    /// If `value` is [`Some`], sets the segment to `{key}={value}`.
    ///
    /// If `value` is [`None`], sets the segment to `{key}`.
    /// # Errors
    /// If the call to [`Url::query`] returns [`None`], returns the error [`SetQuerySegmentError::NoQuery`].
    ///
    /// If the index is out of range, returns the error [`SetQuerySegmentError::QuerySegmentNotFound`].
    pub fn set_query_segment(&mut self, index: isize, key: &str, value: Option<&str>) -> Result<(), SetQuerySegmentError> {
        self.set_query(Some(&set_key_value(
            self.query().ok_or(SetQuerySegmentError::NoQuery)?,
            "&",
            index,
            key,
            value,
            SetQuerySegmentError::QuerySegmentNotFound
        )?));
        Ok(())
    }

    /// Inserts the specified query segment.
    ///
    /// If `value` is [`Some`], sets the segment to `{key}={value}`.
    ///
    /// If `value` is [`None`], sets the segment to `{key}`.
    /// # Errors
    /// If the call to [`Url::query`] returns [`None`], returns the error [`InsertQuerySegmentError::NoQuery`].
    ///
    /// If the index is out of range, returns the error [`InsertQuerySegmentError::QuerySegmentNotFound`].
    pub fn insert_query_segment(&mut self, index: isize, key: &str, value: Option<&str>) -> Result<(), InsertQuerySegmentError> {
        self.set_query(Some(&insert_key_value(
            self.query().ok_or(InsertQuerySegmentError::NoQuery)?,
            "&",
            index,
            &qpeh(key),
            value.map(qpeh).as_deref(),
            InsertQuerySegmentError::QuerySegmentNotFound
        )?));
        Ok(())
    }
}
