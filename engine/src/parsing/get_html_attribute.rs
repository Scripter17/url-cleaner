//! [`get_html_attribute`] and co.

use std::sync::LazyLock;

use regex::Regex;

use crate::prelude::*;

// TODO: Turn into one regex.

/** Regex to get the tag.   **/ static GET_TAG  : LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<[a-zA-Z]+\s*((?:([a-zA-Z-]+)(?:\s*=\s*(?:"(.*?)"|'(.*?)'|(\S*)))?\s*)*)/?>"#).expect("???"));
/** Regex to get the attrs. **/ static GET_ATTRS: LazyLock<Regex> = LazyLock::new(|| Regex::new(                 r#"([a-zA-Z-]+)(?:\s*=\s*(?:"(.*?)"|'(.*?)'|(\S*)))?\s*"#     ).expect("???"));

/// Get the value of the last attribute named `name`.
/// # Errors
/// If `value` does not begin with a valid HTML opening tag, returns the error [`SyntaxError`].
///
/// If the call to [`unescape_html`] returns an error, that error is returned.
/// # Examples
/// ```
/// use url_cleaner_engine::prelude::*;
///
/// assert_eq!(get_html_attribute("<a href='aaa'>"                     , "href").unwrap(), Some(Some("aaa" .into())));
/// assert_eq!(get_html_attribute("<a href='a&quot;a'>"                , "href").unwrap(), Some(Some("a\"a".into())));
/// assert_eq!(get_html_attribute("<a href=\"aaa\">"                   , "href").unwrap(), Some(Some("aaa" .into())));
/// assert_eq!(get_html_attribute("<a href=\"a&quot;a\">"              , "href").unwrap(), Some(Some("a\"a".into())));
/// assert_eq!(get_html_attribute("<a href=aaa>"                       , "href").unwrap(), Some(Some("aaa" .into())));
/// assert_eq!(get_html_attribute("<a href=a&quot;a>"                  , "href").unwrap(), Some(Some("a\"a".into())));
///
/// assert_eq!(get_html_attribute("<a href='aaa'        >"             , "href").unwrap(), Some(Some("aaa" .into())));
/// assert_eq!(get_html_attribute("<a href='a&quot;a'   >"             , "href").unwrap(), Some(Some("a\"a".into())));
/// assert_eq!(get_html_attribute("<a href=\"aaa\"      >"             , "href").unwrap(), Some(Some("aaa" .into())));
/// assert_eq!(get_html_attribute("<a href=\"a&quot;a\" >"             , "href").unwrap(), Some(Some("a\"a".into())));
/// assert_eq!(get_html_attribute("<a href=aaa          >"             , "href").unwrap(), Some(Some("aaa" .into())));
/// assert_eq!(get_html_attribute("<a href=a&quot;a     >"             , "href").unwrap(), Some(Some("a\"a".into())));
///
/// assert_eq!(get_html_attribute("<a href=b href='aaa'        >"      , "href").unwrap(), Some(Some("aaa" .into())));
/// assert_eq!(get_html_attribute("<a href=b href='a&quot;a'   >"      , "href").unwrap(), Some(Some("a\"a".into())));
/// assert_eq!(get_html_attribute("<a href=b href=\"aaa\"      >"      , "href").unwrap(), Some(Some("aaa" .into())));
/// assert_eq!(get_html_attribute("<a href=b href=\"a&quot;a\" >"      , "href").unwrap(), Some(Some("a\"a".into())));
/// assert_eq!(get_html_attribute("<a href=b href=aaa          >"      , "href").unwrap(), Some(Some("aaa" .into())));
/// assert_eq!(get_html_attribute("<a href=b href=a&quot;a     >"      , "href").unwrap(), Some(Some("a\"a".into())));
///
/// assert_eq!(get_html_attribute("<a>"                                , "href").unwrap(), None);
///
/// assert_eq!(get_html_attribute("<a href>"                           , "href").unwrap(), Some(None));
///
/// assert_eq!(get_html_attribute("<a href href=\"1\">"                , "href").unwrap(), Some(Some("1".into())));
/// assert_eq!(get_html_attribute("<a href href=\"1\" href>"           , "href").unwrap(), Some(None));
/// assert_eq!(get_html_attribute("<a href href=\"1\" href href=\"2\">", "href").unwrap(), Some(Some("2".into())));
///
/// assert_eq!(get_html_attribute("<a href=\"1\" href>"                , "href").unwrap(), Some(None));
/// assert_eq!(get_html_attribute("<a href=\"1\" href href=\"2\">"     , "href").unwrap(), Some(Some("2".into())));
/// assert_eq!(get_html_attribute("<a href=\"1\" href href=\"2\" href>", "href").unwrap(), Some(None));
///
/// assert_eq!(get_html_attribute("<a href=\"1\">stuff"                , "href").unwrap(), Some(Some("1".into())));
/// assert_eq!(get_html_attribute("<a href=\"1\"><a href=\"2\">"       , "href").unwrap(), Some(Some("1".into())));
/// assert_eq!(get_html_attribute("<a href=\"1\">stuff<a href=\"2\">"  , "href").unwrap(), Some(Some("1".into())));
/// assert_eq!(get_html_attribute("<a href=\"1\">href=\"2\""           , "href").unwrap(), Some(Some("1".into())));
/// ```
#[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
pub fn get_html_attribute<'a>(value: &'a str, name: &str) -> Result<Option<Option<Cow<'a, str>>>, GetHtmlAttributeError> {
    let mut ret = None;

    let attrs = GET_TAG.captures(value).ok_or(SyntaxError)?.get(1).expect("???").as_str();

    for attr in GET_ATTRS.captures_iter(attrs) {
        let attr_name = attr.get(1).expect("???").as_str();

        if attr_name == name {
            ret = Some(attr.get(2).or(attr.get(3)).or(attr.get(4)).map(|x| x.as_str()));
        }
    }

    Ok(match ret {
        Some(Some(ret)) => Some(Some(unescape_html(ret)?)),
        Some(None     ) => Some(None),
        None            => None,
    })
}
