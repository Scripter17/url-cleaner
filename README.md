# URL Cleaner

[Better URL] | [Engine] | [CLI] | [Site] | [Site Userscript] | [Site CLIent] | [Discord]

Explicit non-consent to URL spytext.

## Spytext?

Spytext is text that enables spyware to work. For URLs, spytext usually takes the form of query parameters like `utm_source=...` on any website or `t=...` and `si=...` on twitter.

If you send your friend a tweet, they open it, and neither of you cleaned the URL, then congratulations! Twitter now knows you're friends, and soon so will your government.

You don't consent to this, but unfortunately the law counts not resisting as consent.

With URL Cleaner, you can make your nonconsent explicit and force websites to be just a little bit less evil.

## Frontends

URL Cleaner currently has 3 official frontends:

- [CLI], a command line tool.
- [Site], an HTTP/WebSocket server.
  - Mainly for use with [Site Userscript] and [Site CLIent], but has a convenient API for you to build your own tools around.
- [Discord], a discord app/bot.

You can also make your own frontends by using the [`url_cleaner_engine`] crate.

When doing so, please try to use the [standard format].

## Bundled Cleaner

See [bundled_cleaner.md] for details.

## Performance

See [benchmarks.md] for details.

## Privacy

URL Cleaner and co. will never contain any telemetry. If I ever add it, you are required to kill me on sight.

However, there are two main concerns when using URL Cleaner:

- If the `http` feature is enabled, as is required to use the Bundled Cleaner, ***URL Cleaner can send arbitrary HTTP requests*** to expand redirect URLs.

  - It is generally better to let URL Cleaner expand redirects than your browser, since URL Cleaner doesn't send or accept cookies and caches redirects for future use.

  - To use a proxy, set the `ALL_PROXY` environment variable to, for example, `socks5://127.0.0.1:9150`.

  - To tell the Bundled Cleaner to not make HTTP requests, set the `no_network` Params Var in a ParamsDiff/Profile.

  - To force any Cleaner to not make HTTP requests, use your frontend's method to disable HTTP.

    - For CLI, Site, Site CLIent, and Discord, this is done with the `--no-http` CLI flag.

    - For clients to Site, this can also be done by setting `JobConfig::http` to `false`.

- If the `cache` feature is enabled, as is required to use the Bundled Cleaner, URL Cleaner stores results of previously made HTTP requests in a SQLite file on disk.

  - To store the cache in memory instead of on disk, wiping it when the program closes, set the cache target to `:memory:`.

    - For CLI, Site, and Discord, this is done with the `--cache` CLI option.

  - To prevent reading from the cache, instead treating it as if it were empty, set `JobConfig::read_cache` to `false`

    - For CLI, Site, Site CLIent, and Discord, this is done with the `--no-read-cache` CLI flag.

    - For clients to Site, this can also be done by setting `JobConfig::read_cache` to `false`.

  - To prevent writing to the cache, set `JobConfig::write_cache` to `false`.

    - For CLI, Site, Site CLIent, and Discord, this is done with the `--no-write-cache` CLI flag.

    - For clients to Site, this can also be done by setting `JobConfig::write_cache` to `false`.

Please note that using URL Cleaner Site Userscript also comes with its own privacy concerns, detailed [here](site/userscript.md#privacy).

## Credits

The people and projects I have stolen various parts of the Bundled Cleaner from.

- [Mozilla Firefox's Extended Tracking Protection's query stripping](https://firefox-source-docs.mozilla.org/toolkit/components/antitracking/anti-tracking/query-stripping/index.html).

- [Brave Browser's query filter](https://github.com/brave/brave-core/blob/master/components/query_filter/utils.cc).

- [AdGuard's Tracking Parameters Filter](https://github.com/AdguardTeam/AdguardFilters/blob/master/TrackParamFilter/sections).

- [FastForward](https://github.com/FastForwardTeam/FastForward).

Please also see the [credits for Better URL](./better-url/README.me#credits).

## Funding

If for some reason you want to give me money, my paypal is jameschristopherwise@gmail.com.

[Better URL]: better-url/README.md
[Engine]: engine/README.md
[`url_cleaner_engine`]: engine/README.md
[CLI]: cli/README.md
[Site]: site/README.md
[Site Userscript]: site/README.md#userscript
[Site CLIent]: site-client/README.md
[Discord]: discord/README.md
[standard format]: format.md
[bundled_cleaner.md]: bundled_cleaner.md
[benchmarks.md]: benchmarks.md
