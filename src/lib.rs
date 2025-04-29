//! Explicit non-consent to URL spytext.
//!
//! Often when a website/app gives you a URL to share to a friend, that URL contains a unique identifier that, when your friend clicks on it, tells the website you sent them that URL.
//! I call this "spytext", as it's text that allows spyware to do spyware things suhc as telling the united states federal government you associate with wrongthinkers.
//!
//! Because of the ongoing human rights catastrophes intentionally enabled by spytext, it is basic decency to remove it before you send a URL, and basic opsec to remove it when you recieve a URL.
//!
//! URL Cleaner is designed to make this as comprehensive, fast, and easy as possible, with the priorities mostly in that order.
//!
//! # PLEASE note that URL Cleaner is not something you should blindly trust!
//!
//! URL Cleaner and its default cleaner are under very active development so many websites may break, be partially unhandled, or give otherwise incorrect results.
//! Even if URL Cleaner gives correct results, there are still details you should keep in mind.
//!
//! 1. Unless you set the `no-network` flag, the default cleaner will expand supported redirect websites like `bit.ly` and `t.co`.
//!    In addition to the obvious issues inherent to redirect sites, if one of the supported redirect sites expires, a malicious actor can buy the domain and send you a URL that gets detected as a redirect to find your IP address.
//!
//! 2. Some redirect websites have URLs that contain a redirect ID and the redirect destination. For example, `https://example-redirect-site.com/redirect/1234?destination=https://example.com`.
//!    For these websites, the default cleaner will use the value of the `destination` query parameter to avoid sending a network request.
//!    While this should always be fine for redirect URLs made by the website, it's possible for malicious intermediaries to replace `destination=https://example.com` with `destination=https://evil-website.com`.
//!    It's possible that `example-redirect-site.com` would ignore that redirect 1234 doesn't go to `https://evil-website.com` and still send normal users to `https://example.com`.
//!    If this mismatch happens, it's possible for that malivius intermediary to detect whether or not you're using URL Cleaner if your version of the default cleaner gets the wrong answer.
//!    If a website is vulnerable to this and the default cleaner gets the wrong answer, the default cleaner is considered at fault and must be updates. If you ever see this happen, PLEASE tell me.
//!
//! 3. One of the main intended ways to use URL Cleaner is [URL Cleaner Site](https://github.com/Scripter17/url-cleaner-site), a basic HTTP server + userscript combo to automatically clean every link on every website you visit.
//!    While it's extremely easy for websites to tell you're using URL Cleaner Site, the info leak is both small and impossible to fix, so I won't try to fix it.
//!    What is an issue is that expanded redirects are cached so network requests only have to be sent once. This means that if you see a `https://bit.ly/1234` link for the first time, URL Cleaner Site sends a request for it and caches the result.
//!    Then, any time you see it again, URL Cleaner Site simply references the cache for the destination. While this should always give the correct result, it means that a website can tell whether or not you've seen the link before based on how fast the cleaning happens.
//!
//! While URL Cleaner's default cleaner is definitely a net positive in most cases and when used carefully, you should at no point blindly trust it to be doing things perfectly and you should always be a little paranoid.

pub mod glue;
pub mod types;
pub mod testing;
pub(crate) mod util;

pub use types::{Cleaner, Job, LazyTaskConfig};
