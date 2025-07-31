//! # [`StringSource`]s
//!
//! When setting a URL part, you sometimes want to choose the value based on another part of the URL.
//!
//! For example, deviantart used to put the username in the subdomain (`https://username.deviantart.com`) but now puts them in the first path segment (`https://deviantart.com/username`).
//!
//! While opening `https://username.deviantart.com` takes you to `https://deviantart.com/username`, it tells deviantart you clicked an old URL.
//!
//! To set the first path segment to the subdomain, string sources are used:
//!
//! ```Json
//! {
//!   "actions": [
//!     {"If": {
//!       "if": {"All": [
//!         {"RegDomainIs": "deviantart.com"},
//!         {"Not": {"SubdomainIsOneOf": [null, "www"]}}
//!       ]},
//!       "then": {"All": [
//!         {"InsertPathSegmentAt": {
//!           "index": 0,
//!           "value": {"Part": "Subdomain"}
//!         }},
//!         {"SetSubdomain": null}
//!       ]}
//!     }}
//!   ]
//! }
//! ```
//!
//! Here, the `InsertPathSegmentAt` action's `value` field is a string source that gets the URL's subdomain.
//!
//! Most places that expect a string, except for sets like the `SubdomainIsOneOf` and some other oddball places, actually take a string source.
//!
//! The most basic string source is the string, written as strings. `{"SetSubdomain": "abc"}` sets the subdomain to `abc`.
//!
//! While not obvious from the name, string sources can also be null. `{"SetSubdomain": null}` removes the subdomain. A `{"Part": "Subdomain"}` string source would then act like null.

pub(crate) use super::*;
