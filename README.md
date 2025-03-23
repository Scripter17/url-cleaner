# URL Cleaner

Websites often put unique identifiers into URLs so that when you send a link to a friend and they open it, the website knows it was you who sent it to them.  
As most people do not understand and therefore cannot consent to this, it is polite to remove the maltext before sending URLs to people.  
URL Cleaner is an extremely versatile tool designed to make this process as comprehensive, easy, and fast as possible.

## Privacy

There are some things you need to be careful about both with URL cleaning in general and URL Cleaner specifically.

### Redirect sites

The main privacy concern when using URL Cleaner for day-to-day activities is the fact URL Cleaner, when the `no-network` flag isn't set, expands redirects/shortlinks.  
For example, passing a bit.ly link to URL Cleaner will effectively click on that URL will send an HTTP request to bit.ly.  
While the default config removes as much tracking stuff as possible before sending the request, some redirect sites may merge the sender and the destination information into the same part of the URL.  
For example, if Alice and Bob share the same social media post with you, the social media may give Alice the URL `https://example.com/share/1234` but give Bob the URL `https://example.com/share/5678`.  
In this case, it's impossible (or extremely difficult to find a way) to expand either link without telling the social media who you got the URL from.

In general it's impossible to prove if a redirect website doesn't merge sender and destination information, so one should always assume it is.  
If you consider this a problem, please use the `no-network` flag.  
Some redirect websites will still be handled but that's only because they can be done entirely offline.

### You look like you use URL Cleaner

The lesser main privacy concern is that the default config makes no attempt to hide from websites that you (or the person sending you a link) uses URL Cleaner.  
For example, amazon product listings are shortened from a paragraph of crap to just `https://amazon.com/dp/PRODUCT-ID`.  
In the past (and possibly the future), extreme cases of this were gated behind a `minimize` flag that would try to only remove tracking stuff.  
It was made the default because I consider the benefit from blending into other URL cleaning programs extremely slim.

### Misc.

- For redirect URLs that can be expanded both by clicking on them and by getting the destination from the URL itself (`website.com/go?url=https://...`), it's possible for malicious sources to change the URL so that clicking it and extracting the destination from it give different results. There are currently no plans to address this but the issue is known.

## C dependencies

These packages are required on Kubuntu 2024.04 (and therefore probably all Debian based distros.):

- `libssl-dev` for the `http` feature flag.
- `libsqlite3-dev` for the `caching` feature flag.

There are likely plenty more dependencies required that various Linux distros may or may not pre-install.

If you can't compile it I'll try to help you out. And if you can make it work on your own please let me know so I can add to this list.

## Basic usage

By default, compiling URL Cleaner includes the [`default-config.json`](default-config.json) file in the binary. Because of this, URL Cleaner can be used simply with `url-cleaner "https://example.com/of?a=dirty#url"`.

Additionally, URL Cleaner can take jobs from STDIN lines. `cat urls.txt | url-cleaner` works by printing each result on the same line as its input.

