# URL Cleaner Site

[![Crates.io Version](https://img.shields.io/crates/v/url-cleaner-site)](https://crates.io/crates/url-cleaner-site/)

[Documentation for URL Cleaner in general](../README.md)

A simple HTTP server to allow using URL Cleaner in web browser userscripts and other applications where SSH tunnels are infeasable.

Licensed under the Affero General Public License V3 or later.

https://www.gnu.org/licenses/agpl-3.0.html

## Usage

### Start URL Cleaner Site

Once you've downloaded/compiled URL Cleaner Site, all you *need* to do is open it.

For more advacned usage, please see [the server management guide](server.md)

### URL Cleaner Site Userscript

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
    "0"    :  9.042,
    "1"    :  8.815,
    "10"   :  8.866,
    "100"  :  8.973,
    "1000" : 11.124,
    "10000": 27.543
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    :  8.769,
    "1"    :  8.787,
    "10"   :  8.820,
    "100"  :  9.014,
    "1000" : 11.593,
    "10000": 35.969
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    :  8.821,
    "1"    :  8.785,
    "10"   :  8.878,
    "100"  :  9.217,
    "1000" : 12.444,
    "10000": 47.368
  }
}
```

With TLS:

```Json
{
  "https://x.com?a=2": {
    "0"    : 24.468,
    "1"    : 24.588,
    "10"   : 24.746,
    "100"  : 24.463,
    "1000" : 27.088,
    "10000": 47.041
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    : 24.658,
    "1"    : 24.470,
    "10"   : 24.492,
    "100"  : 24.571,
    "1000" : 28.164,
    "10000": 56.847
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    : 24.648,
    "1"    : 24.311,
    "10"   : 24.841,
    "100"  : 24.989,
    "1000" : 29.138,
    "10000": 71.758
  }
}
```
