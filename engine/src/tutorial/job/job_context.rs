//! # [`JobContext`]
//!
//! Sometimes you'll want to give a cleaner context on where a job came from.
//! This is mainly used along with [task context](task_context) by URL Cleaner Site and URL Cleaner Site Userscript to provide site-specific optimizations.
//!
//! For example, on twitter, links in tweets are changed to point to `https://t.co/unique_id` which then redirects to the original URL. However, twitter displays the original URL.
//! URL Cleaner Site Userscript sends URL Cleaneer Site "this job came from `x.com`" and, for each applicable link, "the text of this link is `https://example.com/whatever`".
//! While twitter truncates long URLs with a ..., the whole original URL is actually still there.
//!
//! Using this information, the default cleaner uses the link's text instead of expanding the redirect, which is both much faster and doesn't look like you clicked the link.
//!
//! Currently, a [`JobContext`] contains only [`JobContext::source_host`], a [`BetterHost`] that's used with [`StringSource::JobSourceHostPart`] to avoid the whole mess that comes with sending the source host as just a string,
//! and [`JobContext::vars`], a now unused [`HashMap`] from strings to strings.

pub(crate) use super::*;
