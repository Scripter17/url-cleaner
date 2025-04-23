//! Explicit non-consent to URL spytext.
//!
//! Often when a website/app gives you a URL to share to a friend, that URL contains a unique identifier that, when your friend clicks on it, tells the website you sent them that URL.
//! I call this "spytext", as it's text that allows spyware to do spyware things suhc as telling the united states federal government you associate with wrongthinkers.
//!
//! Because of the ongoing human rights catastrophes intentionally enabled by spytext, it is basic decency to remove it before you send a URL, and basic opsec to remove it when you recieve a URL.
//!
//! URL Cleaner is designed to make this as comprehensive, fast, and easy as possible, with the priorities mostly in that order.
//! # PLEASE note that URL Cleaner is not something you should blindly trust!
//! URL Cleaner and its default config are under heavy development, and many websites may break, be partially unhandled, or give incorrect results.
//!
//! While the default config tries its best to minimize info leaks when expanding redirects by both cleaning the URLs before sending the request and only sending a request to the returned value if it's detected as a redirect,
//! this expansion may lead to websites deanonymizing your/your URL Cleaner Site instances IP address. It may also allow malicious enails/comments/DMs to find your IP by buying expired but still handled redirect sites.
//!
//! While URL Cleaner supports using proxies, disabling network access entirely, and doesn't consider hiding the fact you're cleaning URLs in scope,
//! you should be aware that there are going to be edge cases clever attackers can use to betray your confidence in URL Cleaner.
//!
//! Additionally, some redirect websites also put the destination in the URL (`https://example-redirect-site.com/redirect/1234?final_url=https://example.com/`).
//! For these, the default config uses the `final_url` query parameter to skip the network request.
//! It's possible for either the website or the person sending you the link to intentionally mismatch the redirect ID and the `final_url` to send people who use URL Cleaner to different places than people who don't.
//! This attack is very hard for things like email spam filters to detect as it's largely unique to people who clean URLs, which is a very small minority of people who'd report things to spam filter makers.
//! If a website is suceptable to this or similar attacks, then URL Cleaner is considered at fault and a bug report to fix it would be appreciated.
//!
//! Another deanonymization vector involves [URL Cleaner Site](https://github.com/Scripter17/url-cleaner-site) cleaning websites directly in the browser.
//! While this makes it trivial for the website to know you're using URL Cleaner Site, a much bigger issue is that redirects are cached, so if you've seen a redirect before, the website can detect that from both timing and possibly owning the redirect site.
//!
//! While URL Cleaner's default config is definitely a net positive in most cases and when used carefully, you should at no point blindly trust it to be doing things perfectly and you should always be a little paranoid.

pub mod glue;
pub mod types;
pub mod testing;
pub(crate) mod util;

pub use types::{Config, Job, TaskConfig};
