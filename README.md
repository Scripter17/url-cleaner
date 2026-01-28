# URL Cleaner

[Engine](engine) | [CLI](cli) | [Site and Site Userscript](site) | [Site CLIent](site-client) | [Discord](discord)

Explicit non-consent to URL spytext.

## Spytext?

Spytext is text that enables spyware to work. For URLs, spytext usually takes the form of query parameters like `utm_source=twitter` on any website or `?t=...&si=...` on twitter.

The purpose of spytext in URLs is to allow websites to know who sent which URLs to who. If your friend clicks a tweet you sent them and neither of you removed the spytext,
congratulations! Twitter now knows you're friends, and soon so will your government.

If you are not already aware of why you should not let companies or your government know who you associate with, then I am so sorry but you are not ready for what's coming.

## Frontends

URL Cleaner currently has 3 official frontends:

- [A CLI tool](cli)
- [An HTTP/WebSocket server and userscript](site)
  - [A CLI client for Site](site-client)
- [A discord app/bot](discord)

You can also make your own frontends by using the [`url_cleaner_engine`](engine) crate.

When doing so, please try to use the [standard format](format.md).

## Bundled Cleaner

By default, URL Cleaner bundles a cleaner uncreatively called the Bundled Cleaner.

The Bundled Cleaner is meant for general purpose cleaning of URLs you're likely to click on/send to friends and has various flags to allow features you only sometimes want.

For more information, see the [Bundled Cleaner's documentation](bundled_cleaner.md) and its [benchmarks](benchmarks.md).

## Privacy

URL Cleaner and co. will never contain any telemetry. If I ever add it, you are required to kill me on sight.

However, the Bundled Cleaner will by default expand and cache known redirects by sending HTTP requests and writing a SQLite database to disk.

To use proxies, see [reqwest's documentation](https://docs.rs/reqwest/latest/reqwest/#proxies).

To disable network access, set the `no_network` flag.

To disable reading from the cache, use `--no-read-cache`.

To disable writing to the cache, use `--no-write-cache`.

Please note that using URL Cleaner Site Userscript also comes with its own privacy concerns, detailed [here](site/userscript.md#privacy).

## Credits

The people and projects I have stolen various parts of the Bundled Cleaner from.

- [Mozilla Firefox's Extended Tracking Protection's query stripping](https://firefox-source-docs.mozilla.org/toolkit/components/antitracking/anti-tracking/query-stripping/index.html)
- [Brave Browser's query filter](https://github.com/brave/brave-core/blob/master/components/query_filter/utils.cc)
- [AdGuard's Tracking Parameters Filter](https://github.com/AdguardTeam/AdguardFilters/blob/master/TrackParamFilter/sections)
- [FastForward](https://github.com/FastForwardTeam/FastForward)

## Funding

If for some reason you want to give me money, my paypal is jameschristopherwise@gmail.com.
