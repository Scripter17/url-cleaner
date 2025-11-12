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

Last updated 2025-11-11.

Also the numbers are in milliseconds.

Without TLS:

```Json
{
  "https://x.com?a=2": {
    "0"    :  8.967,
    "1"    :  8.718,
    "10"   :  8.774,
    "100"  :  8.922,
    "1000" : 10.657,
    "10000": 27.979
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    :  8.722,
    "1"    :  8.722,
    "10"   :  8.803,
    "100"  :  8.992,
    "1000" : 11.701,
    "10000": 36.666
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    :  8.772,
    "1"    :  8.765,
    "10"   :  8.802,
    "100"  :  9.135,
    "1000" : 12.484,
    "10000": 47.171
  }
}
```

With TLS:

```Json
{
  "https://x.com?a=2": {
    "0"    : 24.338,
    "1"    : 24.605,
    "10"   : 24.593,
    "100"  : 24.656,
    "1000" : 26.832,
    "10000": 46.893
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    : 24.662,
    "1"    : 24.511,
    "10"   : 24.646,
    "100"  : 25.143,
    "1000" : 27.678,
    "10000": 57.660
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    : 24.441,
    "1"    : 24.489,
    "10"   : 24.772,
    "100"  : 24.640,
    "1000" : 29.778,
    "10000": 72.015
  }
}
```
