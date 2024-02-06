# Url Cleaner

A configurable URL cleaner built in Rust

## Basic usage

By default, compiling URL Cleaner includes the [`default-config.json`](default-config.json) file in the binary. Because of this, URL Cleaner can be used simply with `url-cleaner "https://example.com/of?a=dirty#url"`.

In general, the default rules are meant for URLs that one is likely to send or receive to or from other people. Internal API requests or search queries are not guaranteed to be handled properly.  
Additionally, while URL Cleaner is more than capable of handling most URLs, the default config almost certainly contains many issues with things like website search queries.  
If you find any such issues, please open an issue.

## Variables

While the `VariableIs` condition and the `--var` flag allow setting variables to refine URL handling, currently none of the default config uses them.  
Additionally, proper integration (via stuff like the `SetPart` mapper) is currently lacking.

## Flags

Like variables except the only defined as "set" and "not set", various flags are included in the default config for things I want to do frequently.

- `discord-compatibility`: Turns `twitter.com` and `deviantart` URLs into `vxtwitter.com` and `fxdeviantart` URLs.
- `youtube-unshort`: Turns `https://youtube.com/shorts/abc` URLs into `https://youtube.com/watch?v=abc` URLs.
- `antifandom`: Turns `abc.fandom.com` URLs into `antifandom.com/abc` URLs.
- `tumblr-strip-reblogs`: Replace tumblr reblog links with the original post. Does not care about reblog chains so maybe don't set this in your keyboard shortuct.

Flags can be added to configs by using the `FlagSet` condition and specified at runtime by doing `--flag flag1 --flag flag2`

## Custom rules

Although proper documentation of the config schema is pending me being bothered to do it, the `url_cleaner` crate itself is well documented and the structs ansd enums are (I think) fairly easy to understand.  
The main files you want to look at are [`conditions.rs`](src/rules/conditions.rs) and [`mappers.rs`](src/rules/mappers.rs).  
Additionally [`url_part.rs`](src/types/url_part.rs), [`string_location.rs`](src/types/string_location.rs), and [`string_modification.rs`](src/types/string_modification.rs) are very important for more advanced rules.

Quick tip for those who don't know Rust: If a field's type is `Option<...>` that just means it can be `null` in the JSON. `{"abc": "xyz"}` and `{"abc": null}` are both valid states for a `abc: Option<String>` field.

## Anonymity

Because most people don't use URL Cleaner, using URL Cleaner can let websites correlate information similar to URL tracking parameters.  
If you're the only person without a tracking parameter in links, it's fairly easy to distinguish you from everyone else.

## MSRV

The Minimum Supported Rust Version is the latest stable release. URL Cleaner may or may not work on older versions, but there's no guarantee.

## Default config sources

The people and projects I have stolen various parts of the default config from.

- [Brave Browser's query filter](https://github.com/brave/brave-core/blob/ed5fa80c20295ab7f82ab22233531bcc241b9700/components/query_filter/utils.cc#L22)
- [Mozilla Firefox's Extended Tracking Protection's query stripping](https://firefox-source-docs.mozilla.org/toolkit/components/antitracking/anti-tracking/query-stripping/index.html)
- [AdGuard's Tracking Parameters Filter](https://github.com/AdguardTeam/AdguardFilters/blob/master/TrackParamFilter/sections)
