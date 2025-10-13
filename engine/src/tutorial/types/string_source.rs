//! # [`StringSource`]
//!
//! When setting a URL part, you sometimes want to choose the value based on another part of the URL.
//!
//! Almost all components that take string inputs actually take string sources. This is usually invisible because JSON string literals deserialize to [`StringSource::String`].
//!
//! For example, deviantart used to put the username in the subdomain (`https://username.deviantart.com`) but now puts them in the first path segment (`https://deviantart.com/username`).
//!
//! To get the subdomain, [`StringSource::Part`] is used with [`UrlPart::Subdomain`]. Written in JSON as `{"Part": "Subdomain"}`.
//!
//! To then remove the subdomain, [`Action::SetSubdomain`] is given [`StringSource::None`], written in JSON as `null`.
//!
//! ```Json
//! {"If": {
//!   "if": {"All": [
//!     {"RegDomainIs": "deviantart.com"},
//!     {"Not": {"SubdomainIsOneOf": [null, "www"]}}
//!   ]},
//!   "then": {"All": [
//!     {"InsertPathSegmentAt": {
//!       "index": 0,
//!       "value": {"Part": "Subdomain"}
//!     }},
//!     {"SetSubdomain": null}
//!   ]}
//! }}
//! ```
//!
//! ## Complex construction
//!
//! String sources can be very complex. A common example of this is [`StringSource::Modified`].
//!
//! For example, some redirect URLs encode the redirect in Base64 in a path segment. To clean these, you must first get that path segment then Base64 decode it (which, in URL Cleaner Engine, defaults to the URL safe alphabet).
//!
//! ```Json
//! {"Modified": {
//!   "value": {"Part": {"PathSegment": 1}},
//!   "modification": "Base64Decode"
//! }}
//! ```
//!
//! This might seem slightly inside out (it's `x.map(f)` instead of `f(x)`) but it was the easiest way to design this.
//!
//! For more information on what [`StringSource::Modified`] can do, see the tutorial on [string modifications](string_modification).
//!
//! Additionally, there's also [`StringSource::HttpRequest`] to do HTTP requests, [`StringSource::IfFlag`], [`StringSource::Map`], [`StringSource::ParamsMap`], [`StringSource::NamedPartitioning`], and so on for control flow, [`StringSource::Join`] for joining several other string sources, and so on.

pub(crate) use super::*;
