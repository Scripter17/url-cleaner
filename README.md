# URL Cleaner

[Engine](engine) | [CLI tool](cli) | [HTTP server and userscript](site) | [Discord app](discord-app)

Explicit non-consent to URL spytext.

## Spytext?

Spytext is text that enables spyware to work. For URLs, spytext usually takes the form of query parameters like `utm_source=twitter` on any website or `?t=...&si=...` on twitter.

The purpose of spytext in URLs is to allow websites to know who sent which URLs to who. If your friend clicks a tweet you sent them and neither of you removed the spytext,
congratulations! Twitter now knows you're friends, and soon so will your government.

If you are not already aware of why you should not let companies or your government know who you associate with, then I am so sorry but you are not ready for what's coming.

## Why this URL Cleaner?

There are many projects that clean URLs. Most adblockers have some basic URL cleaning built in.

The problem with adblockers is that they exist only in the browser, and the problem with all the other tools I've found is they are extremely inadequate.

URL Cleaner is designed to be as comprehensive, flexible, and fast as possible, with the priorities mostly in that order.

URL Cleaner's engine, URL Cleaner Engine:

- Is just a normal Rust crate, and so can be integrated into almost anything. See the [frontends](#frontends) below.
- Is incredibly fast, cleaning 10 thousand amazon product listing URLs in less than 40 milliseconds on a thinkpad T460S from 2016.
- Supports using profiles to give names to sets of toggles, enabling frontends to only have to compute the toggles once.
- Doesn't use regex and glob for everything.

URL Cleaner also comes bundled with a cleaner called the Bundled Cleaner, which:

- Does all the normal tracking parameter removal.
- Handles redirects.
- Finds ways to skip steps in redirect chains.
- Caches every step in redirect chains.

## Frontends

URL Cleaner currently has 3 official frontends:

- [A CLI tool](cli)
- [An HTTP server and userscript](site)
- [A discord app/bot](discord-app)

You can also make your own frontends by using the [`url_cleaner_engine`](engine) crate.

## Bundled Cleaner

By default, URL Cleaner bundles a cleaner uncreatively called the Bundled Cleaner.

The Bundled Cleaner is meant for general purpose cleaning of URLs you're likely to click on/send to friends and has various flags to allow features you only sometimes want.

For more information, see the [Bundled Cleaner's documentation](bundled_cleaner.md).

# Privacy

URL Cleaner and co. will never contain any telemetry. If I ever add it, you are required to kill me on sight.

However, the Bundled Cleaner will by default expand known redirects by sending HTTP requests and cache those results to a local SQLite database.

To disable network access when using the Bundled Cleaner, enable the `no_network` flag.
For URL Cleaner CLI this can be done with `-f no_network`.
Fer all 3 frontends this can be done with either `ParamsDiff`s or profiles.

To disable writing the cache, you can:
- For either URL Cleaner CLI/URL Cleaner Discord App with the `--no-write-cache` command line argument.
- For URL Cleaner Site, always set `write_cache` to `false`.
- For URL Cleaner Site Userscript, set `write_cache` to `false`.

Please note that using URL Cleaner Site Userscript also comes with its own privacy concerns, detailed [here](site/userscript.md#privacy).

# Credits

The people and projects I have stolen various parts of the Bundled Cleaner from.

- [Mozilla Firefox's Extended Tracking Protection's query stripping](https://firefox-source-docs.mozilla.org/toolkit/components/antitracking/anti-tracking/query-stripping/index.html)
- [Brave Browser's query filter](https://github.com/brave/brave-core/blob/master/components/query_filter/utils.cc)
- [AdGuard's Tracking Parameters Filter](https://github.com/AdguardTeam/AdguardFilters/blob/master/TrackParamFilter/sections)
- [FastForward](https://github.com/FastForwardTeam/FastForward)

# Funding

If for some reason you want to give me money, my paypal is jameschristopherwise@gmail.com.
