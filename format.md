# Formatting

For simplicity, CLI, Site, and Site CLIent use a single format for task and result streams.

Where applicable, other frontends should try to mimic this format.

## Behavior

A frontend or client to a frontend will never require additional lines of inputs to be sent or that the input stream be closed before returning all corresponding result lines.

## Lines

Both `\r\n` and `\n` are valid line separators.

The input `a\nb\r\nc` is said to contain the lines `a`, `b`, and `c`.

Whether or not the empty string contains zero lines or one empty line is avoided by ignoring empty lines.

Similarly, whether or not a trailing line separator counts as a trailing empty line is avoided by ignoring empty lines.

## Chunks

A stream of chunks is a stream of each chunk's lines, as defined above.

Chunks `a\nb` and `c\r\nd` are said to contain lines `a`, `b`, `c`, and `d`.

Whether or not the empty chunk contains zero lines or one empty line is avoided by ignoring empty lines.

Similarly, whether or not a trailing line separator counts as a trailing empty line is avoided by ignoring empty lines.

## Input

For each line:

- If it is empty, it is ignored.

- If it starts with an ASCII letter, `{`, or `"`, it is a task:

  - If it starts with an ASCII letter it is parsed as a URL.

  - If it starts with `{` it parsed as a JSON encoded task.

  - If it starts with `"` it parsed as a JSON encoded task.

- Otherwise, it is invalid and the corresponding line of output is an error.

## Output

There are two flags, "brief unchanged" and "brief error", that affect the formatting of the output.

Frontends should default both to disable and give clients the option to enable either or both on a per-job basis.

For each line:

- If it is empty, it is ignored.

- If it starts with an ASCII letter, it is a success result containing its task's cleaned URL.

  - If "brief unchanged" is enabled and it starts with `=`, it is a success result whose input was already clean.

    - Please note that this only checks if the value ever changed during cleaning. Some inputs may give unchanged outputs that aren't caught by this.

  - If "brief unchanged" is disabled, it will never start with `=`.

- If it starts with `-`, it is an error result.

  - If "brief error" is enabled, the rest of the line is empty.

  - If "brief error" is disabled, the rest of the line is a non-empty string detailing the error.

- Otherwise, it is invalid and clients are suggested to ignore it but may do whatever they want.

Frontends should, but are not required to:

- Explicitly state if and when any of the following or other additional guarantees are followed.

- Never output an empty line unless it is the last line.

- Never output an empty chunk.

- Prefer returning lines separated by `\n` rather than `\r\n`.
