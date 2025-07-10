# URL Cleaner

Explicit non-consent to URL spytext.

Often when a website/app gives you a URL to share to a friend, that URL contains a unique identifier that, when your friend clicks on it, tells the website you sent them that URL.
I call this "spytext", as it's text that allows spyware to do spyware things suhc as telling the united states federal government you associate with wrongthinkers.

Because of the ongoing human rights catastrophes intentionally enabled by spytext, it is basic decency to remove it before you send a URL, and basic opsec to remove it when you recieve a URL.

URL Cleaner is designed to make this as comprehensive, fast, and easy as possible, with the priorities mostly in that order.

# Why specifically this URL cleaner?

URL Cleaner, not to be confused with the many other projects called URL Cleaner or the genre of projects that clean URLs, is comically versatile.

With if-then-else, switch case, and loop-until-no-change control flow, [PSL](https://publicsuffix.org) powered subdomain/domain middle/domain suffix parts,
constructing strings from both the cleaner's params and the URL, setting arbitrary parts of the URL to those strings, regex, base64, and HTTP requests, basically every URL you could ever want to clean is cleanable.

Despite how complex it is, URL Cleaner is very fast! On my Lenovo Thinkpad T460S from 2016, ten thousand amazon product listing URLs can be cleaned in under 50 milliseconds using the included default cleaner.
And that's the CLI program reading from STDIN and writing the cleaned URLs to STDOUT.

URL Cleaner also comes with an HTTP server and a browser userscript for integration with any browser that has a Greasemonkey/Tampermonkey type browser extension.

# Privacy

URL Cleaner and co. will never contain any telemetry. If I ever add it, you are required to kill me on sight.

However, using URL Cleaner Site and its included userscript to clean every URL on every webpage you visit naturally raises a few issues, the majority of which are due to expanding redirect URLs by sending HTTP requests.

If you want to sidestep the entire headache and replace it with a worse one just set the `no_network` flag.

1. Websites can tell you're using URL Cleaner Site and its userscript. It's not hard to tell.

2. Websites can possibly figure out which version of the default cleaner you're using, and pretty easily figure out what params diff you're using.

3. Redirects are cached to reduce information leaks. URL Cleaner also caches how long the redirect took and lets you optionally wait about that long (plus or minus up to 12.5%) when reading from the cache to stop websites from noticing if you have a redirect cached.

4. Even with cache delays, websites can figure out how many threads your instance of URL Cleaner Site is using by measuring how long various amounts of the same redirect takes.
  To defend against this, URL Cleaner has an optional "unthreading" functionality that lets requests, cache reads, etc. be effectively single threaded.

5. Caching at all means the website you're on and the website whose redirect URL you're getting from the cache can check the redirect website's logs to see whether or not you actually sent an HTTP request.

# Default cleaner

See [`default_cleaner.md`](default_cleaner.md) for details about the included default cleaner.

# Performance

URL Cleaner is reasonably fast. See [`cli/README.md#performance`](cli/README.md#performance) and [`site/README.md#performance`](site/README.md#performance) for detials.

TL;DR: On a decade old thinkpad running nothing else, URL Cleaner can do 10000 amazon product URLs in about 50ms.

# Credits

The people and projects I have stolen various parts of the default cleaner from.

- [Mozilla Firefox's Extended Tracking Protection's query stripping](https://firefox-source-docs.mozilla.org/toolkit/components/antitracking/anti-tracking/query-stripping/index.html)
- [Brave Browser's query filter](https://github.com/brave/brave-core/blob/master/components/query_filter/utils.cc)
- [AdGuard's Tracking Parameters Filter](https://github.com/AdguardTeam/AdguardFilters/blob/master/TrackParamFilter/sections)
- [FastForward](https://github.com/FastForwardTeam/FastForward)

# Funding

If for some reason you want to give me money, my paypal is jameschristopherwise@gmail.com.
