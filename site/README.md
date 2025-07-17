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

Last updated 2025-07-16.

```Json
{
  "https://x.com?a=2": {
    "0"    :  8.951,
    "1"    :  8.937,
    "10"   :  8.791,
    "100"  :  8.961,
    "1000" : 11.217,
    "10000": 30.811
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    :  8.739,
    "1"    :  8.799,
    "10"   :  8.757,
    "100"  :  9.109,
    "1000" : 12.395,
    "10000": 44.265
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    :  8.770,
    "1"    :  8.761,
    "10"   :  8.820,
    "100"  :  9.296,
    "1000" : 13.798,
    "10000": 59.673
  }
}
```

And with TLS:

```Json
{
  "https://x.com?a=2": {
    "0"    : 24.138,
    "1"    : 24.091,
    "10"   : 24.131,
    "100"  : 24.319,
    "1000" : 26.939,
    "10000": 49.458
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    : 24.156,
    "1"    : 24.118,
    "10"   : 24.414,
    "100"  : 24.517,
    "1000" : 27.989,
    "10000": 64.597
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    : 24.171,
    "1"    : 24.201,
    "10"   : 24.151,
    "100"  : 24.738,
    "1000" : 29.894,
    "10000": 84.621
  }
}
```

If you're using FireFox, you should know that Greasemonkey gives me much better performance of the userscript than Tampermonkey.

As for the performance of the userscript itself... I honestly can't say. Nothing strikes me as particularly bad in terms of either CPU or memory usage, but I haven't seriously used javascript in years.
It probably has a very slow memory leak that would be a problem when on a long-running webpage session having billions of elements, but that's very unlikely to ever happen outside testing.
