# Formatting

For simplicity, URL Cleaner CLI and URL Cleaner Site's HTTP and WebSocket frontends all use (effectively) identical formats for sending tasks and receiving results.

Where applicable, other frontends should try to mimic this format.

Frontends may make additional guarantees as desired, but clients should try to not rely on them if practical.

## Lines

A series of UTF-8 bytes is split into lines by `\r\n` and `\n`, whichever comes first, and with no requirement that all lines be separated by the same separator.

For example, `1\r2\r\n3\n\n4\n` becomes the lines `1\r2`, `3`, an empty line, `4`, and an empty line.

Whether or not the empty string is treated as one empty line or zero lines is undefined and made irrelevant by empty lines being ignored.

## Input

Frontends consume a series of [lines](#lines).

For each line:

- If it is empty, it is ignored.

- If it starts with `{`, `"`, or an ASCII letter, it is a task config.

  - If it starts with an ASCII letter it is parsed as a URL.

  - If it starts with `{` it parsed as a JSON encoded `Task` struct.

  - If it starts with `"` it parsed as a JSON encoded task config.

- Otherwise, behavior is frontend-defined.

## Output

Frontends output a series of [lines](#lines).

For each line:

- If it is empty, it is ignored.

- If it starts with `-` or an ASCII letter, it is a result.

  - If it starts with an ASCII letter it is a success result and the whole line is the cleaned URL.

  - If it starts with `-` it is an error result and the rest of the line is a string of unspecified format describing the error.

- Otherwise, meaning is frontend-defined.

Additionally:

- There are exactly as many results as task configs.

- Results are in the exact same order as their tasks.

## Chunking

Some frontends may have an implicit notion of chunking, such as reading from a file.
For these, a stream of chunks is treated as if no chunking were happening.

However some frontends may have an explicit notion of chunking, such as a WebSocket stream.
For these, a stream of chunks is treated as a stream of each chunk's [lines](#lines).

Frontends where the explicitness of chunking is unclear should clarify their assumptions.
