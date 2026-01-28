# Formatting

For simplicity, CLI, Site, and Site CLIent use a single format for task and result streams.

Where applicable, other frontends should try to mimic this format.

## Lines

Both `\r\n` and `\n` are valid line separators.

## Chunks

A stream of chunks is a stream of each chunk's lines, as defined above.

## Input

For each line:

- If it is empty, it is ignored.

- If it starts with an ASCII letter, `{`, or `"`, it is a task:

  - If it starts with an ASCII letter it is parsed as a URL.

  - If it starts with `{` it parsed as a JSON encoded task.

  - If it starts with `"` it parsed as a JSON encoded line.

- Otherwise, it is invalid and the corresponding line of output is an error.

## Output

For each line:

- If it is empty, it is ignored.

- If it begins with an ASCII letter, it is a success result containing its task's cleaned URL.

- If it begins with `-`, it is an error result and the rest of the line is a (possibly empty) error message of unspecified format.
