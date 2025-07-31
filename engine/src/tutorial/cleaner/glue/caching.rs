//! # [Caching](crate::glue::caching)
//!
//! URL Cleaner Engine uses a Sqlite database to cache basically just network requests.
//!
//! The database has one table, `cache` with 4 columns:
//!
//! - `subject` (`TEXT NOT NULL`): The subject of the cache entry. For example, redirects have their `subject` set to `redirect`.
//!
//! - `key` (`TEXT NOT NULL`): The key of the key/value pair. For example, redirects have their `key` set to the redirect URL.
//!
//! - `value` (`TEXT` (maybe null)): The value of the key/value pair. For example, redirects have their `value` set to the URL the starting redirect URL points to.
//!
//! - `duration` (`FLOAT`): The amount of time (in seconds) it took to do the thing being cached. For example, redirects have their `duration` set to about as long as it took to do the network request(s). This is used by [`CacheHandle`] to artificially delay cache reads if [`CacheHandleConfig::delay`] is [`true`] to reduce the ability of websites to tell if you've seen a certain URL before.
//!
//! Every pair of `subject` and `key` is unique.
//!
//! ## Defending against cache detection
//!
//! When using URL Cleaner Site Userscript, websites can see you're using URL Cleaner Site Userscript by checking if URLs are being cleaned.
//!
//! "Cache detection" is the problem where a website can tell you've seen a redirect URL before by the fact it got cleaned significantly faster than a network request would take.
//!
//! URL Cleaner Engine has a defense against cache detection and a defense against thread count detection that happens to also be theoretically useful for defending against cache detection.
//!
//! 1. Artificial delays. When enabled, getting an existing entry from the cache will wait as long as its `duration` column plus or minus anywhere from 0% to 12.5%.
//!    For example, a redirect that took 2 seconds will have an artificial delay anywhere from 1.75 seconds to 2.25 seconds. This is to simulate the normal variance network requests have.
//!    It's not intended to be perfect, it's just intended to make it really annoying for websites to distinguish from the regular variance you would have from your specific network to that specific website.
//!
//! 2. Unthreading. When a user of URL Cleaner Engine has enabled multithreading, cleaning 100 of a redirect will take as long as 100 minus the number of threads.
//!    To defend against both thread count detection and cache detection, jobs can enable unthreading which forces network requests and cache reads to happen one at a time.
//!
//! Both URL Cleaner's CLI and URL Cleaner Site disable these defenses by default, with URL Cleaner Site Userscript enabling them only for its jobs.
//!
//! An method of cache detection that is not and likely cannot be defended against is noticing that a redirect gets expanded when the website is offline.
//! For example, purely hypothetically because there's no way such a stupid thing would ever happen, if `goo.gl` were to go offline, a website that sees a `goo.gl` redirect get expanded would know it had to come from your cache.

pub(crate) use super::*;
