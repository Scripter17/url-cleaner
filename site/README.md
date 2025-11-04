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

Last updated 2025-11-03.

Also the numbers are in milliseconds.

Without TLS:

```Json
{
  "https://x.com?a=2": {
    "0"    :  9.074,
    "1"    :  8.833,
    "10"   :  9.044,
    "100"  :  9.038,
    "1000" : 11.324,
    "10000": 27.265
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    :  8.822,
    "1"    :  8.768,
    "10"   :  8.868,
    "100"  :  9.028,
    "1000" : 11.610,
    "10000": 36.859
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    :  8.994,
    "1"    :  8.990,
    "10"   :  9.023,
    "100"  :  9.215,
    "1000" : 12.702,
    "10000": 47.536
  }
}
```

With TLS:

```Json
{
  "https://x.com?a=2": {
    "0"    : 24.427,
    "1"    : 24.317,
    "10"   : 25.239,
    "100"  : 24.694,
    "1000" : 26.882,
    "10000": 47.053
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    : 24.448,
    "1"    : 24.381,
    "10"   : 24.990,
    "100"  : 24.779,
    "1000" : 28.139,
    "10000": 57.262
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    : 24.663,
    "1"    : 24.384,
    "10"   : 24.640,
    "100"  : 24.537,
    "1000" : 29.823,
    "10000": 73.091
  }
}
```
