//! # [`Commons`]
//!
//! A "common" is basically a function, with `common_args` containing the arguments to that function.
//!
//! In the following cleaner, the `utm_*` family of query parameters are removed both before expanding bit.ly redirects and before outputting the result.
//!
//! ```Json
//! {
//!   "commons": {
//!     "actions": {
//!       "universal": {"RemoveQueryParams": ["utm_campaign", "utm_content", "utm_id", "utm_medium", "utm_source", "utm_term"]}
//!     }
//!   },
//!   "actions": [
//!     {"If": {
//!       "if": {"NormalizedHostIs": "bit.ly"},
//!       "then": {"All": [
//!         {"Common": "universal"},
//!         "ExpandRedirect"
//!       ]}
//!     }},
//!     {"Common": "universal"}
//!   ]
//! }
//! ```
//!
//! This prevents bit.ly from seeing the tracking parameters while keeping all places that require that in sync.
//!
//! ## Common args
//!
//! The not stupid way for a website to do redirects is to return an HTTP 301 status code with a header saying "go to `https://example.com/whatever`".
//!
//! Some websites instead do stupid things like using the [`meta`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta/http-equiv) HTML element or javascript to do redirects.
//!
//! The default cleaner handles this stupidity by having a common action called `extract_from_page` that
//!
//! 1. Takes a string modification.
//!
//! 2. Gets the body of the webpage.
//!
//! 3. Applies the provided string modification to the body.
//!
//! 4. Replaces the URL with the result.
//!
//! For example, to handle `smarturl.it` redirects, the default cleaner uses the `extract_from_page` common action to search for `"originalUrl":` then extracts the value of the javascript string literal immediately after it.
//!
//! ```Json
//! {
//!   "commons": {
//!     "actions": {
//!       "extract_from_page": {"SetWhole": {"Modified": {
//!         "value": {"HttpRequest": {}},
//!         "modification": {"CommonCallArg": "extractor"}
//!       }}}
//!     }
//!   },
//!   "actions": [
//!     {"If": {
//!       "if": {"NormalizedHostIs": "smarturl.it"},
//!       "then": {"Common": {
//!         "name": "extract_from_page",
//!         "args": {
//!           "string_modifications": {
//!             "extractor": {"All": [
//!               {"KeepAfter": "\"originalUrl\":"},
//!               "GetJsStringLiteralPrefix"
//!             ]}
//!           }
//!         }
//!       }}
//!     }}
//!   ]
//! }
//! ```
//!
//! While here the benefit of using a common is small, the actual code in the default cleaner includes caching, applies the `universal` common action, and accounts for the `no_network` flag, making it much more beneficial.
//!
//! A common can take flags, vars, conditions, actions, string sources, string modifications, and string matchers. These go in the `common_args` section seen in the [debugging](#Debugging) section.
//!
//! Additionally, conditions, actions, string sources, string modifications, and string matchers all have commons that can be invoked in the same way.

pub(crate) use super::*;
