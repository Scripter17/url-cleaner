# URL Cleaner CLI

[![Crates.io Version](https://img.shields.io/crates/v/url-cleaner)](https://crates.io/crates/url-cleaner/)

[Documentation for URL Cleaner in general](../README.md)

The CLI interface for URL Cleaner.

Licensed under the Affero General Public License V3 or later.

https://www.gnu.org/licenses/agpl-3.0.html

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
