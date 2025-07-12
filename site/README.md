# URL Cleaner Site

[![Crates.io Version](https://img.shields.io/crates/v/url-cleaner-site)](https://crates.io/crates/url-cleaner-site/)

A simple HTTP server to allow using URL Cleaner in web browser userscripts and other applications where SSH tunnels are infeasable.

See the [included userscript](url-cleaner-site.js) to easily use URL Cleaner Site with any browser.

## Default cleaner

See [`../default_cleaner.md`](../default_cleaner.md) for details about the included default cleaner.

## TLS/HTTPS

TLS/HTTPS can be used with the `--key` and `--cert` arguments.
[Minica](https://github.com/jsha/minica) makes it easy to have stuff shut up about self signed certificates.
For FireFox, where this is unreasonably difficult, simply opening `https://localhost:9149`, clicking "Advanced", then "Accept the Risk and Continue" seems to work.

Please note that this requires changing `window.URL_CLEANER_SITE = "http://localhost:9149";` in the userscript to https.

Currently the default port of 9149 applies to both HTTP and HTTPS servers.

## Performance

Due to the overhead of using HTTP, the lack of streaming tasks and results, and optionally TLS, performance is significantly worse than the CLI.

On the same laptop used in URL Cleaner's example benchmarks and without TLS, hyperfine (using CURL) gave me the following benchmarks:

Last updated 2025-07-11.

```Json
{
  "https://x.com?a=2": {
    "0"    :  8.932,
    "1"    :  8.717,
    "10"   :  8.701,
    "100"  :  8.953,
    "1000" : 11.327,
    "10000": 30.968
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    :  8.671,
    "1"    :  8.699,
    "10"   :  8.712,
    "100"  :  9.060,
    "1000" : 12.322,
    "10000": 44.603
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    :  8.719,
    "1"    :  8.745,
    "10"   :  8.793,
    "100"  :  9.251,
    "1000" : 13.860,
    "10000": 59.026
  }
}
```

And with TLS:

```Json
{
  "https://x.com?a=2": {
    "0"    : 24.033,
    "1"    : 24.187,
    "10"   : 24.176,
    "100"  : 24.457,
    "1000" : 26.814,
    "10000": 49.252
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    : 24.076,
    "1"    : 24.212,
    "10"   : 24.141,
    "100"  : 24.496,
    "1000" : 28.022,
    "10000": 65.530
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    : 24.039,
    "1"    : 24.179,
    "10"   : 24.125,
    "100"  : 24.739,
    "1000" : 29.932,
    "10000": 83.002
  }
}
```

If you're using FireFox, you should know that Greasemonkey gives me much better performance of the userscript than Tampermonkey.

As for the performance of the userscript itself... I honestly can't say. Nothing strikes me as particularly bad in terms of either CPU or memory usage, but I haven't seriously used javascript in years.
It probably has a very slow memory leak that would be a problem when on a long-running webpage session having billions of elements, but that's very unlikely to ever happen outside testing.
