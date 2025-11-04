# URL Cleaner CLI

[![Crates.io Version](https://img.shields.io/crates/v/url-cleaner)](https://crates.io/crates/url-cleaner/)

[Documentation for URL Cleaner in general](../README.md)

The CLI interface for URL Cleaner.

Licensed under the Affero General Public License V3 or later.

https://www.gnu.org/licenses/agpl-3.0.html

## Performance

On a mostly stock lenovo thinkpad T460S (Intel i5-6300U (4) @ 3.000GHz) running Kubuntu 25.04 (kernel 6.14.0) that has "not much" going on (FireFox, Steam, etc. are closed), hyperfine gives me the following benchmark:

Last updated 2025-11-03.

Also the numbers are in milliseconds.

```Json
{
  "https://x.com?a=2": {
    "0"    :  5.989,
    "1"    :  6.188,
    "10"   :  6.210,
    "100"  :  6.324,
    "1000" :  8.225,
    "10000": 25.109
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    :  5.959,
    "1"    :  6.075,
    "10"   :  6.148,
    "100"  :  6.402,
    "1000" :  8.975,
    "10000": 31.778
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    :  6.008,
    "1"    :  6.137,
    "10"   :  6.236,
    "100"  :  6.438,
    "1000" :  8.923,
    "10000": 34.542
  }
}
```

### Memory usage

Because cleaning enough URLs to reach max memory usage when ussing massif takes a LONG time, expect these numbers to be updated much less frequently.

**Please note that URL Cleaner CLI uses a buffered output when it detects it's outputting to a program or file. That improves performance and dramatically improves memory usage.**

**These numbers use that optimization.**

Though if you're printing 10 million URLs of output to your terminal you're probably doing it by accident and therefore don't really care about the 40-ish meagbytes it took before the buffering.

Also to turn off the buffering you can use `--unbuffer-output`.

The format is `(number of the URL): (max memory usage in bytes)`.

Updated 2025-10-14

```
Massif
 https://x.com?a=2
  0: 614,479
  1: 614,479
  10: 614,479
  100: 614,479
  1000: 737,647
  10000: 2,330,884
  100000: 5,787,300
  1000000: 9,927,374
  10000000: 12,370,646
 https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id
  0: 614,479
  1: 614,479
  10: 614,479
  100: 614,479
  1000: 884,168
  10000: 2,083,448
  100000: 4,292,294
  1000000: 10,756,438
  10000000: 5,887,986
 https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8
  0: 614,479
  1: 614,479
  10: 614,479
  100: 614,479
  1000: 761,637
  10000: 828,922
  100000: 793,266
  1000000: 884,616
  10000000: 1,063,494
```

Assuming you store 10 million URLs to give to URL Cleaner (in a real situation, for this I jused used the `yes` command), you probably have the 1 to 15 megabytes of RAM needed to clean all of them.

And no I don't know what's going on with 1 million of the example.com URL taking twice the memory as 10 million of the same URL.

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
