# Better URL

[![docs.rs](https://img.shields.io/docsrs/better-url)](https://docs.rs/better-url/latest)
[![Crates.io Version](https://img.shields.io/crates/v/better-url)](https://crates.io/crates/better-url/)

A URL crate that's better than the one provided by Servo.

Licensed under the Affero General Public License V3 or later.

https://www.gnu.org/licenses/agpl-3.0.html

## Performance

See [`better-url-bench`](../better-url-bench) for performance details.

TL;DR: It's almost always way faster than Servo's URL crate and usually a respectable fraction the speed of Ada's URL crate.

## Credits

Several tests have been taken from [Ada](https://github.com/ada-url), catching many things the [WPT tests](https://github.com/web-platform-tests/wpt/tree/master/url/resources) don't.
