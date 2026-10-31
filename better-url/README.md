# Better URL

[![docs.rs](https://img.shields.io/docsrs/better-url)](https://docs.rs/better-url/latest)
[![Crates.io Version](https://img.shields.io/crates/v/better-url)](https://crates.io/crates/better-url/)

A URL crate that's better than the one provided by Servo.

Licensed under the Affero General Public License V3 or later.

https://www.gnu.org/licenses/agpl-3.0.html



# Performance

See [`better-url-bench`](better-url-bench/README.md) for performance details.

TL;DR: It's almost always way faster than Servo's URL crate and usually a respectable fraction the speed of Ada's URL crate.



# Credits

- [WPT tests](https://github.com/web-platform-tests/wpt/tree/master/url/resources) for basic conformance.

- [Ada](https://github.com/ada-url) for both [design inspiration](https://arxiv.org/pdf/2311.10533) and [tests](https://github.com/ada-url/ada/tree/main/tests) for catching issues WPT didn't.

- [Servo's URL crate](https://github.com/servo/rust-url) for some design inspiration and carrying URL Cleaner until I could write Better URL.

- [whatwg/url#914](https://github.com/whatwg/url/pull/914) for singlehandedly saving me from *so* much misery.
