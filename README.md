# Url Cleaner

A configurable URL cleaner built in Rust

## Basic usage

By default, compiling URL Cleaner includes the [`default-rules.json`](default-rules.json) file in the binary. Because of this, URL Cleaner can be used simply with `url-cleaner "https://example.com/of?a=dirty#url"`.

In general, the default rules are meant for URLs that one is likely to send or receive to or from other people. Internal API requests or search queries are not guaranteed to be handled properly.
As such, it is currently recommended that you do not make a browser extension that automatically passes all HTTP requests into URL Cleaner. I know it's tempting but give me a bit to work out all the kinks.

## Anonymity

Because most people don't use URL Cleaner, using URL Cleaner can let websites correlate information similar to URL tracking parameters.
This is a similar situation to using Tor Browser where, although everyone who uses Tor Browser looks the same, it is easy for websites to tell that you're using Tor Browser.

## Rule sources

- [Brave Browser's query filter](https://github.com/brave/brave-core/blob/ed5fa80c20295ab7f82ab22233531bcc241b9700/components/query_filter/utils.cc#L22)
- [Mozilla Firefox's Extended Tracking Protection's query stripping](https://firefox-source-docs.mozilla.org/toolkit/components/antitracking/anti-tracking/query-stripping/index.html)
- [AdGuard's Tracking Parameters Filter](https://github.com/AdguardTeam/AdguardFilters/blob/master/TrackParamFilter/sections)