See [Parsing output](#parsing-output) for details on the output format, and yes JSON output is supported.

### The default config

The default config is intended to always obey the following rules:

- "Meaningful semantic changes"<sup>[definition?]</sup> from the input URL and output URL should only ever occur as a result of a flag being enabled.
    - Insignificant details like the item categories navbar on amazon listings being slightly different are insignificant.
- URLs that are "semantically valid"<sup>[definition?]</sup> shouldn't ever return an error.
    - URLs that aren't semantically valid also shouldn't ever throw an error but that is generally less important.
    - URLs that are semantically invalid may become semantically valid if there is an obvious way to do so. See the `unmangle` flag for details.
- Outside of long (>10)/infinite redirects, it should always be idempotent.
- Outside of redirect sites changing behavior, network connectivity issues, or other similarly difficult things to guarantee determinism for, it should always be deterministic.
- The `command` and `custom` features, as well as any features starting with `debug` or `experiment` are never expected to be enabled.
    The `command` feature is enabled by default for convenience but, for situations where untrusted/user-provided configs have a chance to be run, should be disabled.
- It is acceptable for websites to be able to tell a URL has been cleaned, and even for it to know it was URL Cleaner specifically.
    - Blending in with other URL cleaning solutions is not considered a priority.
    - For URL Cleaner Site, this extends to websites knowing they're being tampered with.

Currently no guarantees are made, though when the above rules are broken it is considered a bug and I'd appreciate being told about it.

Additionally, these rules may be changed at any time for any reason. Usually just for clarification.

<!--cmd scripts/gen-docs.py-->
#### Flags

- `breezewiki`: Replace fandom/known Breezewiki hosts with the `breezewiki-host` variable.
- `unbreezewiki`: Replace Breezewiki hosts with fandom.com.
- `nitter`: Replace twitter/known Nitter hosts with the `nitter-host` variable.
- `unnitter`: Replace Nitter hosts with x.com.
- `invidious`: Replace youtube/known Invidious hosts with the `invidious-host` variabel.
- `uninvidious`: Replace Invidious hosts with youtube.com
- `embed-compatibility`: Sets the domain of twitter domiains (and supported twitter redirects like `vxtwitter.com`) to the variable `twitter-embed-host` and `bsky.app` to the variable `bsky-embed-host`.
- `discord-unexternal`: Replace `images-ext-1.discordapp.net` with the original images they refer to.
- `bypass.vip`: Use [bypass.vip](https://bypass.vip) to expand linkvertise and some other links.
- `no-https-upgrade`: Disable upgrading `http` URLs to `https`.
- `no-network`: Don't make any HTTP requests. Some redirect websites will still work because they include the destination in the URL.
- `tor2web2tor`: Replace `**.onion.**` domains with `**.onion` domains.
- `tumblr-unsubdomain-blog`: Changes `blog.tumblr.com` URLs to `tumblr.com/blog` URLs. Doesn't move `at` or `www` subdomains.
- `unmobile`: Convert `https://m.example.com`, `https://mobile.example.com`, `https://abc.m.example.com`, and `https://abc.mobile.example.com` into `https://example.com` and `https://abc.example.com`.
- `youtube-unlive`: Turns `https://youtube.com/live/abc` into `https://youtube.com/watch?v=abc`.
- `youtube-unplaylist`: Removes the `list` query parameter from `https://youtube.com/watch` URLs.
- `youtube-unshort`: Turns `https://youtube.com/shorts/abc` into `https://youtube.com/watch?v=abc`.
- `youtube-unembed`: Turns `https://youtube.com/embed/abc` into `https://youtube.com/watch?v=abc`.
- `youtube-keep-sub-confirmation`: Don't remove the `sub_confirmation` query param from youtube.com URLs.
- `remove-unused-search-query`: Remove search queries from URLs that aren't search results (for example, posts).
- `instagram-unprofilecard`: Turns `https://instagram.com/username/profilecard` into `https://instagram.com/username`.
- `keep-lang`: Keeps language query parameters.
- `furaffinity-sfw`: Turn `furaffinity.net` into `sfw.furaffinity.net`
- `furaffinity-unsfw`: Turn `sfw.furaffinity.net` into `furaffinity.net`

#### Vars

- `breezewiki-host`: The domain to replace fandom/Breezewiki domains with when the `breezewiki` flag is enabled
- `nitter-host`: The domain to replace twitter/nitter domains with when the `nitter` flag is enabled
- `invidious-host`: The domain to replace twitter/Invidious domains with when the `invidious` flag is enabled
- `twitter-embed-host`: The domain to use for twitter when the `embed-compatibility` flag is set. Defaults to `vxtwitter.com`.
- `bluesky-embed-host`: The domain to use for bluesky when the `embed-compatibility` flag is set. Defaults to `fxbsky.com`.
- `bypass.vip-api-key`: The API key used for [bypass.vip](https://bypass.vip)'s premium backend. Overrides the `URL_CLEANER_BYPASS_VIP_API_KEY` environment variable.

#### Environment Vars

- `URL_CLEANER_BYPASS_VIP_API_KEY`: The API key used for [bypass.vip](https://bypass.vip)'s premium backend. Can be overridden with the `bypass.vip-api-key` variable.

#### Sets

- `bypass.vip-hwwwwdpafqdnps`: The `HostWithoutWWWDotPrefixAndFqdnPeriod`es of websites bypass.vip can expand.
- `email-link-format-1-hosts`: (TEMPORARY NAME) Hosts that use unknown link format 1.
- `https-upgrade-host-blacklist`: Hosts to never upgrade from `http` to `https`.
- `redirect-hwwwwdpafqdnps`: Hosts that are considered redirects in the sense that they return HTTP 3xx status codes. URLs with hosts in this set (as well as URLs with hosts that are "www." then a host in this set) will have the `ExpandRedirect` mapper applied.
- `redirect-reg-domains`: The `redirect-hwwwwdpafqdnpes` set but using the `RegDomain` of the URL.
- `remove-empty-fragment-reg-domain-blacklist`: The RegDomains to not remove an empty fragment (the #stuff at the end (but specifically just a #)) from.
- `remove-empty-query-reg-domain-blacklist`: The RegDomains to not remove an empty query from.
- `remove-www-subdomain-reg-domain-blacklist`: `RegDomain`s where a `www` `Subdomain` is important and thus won't have it removed.
- `unmobile-reg-domain-blacklist`: Effectively unsets the `unmobile` flag for the specified `RegDomain`s.
- `utps`: The set of "universal tracking parameters" that are always removed for any URL with a host not in the `utp-host-whitelist` set. Please note that the `utps` common mapper in the default config also removes any parameter starting with any string in the `utp-prefixes` list and thus parameters starting with those can be omitted from this set.
- `utps-reg-domain-whitelist`: RegDomains to never remove universal tracking parameters from.

#### Lists

- `utp-prefixes`: If a query parameter starts with any of the strings in this list (such as `utm_`) it is removed.

#### Maps

- `hwwwwdpafqdnp_lang_query_params`: The name of the `HostWithoutWWWDotPrefixAndFqdnPeriod`'s language query parameter.

#### Named Partitionings

- `hwwwwdpafqdnp_categories`: Categories of similar websites with shared cleaning methods.

#### Jobs Context

##### Vars

- `SOURCE_HOST`: The `Host` of the "source" of the jobs. Usually the webpage it came from.
- `SOURCE_REG_DOMAIN`: The `RegDomain` of the "source" of the jobs, Usually the webpage it came from.

#### Job Context

##### Vars

- `redirect_shortcut`: For links that use redirect sites but have the final URL in the link's text/title/whatever, this is used to avoid sending that HTTP request.
- `site_name`: For furaffinity contact info links, the name of the website the contact info is for. Used for unmangling.
- `link_text`: The text of the link the job came from.
- `bsky_handle`: The handle of an `@user.bsky.social`, used to replace the `/did:plc:12345678` in the URL with the actual handle.
<!--/cmd-->

#### But how fast is it?

Reasonably fast. [`benchmarking/benchmark.sh`] is a Bash script that runs some Hyperfine and Valgrind benchmarking so I can reliably check for regressions.

On a mostly stock lenovo thinkpad T460S (Intel i5-6300U (4) @ 3.000GHz) running Kubuntu 24.10 (kernel 6.11.0) that has "not much" going on (FireFox, Steam, etc. are closed), hyperfine gives me the following benchmark:

Last updated 2025-03-09.

Also the numbers are in milliseconds.

```Json
{
  "https://x.com?a=2": {
    "0"    :  7.383,
    "1"    :  7.478,
    "10"   :  7.485,
    "100"  :  7.840,
    "1000" : 10.115,
    "10000": 32.909
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    :  7.380,
    "1"    :  7.451,
    "10"   :  7.549,
    "100"  :  7.872,
    "1000" : 11.211,
    "10000": 45.314
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    :  7.378,
    "1"    :  7.391,
    "10"   :  7.614,
    "100"  :  8.530,
    "1000" : 12.563,
    "10000": 60.176
  }
}
```

For reasons not yet known to me, everything from an Intel i5-8500 (6) @ 4.100GHz to an AMD Ryzen 9 7950X3D (32) @ 5.759GHz seems to max out at between 140 and 110ms per 100k (not a typo) of the amazon URL despite the second CPU being significantly more powerful.

In practice, when using [URL Cleaner Site and its userscript](https://github.com/Scripter17/url-cleaner-site), performance is significantly (but not severely) worse.  
Often the first few cleanings will take a few hundred milliseconds each because the page is still loading.  
However, because of the overhead of using HTTP (even if it's just to localhost) subsequent cleanings, for me, are basically always at least 10ms.

#### Credits

The people and projects I have stolen various parts of the default config from.

- [Mozilla Firefox's Extended Tracking Protection's query stripping](https://firefox-source-docs.mozilla.org/toolkit/components/antitracking/anti-tracking/query-stripping/index.html)
- [Brave Browser's query filter](https://github.com/brave/brave-core/blob/master/components/query_filter/utils.cc)
- [AdGuard's Tracking Parameters Filter](https://github.com/AdguardTeam/AdguardFilters/blob/master/TrackParamFilter/sections)
- [FastForward](https://github.com/FastForwardTeam/FastForward)

## MSRV

The Minimum Supported Rust Version is the latest stable release. URL Cleaner may or may not work on older versions, but there's no guarantee.

## Untrusted input

Although URL Cleaner has various feature flags that can be disabled at compile time to make handling untrusted input safer, no guarantees are made. Especially if the config file being used is untrusted.  
That said, if you notice any rules that use but don't actually need HTTP requests or other data-leaky features, please let me know.

## CLI

### Parsing output

Note: [JSON output is supported](#json-output).

Unless a `Debug` variant is used, the following should always be true:

1. Input URLs are a list of URLs starting with URLs provided as command line arguments then each line of the STDIN.
2. The nth line of STDOUT corresponds to the nth input URL.
3. If the nth line of STDOUT is empty, then something about reading/parsing/cleaning the URL failed.
4. The nth non-empty line of STDERR corresponds to the nth empty line of STDOUT.
    1. Currently empty STDERR lines are not printed when a URL succeeds. While it would make parsing the output easier it would cause visual clutter on terminals. While this will likely never change by default, parsers should be sure to follow 4 strictly in case this is added as an option.

### JSON output

The `--json`/`-j` flag can be used to have URL Cleaner output JSON instead of lines.

The exact format is currently in flux, though it should always be identical to [URL Cleaner Site](https://github.com/Scripter17/url-cleaner-site)'s output.

### Exit code

Currently, the exit code is determined by the following rules:

- If no   cleanings work and none fail, returns 0. This only applies if no URLs are provided.
- If no   cleanings work and some fail, returns 1.
- If some cleanings work and none fail, returns 0.
- If some cleanings work and some fail, returns 2. This only applies if multiple URLs are provided.

## Panic policy

URL Cleaner should only ever panic under the following circumstances:

- Parsing the CLI arguments failed.
- Loading/parsing the config failed.
- Printing the config failed. (Shouldn't be possible.)
- Testing the config failed.
- Reading from/writing to STDIN/STDOUT/STDERR has a catastrophic error.
- Running out of memory resulting in a standard library function/method panicking. This should be extremely rare.
- (Only possible when the `debug` feature is enabled) The mutex controlling debug printing indenting is poisoned and a lock is attempted.
    This should only be possible when URL Cleaner is used as a library.

Outside of these cases, URL Cleaner should never panic. However as this is equivalent to saying "URL Cleaner has no bugs", no actual guarantees can be made.

## Funding

URL Cleaner does not accept donations. If you feel the need to donate please instead donate to [The Tor Project](https://donate.torproject.org/) and/or [The Internet Archive](https://archive.org/donate/).
