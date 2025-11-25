# Formatting

For simplicity, URL Cleaner CLI and URL Cleaner Site's HTTP and WebSocket frontends all use (effectively) identical formats for sending tasks and receiving results.

Where applicable, other frontends should try to mimic this format.

## Sending tasks

Tasks are sent as lines of `LazyTaskConfig::Str` or `LazyTaskConfig::ByteSlice`.

For each line:

- If the line starts with `{` or `"`, parse the line as JSON.
- Otherwise, parse the line as a URL.

Lines are determined according to Rust's [`str::lines`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.lines).

## Receiving results

Results are returned as lines of strings.

For each line:

- If the line starts with `-`, its corresponding task returned an error. The rest of the line is a string of unspecified format about the exact error returned.
- Otherwise, the line is the cleaned URL.

Every result, even the last one, is succeeded by a lone `\n`.

## Examples

Tasks:

```
https://example.com?utm_source=docs
notaurl
https://example.com?utm_source=docs
```

Results:
```
https://example.com/
-MakeTaskError(UrlParseError(RelativeUrlWithoutBase))
https://example.com/
```
