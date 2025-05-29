# URL Cleaner Site

[![Crates.io Version](https://img.shields.io/crates/v/url-cleaner-site)](https://crates.io/crates/url-cleaner-site/)

A simple HTTP server to allow using URL Cleaner in web browser userscripts and other applications where SSH tunnels are infeasable.

## Default cleaner.

See [`../default_cleaner.md`](../default_cleaner.md) for details about the included default cleaner.

## TLS/HTTPS

TLS/HTTPS can be used with the `--key` and `--cert` arguments.  
[Minica](https://github.com/jsha/minica) makes it easy to have stuff shut up about self signed certificates.  
For FireFox, where this is unreasonably difficult, simply opening `https://localhost:9149`, clicking "Advanced", then "Accept the Risk and Continue" seems to work.

Please note that this requires changing `window.URL_CLEANER_SITE = "http://localhost:9149";` in the userscript to https.

Currently the default port of 9149 applies to both HTTP and HTTPS servers.

## Performance

Due to the overhead of using HTTP, getting all the jobs before running them, and optionally TLS, performance is significantly worse than the CLI.

On the same laptop used in URL Cleaner's example benchmarks and with TLS, hyperfine (using CURL) gave me the following benchmarks:

Without TLS, the benchmarks are about 15ms faster, but the worst case scenario is provided because it's more useful.

Last updated 2025-05-26.

```Json
{
  "https://x.com?a=2": {
    "0"    : 24.297,
    "1"    : 24.373,
    "10"   : 24.478,
    "100"  : 24.757,
    "1000" : 27.617,
    "10000": 55.571
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    : 24.276,
    "1"    : 24.434,
    "10"   : 24.397,
    "100"  : 24.879,
    "1000" : 29.133,
    "10000": 73.093
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    : 24.278,
    "1"    : 24.450,
    "10"   : 24.375,
    "100"  : 25.045,
    "1000" : 31.305,
    "10000": 92.470
  }
}
```

If you're using FireFox, you should know that Greasemonkey gives me much better performance of the userscript than Tampermonkey.  

As for the performance of the userscript itself... I honestly can't say. Nothing strikes me as particularly bad in terms of either CPU or memory usage, but I haven't seriously used javascript in years.  
It probably has a very slow memory leak that would be a problem when on a long-running webpage session having billions of elements, but that's very unlikely to ever happen outside testing.
