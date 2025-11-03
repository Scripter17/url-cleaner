# URL Cleaner Site

[![Crates.io Version](https://img.shields.io/crates/v/url-cleaner-site)](https://crates.io/crates/url-cleaner-site/)

[Documentation for URL Cleaner in general](../README.md)

A simple HTTP server to allow using URL Cleaner in web browser userscripts and other applications where SSH tunnels are infeasable.

Licensed under the Affero General Public License V3 or later.

https://www.gnu.org/licenses/agpl-3.0.html

## Usage

### Start URL Cleaner Site

Once you've downloaded/compiled URL Cleaner Site, all you need to do is open it.

For more advacned usage, please see [the server management guide](server.md)

### Userscript

The main way to use URL Cleaner Site is with URL Cleaner Site Userscript to clean every URL on every webpage you visit.

See [userscript.md](userscript.md) for details.

### API

See the [API documentation](api.md) for how to interact with URL Cleaner Site.

## Performance

On a mostly stock lenovo thinkpad T460S (Intel i5-6300U (4) @ 3.000GHz) running Kubuntu 25.04 (kernel 6.14.0) that has "not much" going on (FireFox, Steam, etc. are closed), hyperfine gives me the following benchmark:

Last updated 2025-11-02.

Also the numbers are in milliseconds.

Without TLS:

```Json
{
  "https://x.com?a=2": {
    "0"    :  9.043,
    "1"    :  8.769,
    "10"   :  8.858,
    "100"  :  8.983,
    "1000" : 10.760,
    "10000": 27.681
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    :  8.809,
    "1"    :  8.822,
    "10"   :  8.793,
    "100"  :  9.056,
    "1000" : 11.627,
    "10000": 37.025
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    :  8.797,
    "1"    :  8.812,
    "10"   :  8.840,
    "100"  :  9.208,
    "1000" : 12.607,
    "10000": 47.874
  }
}
```

With TLS:

```Json
{
  "https://x.com?a=2": {
    "0"    : 24.389,
    "1"    : 24.455,
    "10"   : 24.989,
    "100"  : 24.575,
    "1000" : 27.432,
    "10000": 47.799
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    : 24.831,
    "1"    : 25.058,
    "10"   : 24.711,
    "100"  : 24.755,
    "1000" : 27.993,
    "10000": 57.524
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    : 24.542,
    "1"    : 24.384,
    "10"   : 24.498,
    "100"  : 24.957,
    "1000" : 29.089,
    "10000": 72.968
  }
}
```
