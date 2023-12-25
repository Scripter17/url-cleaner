# Url Cleaner

A configurable URL cleaner built in Rust

## Basic usage

By default, compiling URL Cleaner includes the `default-config.json` file in the binary. Because of this, the URL Cleaner can be used simply with `url-cleaner https://example.com/of?a=dirty#url`.

For example, a `twitter.com`, `vxtwitter.com`, `fxtwitter.com`, or a `x.com` URL will have the host changed to `twitter.com` and remove any query paramaters (The `?s=number&t=whatever` stuff that does nothing).  
While the default configuration is limited to just what I need it to do, custom rule configurations can be passed in at runtime so you don't need to recompile the binary every time you want to sanitize a new website.

## Rule sources

- [Brave Browser's query filter](https://github.com/brave/brave-core/blob/ed5fa80c20295ab7f82ab22233531bcc241b9700/components/query_filter/utils.cc#L22)
- [Mozilla Firefox's Extended Tracking Protection's query stripping](https://firefox-source-docs.mozilla.org/toolkit/components/antitracking/anti-tracking/query-stripping/index.html)
- [AdGuard's Tracking Paramaters Filter](https://github.com/AdguardTeam/AdguardFilters/blob/master/TrackParamFilter/sections)
