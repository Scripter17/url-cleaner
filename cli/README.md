# URL Cleaner

[![Crates.io Version](https://img.shields.io/crates/v/url-cleaner)](https://crates.io/crates/url-cleaner/)

The CLI interface for URL Cleaner.

Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)

https://www.gnu.org/licenses/agpl-3.0.html

## Default cleaner

See [`../default_cleaner.md`](../default_cleaner.md) for details about the included default cleaner.

## Performance

On a mostly stock lenovo thinkpad T460S (Intel i5-6300U (4) @ 3.000GHz) running Kubuntu 25.04 (kernel 6.14.0) that has "not much" going on (FireFox, Steam, etc. are closed), hyperfine gives me the following benchmark:

Last updated 2025-08-11.

Also the numbers are in milliseconds.

```Json
{
  "https://x.com?a=2": {
    "0"    :  6.095,
    "1"    :  6.292,
    "10"   :  6.370,
    "100"  :  6.621,
    "1000" :  9.066,
    "10000": 29.312
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    :  6.124,
    "1"    :  6.307,
    "10"   :  6.408,
    "100"  :  6.648,
    "1000" : 10.195,
    "10000": 40.391
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    :  6.154,
    "1"    :  6.276,
    "10"   :  6.480,
    "100"  :  6.801,
    "1000" : 10.625,
    "10000": 43.358
  }
}
```

For reasons not yet known to me, everything from an Intel i5-8500 (6) @ 4.100GHz to an AMD Ryzen 9 7950X3D (32) @ 5.759GHz seems to max out at between 140 and 110ms per 100k (not a typo) of the amazon URL despite the second CPU being significantly more powerful.

## Parsing output

Note: [JSON output is supported](#json-output).

Unless a `Debug` variant is used, the following should always be true:

1. Input URLs are a list of URLs starting with URLs provided as command line arguments then, if applicable, each line of the STDIN.
2. The nth line of STDOUT corresponds to the nth input URL.
3. If the nth line of STDOUT is empty, then something about reading/parsing/cleaning the URL failed.
4. The nth non-empty line of STDERR corresponds to the nth empty line of STDOUT.

### JSON output

The `--json`/`-j` flag can be used to have URL Cleaner output JSON instead of lines.

The format looks like this, but minified:

```Json
{"Ok": {
  "urls": [
    {"Ok": "https://example.com/success"},
    {"Err": "Error message"}
  ]
}}
```

The surrounding `{"Ok": {...}}` is to let URL Cleaner Site return `{"Err": {...}}` on invalid input.

### Exit code

Currently, the exit code is determined by the following rules:

- If no   cleanings work and none fail, returns 0. This only applies if no URLs are provided.
- If no   cleanings work and some fail, returns 1.
- If some cleanings work and none fail, returns 0.
- If some cleanings work and some fail, returns 2. This only applies if multiple URLs are provided.
