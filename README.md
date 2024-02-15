# Url Cleaner

A configurable URL cleaner built in Rust

## Basic usage

By default, compiling URL Cleaner includes the [`default-config.json`](default-config.json) file in the binary. Because of this, URL Cleaner can be used simply with `url-cleaner "https://example.com/of?a=dirty#url"`.

The default config shouldn't ever change the semantics of a URL. Opening a URL before and after cleaning should always give the same result.  
Because websites tend to not document what parts of their URLs are and aren't necessary, the default config almost certainly runs into issues when trying to clean niche URLs like advanced search queries or API endpoints.  
If you find any instance of the default config changing the meaning/result of a URL, please open an [issue](https://github.com/Scripter17/url-cleaner/issues).

Additionally, if you find any example of a malformed URL that can be unambiguously transformed into what was intended (`https://abc.tumblr.com.tumblr.com` -> `https://abc.tumblr.com` and `https://bsky.app/profile/abc` -> `https://bsky.app/profile/abc.bsky.social`), please open an issue.  
Since these are somewhat common when social media sites have dedicated fields for other social medias, it's worth handling these.

## Variables

While the `VariableIs` condition and the `--var` flag allow setting variables to refine URL handling, currently none of the default config uses them.  
Additionally, proper integration into mappers and conditions is currently somewhat lacking.

## Flags

Like variables except the only defined as "set" and "not set", various flags are included in the default config for things I want to do frequently.

- `discord-compatibility`: Turns `twitter.com` and `deviantart` URLs into `vxtwitter.com` and `fxdeviantart` URLs.
- `youtube-unshort`: Turns `https://youtube.com/shorts/abc` URLs into `https://youtube.com/watch?v=abc` URLs.
- `tumblr-strip-reblogs`: Replace tumblr reblog links with the original post. Does not care about reblog chains so maybe don't set this in your keyboard shortcut.
- `antifandom`: Turns `abc.fandom.com` URLs into `antifandom.com/abc` URLs.
- `unfix-domains`: Replace `antifandom.com` URLs with `fandom.com` URLs.

Flags can be added to configs by using the `FlagSet` condition and specified at runtime by doing `--flag flag1 --flag flag2`.

## Custom rules

Although proper documentation of the config schema is pending me being bothered to do it, the `url_cleaner` crate itself is well documented and the structs and enums are (I think) fairly easy to understand.  
The main files you want to look at are [`conditions.rs`](src/rules/conditions.rs) and [`mappers.rs`](src/rules/mappers.rs).  
Additionally [`url_part.rs`](src/types/url_part.rs), [`string_location.rs`](src/types/string_location.rs), and [`string_modification.rs`](src/types/string_modification.rs) are very important for more advanced rules.

Tips for people who don't know Rust's syntax:

- If a field's type is `Option<...>` that just means it can be `null` in the JSON. `{"abc": "xyz"}` and `{"abc": null}` are both valid states for a `abc: Option<String>` field.
- If a field's type is `Box<...>` you don't need to worry about it. `Box`es are just used to let things contain other things of the same type. They have no bearing on JSON serialization.
- `Vec<...>` and `HashSet<...>` are written as lists in JSON.
- `HashMap<..., ...>` and `HeaderMap` are written as dictionaries in JSON.
- Fields preceeded by `#[serde(default)]` or `#[serde(default = "...")]` can be omitted from config files. The defaults are almost always what you want.
- `u8`, `u16`, `u32`, `u64`, `u128`, and `usize` are unsigned (non-negative) integers. `i8`, `i16`, `i32`, `i64`, `i128`, and `isize` are signed integers. `usize` is a `u32` on 32-bit computers and `u64` on 64-bit computers. Likewise `isize` is `i32` and `i64` under the same conditions. Basically if a number makes sense to be used in a field then it'll fit.

## Anonymity

Because most people don't use URL Cleaner, using URL Cleaner can let websites correlate information similar to URL tracking parameters.  
If you're the only person without a tracking parameter in links, it's fairly easy to distinguish you from everyone else.

As with Tor, safety comes in numbers.

## MSRV

The Minimum Supported Rust Version is the latest stable release. URL Cleaner may or may not work on older versions, but there's no guarantee.

## Untrusted input

Although URL Cleaner has various feature flags that can be disabled to make handling untrusted input safer, no guarantees are made. Especially if the config file being used is untrusted.  
That said, if you find something to be unnecessarily unsafe, please open an issue so it can be fixed.

## Backwards compatibility

Although URL Cleaner is currently in beta, I will do my best to not make breaking changed without good reasons. This includes making things that error no longer error.

Changes to the default config that makes partial feature sets not work is not considered a breaking change. The default config will always assume all default features are enabled.

## Default config sources

The people and projects I have stolen various parts of the default config from.

- [Mozilla Firefox's Extended Tracking Protection's query stripping](https://firefox-source-docs.mozilla.org/toolkit/components/antitracking/anti-tracking/query-stripping/index.html)
- [Brave Browser's query filter](https://github.com/brave/brave-core/blob/master/components/query_filter/utils.cc)
- [AdGuard's Tracking Parameters Filter](https://github.com/AdguardTeam/AdguardFilters/blob/master/TrackParamFilter/sections)
