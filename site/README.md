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

Last updated 2025-07-09.

```Json
{
  "https://x.com?a=2": {
    "0"    :  8.935,
    "1"    :  8.793,
    "10"   :  8.732,
    "100"  :  8.956,
    "1000" : 11.361,
    "10000": 31.218
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    :  8.678,
    "1"    :  8.674,
    "10"   :  8.717,
    "100"  :  9.085,
    "1000" : 12.227,
    "10000": 44.435
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    :  8.707,
    "1"    :  8.697,
    "10"   :  8.799,
    "100"  :  9.402,
    "1000" : 14.246,
    "10000": 63.412
  }
}
```

And with TLS:

```Json
{
  "https://x.com?a=2": {
    "0"    : 24.138,
    "1"    : 24.058,
    "10"   : 24.151,
    "100"  : 24.227,
    "1000" : 26.830,
    "10000": 49.048
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    : 23.946,
    "1"    : 23.954,
    "10"   : 24.275,
    "100"  : 24.534,
    "1000" : 28.176,
    "10000": 64.751
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    : 24.176,
    "1"    : 24.127,
    "10"   : 24.250,
    "100"  : 24.673,
    "1000" : 30.469,
    "10000": 88.423
  }
}
```

If you're using FireFox, you should know that Greasemonkey gives me much better performance of the userscript than Tampermonkey.  

As for the performance of the userscript itself... I honestly can't say. Nothing strikes me as particularly bad in terms of either CPU or memory usage, but I haven't seriously used javascript in years.  
It probably has a very slow memory leak that would be a problem when on a long-running webpage session having billions of elements, but that's very unlikely to ever happen outside testing.
