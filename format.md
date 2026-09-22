# Formatting

For simplicity, CLI, Site, and Site CLIent use a single format for task and result streams.

Where applicable, other frontends should try to mimic this format.

## Lines

Both `\r\n` and `\n` are valid line separators.

The input `a\nb\r\nc` is said to contain the lines `a`, `b`, and `c`.

Whether or not the empty string contains zero lines or one empty line is avoided by ignoring empty lines.

## Chunks

A stream of chunks is a stream of each chunk's lines, as defined above.

Chunks `a\nb` and `c\r\nd` are said to contain lines `a`, `b`, `c`, and `d`.

Whether or not the empty chunk contains zero lines or one empty line is avoided by ignoring empty lines.

## Input

For each line:

- If it is empty, it is ignored.

- If it starts with an ASCII letter, `{`, or `"`, it is a task:

  - If it starts with an ASCII letter it is parsed as a URL.

  - If it starts with `{` it parsed as a JSON encoded task.

  - If it starts with `"` it parsed as a JSON encoded line.

- Otherwise, it is invalid and the corresponding line of output is an error.

## Output

There are two flags, "brief unchanged" and "brief error", that affect the formatting of the output.

When and how each flag is enabled is defined purely by the frontend, though it is suggested that frontends default both to disabled and provide ways for clients to enable either or both on a per-job basis.

For each line:

- If it is empty, it is ignored.

- If it starts with an ASCII letter, it is a success result containing its task's cleaned URL.

- If "brief unchanged" is enabled and it starts with `=`, it is a success result whose input was already clean.

- If "brief unchanged" is disabled, it will never start with `=`.

- If it starts with `-`, it is an error result.

  - If "brief error" is enabeld, the rest of the line is empty.

  - If "brief error" is disabled, the rest of the line is a non-empty string detailing the error.

Frontends should, but are not required to:

- Explicitly state if and when any of the following additional guarantees are followed.

- Never output an empty line followed by a non-empty line.

- Never output an empty chunk.

- Prefer returning lines separated by `\n` rather than `\r\n`.
