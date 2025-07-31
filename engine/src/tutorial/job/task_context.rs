//! # [`TaskContext`]
//!
//! Sometimes you'll want to give a cleaner context about an individual task.
//! This is mainly used along with [job context](job_context) by URL Cleaner Site and URL Cleaner Site Userscript to provide site-specific optimizations.
//!
//! For example, on twitter, links in tweets are changed to point to `https://t.co/unique_id` which then redirects to the original URL. However, twitter displays the original URL.
//! URL Cleaner Site Userscript sends URL Cleaneer Site "this job came from `x.com`" and, for each applicable link, "the text of this link is `https://example.com/whatever`".
//! While twitter truncates long URLs with a ..., the whole original URL is actually still there.
//!
//! By knowing the link is on x.com and the link's text, the default cleaner can skip expanding the redirect with an HTTP request, which is both much faster and doesn't look like you clicked the link.
//!
//! Currently, a [`TaskContext`] contains only [`TaskContext::vars`], a [`HashMap`] from strings to strings.

use std::collections::HashMap;

pub(crate) use super::*;
