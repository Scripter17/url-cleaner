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

However, using URL Cleaner, and especially URL Cleaner Site, creates a number of privacy concerns of various obviousity.

Using [URL Cleaner Site](site) and its [userscript](site/url-cleaner-site.js), you can automatically clean all links on every website you visit.
This of course comes with the obvious fingerprinting issues where websites can tell you're using URL Cleaner Site, what ParamsDiff you've chosen, and sometimes even which version of the default cleaner it's using.
However, because redirect links (like bit.ly links) are expanded and cached by default, websites can also tell if you've seen a certain link before based on how long it takes for the userscript to replace it.

**This allows malicious websites to fingerprint you basically perfectly.**

You can defend against this by enabling the `no_network` flag and/or by specifying `--read-cache false` on the command line.

In the future, better defenses like artificial delays, cache expiration, and others may or may not (but should) be added.

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
