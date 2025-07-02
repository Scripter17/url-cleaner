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

Last updated 2025-07-02.

```Json
{
  "https://x.com?a=2": {
    "0"    :  8.998,
    "1"    :  9.030,
    "10"   :  8.948,
    "100"  :  9.088,
    "1000" : 11.217,
    "10000": 30.652
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    :  8.849,
    "1"    :  8.832,
    "10"   :  8.899,
    "100"  :  9.093,
    "1000" : 12.285,
    "10000": 43.429
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    :  8.859,
    "1"    :  8.797,
    "10"   :  8.927,
    "100"  :  9.429,
    "1000" : 14.314,
    "10000": 63.623
  }
}
```

And with TLS:

```Json
{
  "https://x.com?a=2": {
    "0"    : 24.202,
    "1"    : 24.256,
    "10"   : 24.187,
    "100"  : 24.408,
    "1000" : 26.872,
    "10000": 48.168
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    : 24.192,
    "1"    : 24.032,
    "10"   : 24.182,
    "100"  : 24.526,
    "1000" : 28.274,
    "10000": 63.902
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    : 24.150,
    "1"    : 24.253,
    "10"   : 24.142,
    "100"  : 24.852,
    "1000" : 30.649,
    "10000": 88.284
  }
}
```

If you're using FireFox, you should know that Greasemonkey gives me much better performance of the userscript than Tampermonkey.  

As for the performance of the userscript itself... I honestly can't say. Nothing strikes me as particularly bad in terms of either CPU or memory usage, but I haven't seriously used javascript in years.  
It probably has a very slow memory leak that would be a problem when on a long-running webpage session having billions of elements, but that's very unlikely to ever happen outside testing.
