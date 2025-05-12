# URL Cleaner

[![docs.rs](https://img.shields.io/docsrs/url-cleaner)](https://docs.rs/url-cleaner/latest/url_cleaner/)
[![Crates.io Version](https://img.shields.io/crates/v/url-cleaner)](https://crates.io/crates/url-cleaner/)

The CLI interface for URL Cleaner.

## Default cleaner

See [`../default_cleaner.md`](../default_cleaner.md) for details about the included default cleaner.

## Performance

On a mostly stock lenovo thinkpad T460S (Intel i5-6300U (4) @ 3.000GHz) running Kubuntu 24.10 (kernel 6.14.0) that has "not much" going on (FireFox, Steam, etc. are closed), hyperfine gives me the following benchmark:

Last updated 2025-05-06.

Also the numbers are in milliseconds.

```Json
{
  "https://x.com?a=2": {
    "0"    :  6.142,
    "1"    :  6.199,
    "10"   :  6.315,
    "100"  :  6.535,
    "1000" :  9.192,
    "10000": 31.578
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    :  6.136,
    "1"    :  6.236,
    "10"   :  6.292,
    "100"  :  6.815,
    "1000" : 11.300,
    "10000": 52.947
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    :  6.240,
    "1"    :  6.223,
    "10"   :  6.331,
    "100"  :  6.934,
    "1000" : 11.573,
    "10000": 54.185
  }
}
```

For reasons not yet known to me, everything from an Intel i5-8500 (6) @ 4.100GHz to an AMD Ryzen 9 7950X3D (32) @ 5.759GHz seems to max out at between 140 and 110ms per 100k (not a typo) of the amazon URL despite the second CPU being significantly more powerful.

## Parsing output

Note: [JSON output is supported](#json-output).

Unless a `Debug` variant is used, the following should always be true:

1. Input URLs are a list of URLs starting with URLs provided as command line arguments then each line of the STDIN.
2. The nth line of STDOUT corresponds to the nth input URL.
3. If the nth line of STDOUT is empty, then something about reading/parsing/cleaning the URL failed.
4. The nth non-empty line of STDERR corresponds to the nth empty line of STDOUT.
    1. Currently empty STDERR lines are not printed when a URL succeeds. While it would make parsing the output easier it would cause visual clutter on terminals. While this will likely never change by default, parsers should be sure to follow 4 strictly in case this is added as an option.

### JSON output

The `--json`/`-j` flag can be used to have URL Cleaner output JSON instead of lines.

The format looks like this, but minified:

```Json
{"Ok": {
  "urls": [
    {"Ok": "https://example.com/success"},
    {"Err": "https://example.com/failure"}
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
