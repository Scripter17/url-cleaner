# Formatting

For simplicity, URL Cleaner CLI and URL Cleaner Site's HTTP and WebSocket frontends all use (effectively) identical formats for sending tasks and receiving results.

Where applicable, other frontends should try to mimic this format.

## Lines

Lines are defined per Rust's [`str::lines`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.lines) with the exception that the treatment of carriage returns not succeeded by newlines is undefined.

For example, while `1\r\n2` contains the lines `1` and `2`, `1\r\n2\r` may contain the lines `1` and `2` or the lines `1` and `2\r`.

`1\r\n2\n`, while the line delimination is inconsistent, always contains the lines `1` and `2`.

``, the empty string, is treated as zero lines. This is a pain to ensure in languages that disagree but it is important.

## Tasks

Tasks are given to URL Cleaner frontends as a series of of lines ("task lines").

For how lines are processed into `TaskConfig`s, see `LazyTaskConfig::Str`/`LazyTaskConfig::ByteSlice`.

Of note, lines starting with `{` or `"` are treated as JSON with no need for frontends to do manual conversion.

If a JSON encoded line would contain only a URL, clients should try to send it as such.
For example both `{"url":"https://example.com"}` and `"https://example.com"` should be sent as just `https://example.com`.
JSON encoding and decoding is expensive and should be reserved only for when a `TaskContext` contains stuff.

Frontends may allow otherwise invalid task lines, such as those starting with `/`, `!`, etc. for any purpose such as commands.

If practical, frontends should not require clients to provide custom lines.

If practical, frontends should ignore unknown custom lines.

## Results

Results are returned as a series of lines ("result lines") returned in the same order as their corresponding tasks.

For example, a tasks A then B will never be returned as B then A.

If the stream ends in either `\n` or `\r\n`, treat it as if it didn't. For example, `1\n2`, `1\n2\n`, and `1\n2\r\n` are equivalent.

If a line starts with a `-` then it represents an error. The rest of the line is a string of unspecified format that describes the error.

If a line starts with an ASCII letter then it represents a success and is a valid URL.

Frontends may allow otherwise invalid result lines, such as those starting with `/`, `!`, etc. for any purpose such as debugging info or command results.

If practical, frontends should not require clients to handle custom lines.

If practical, clients should ignore unknown custom lines.

## Streaming

For APIs that allow streaming tasks and results, each individual message follows the above format.

No effort to have any concattenation of consecutive messages be a valid result stream is required.

For example, the messages `1` and `2` are equivalent to the messages `1` and `2\n`, `1\n` and `2`, and `1\n` and `2\n`.

The number of lines in a result message is not defined to correlate at all with the numbers of lines in task messages.

For example, separate task messages `1`, `2`, and `3` may be returned as `1\n2\n3`, `1` and `2\n3`, `1\n2` and `3`, `1`, `2`, and `3`, or any other combination.

The order of lines is still defined to be first in first out. `2\n1\n3` is an invalid response to `1`, `2`, and `3`.

Individual frontends may provide stricter guarantees but clients should carefully consider whether relying on those guarantees is a good idea (it usually isn't).
