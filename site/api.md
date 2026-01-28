# URL Cleaner Site API

A basic overview of the API of URL Cleaner Site's API.

For a CLI client for Site, see [URL Cleaner Site CLIent](../site-client).

## `/get-info`

A GET endpoint that returns the following information as JSON.

```Rust
/// Info about a URL Cleaner Site server.
pub struct Info {
    /// The link to the source code.
    pub source_code: String,
    /// The version.
    pub version: String,
    /// Whether or not you need a password to clean URLs.
    pub password_required: bool
}
```

## `/get-cleaner`

A GET endpoint that returns the loaded `Cleaner`.

## `/get-profiles`

A GET endpoint that returns the loaded `ProfilesConfig`.

## `/clean`

Either a WebSocket or HTTP POST/PUT duplex.

- The `JobConfig` is sent in the `config` query parameter XOR the `X-Config` header.

### WebSocket

- Task messages can be either binary or text.

- For complex performance reasons, each task message should contain multiple tasks.

- Result messages are text.

- There are no empty result messages.

- Result messages contain no empty lines.

- Result messages use only `\n` as a line separator.

### HTTP

- Each result line is succeeded by a `\n`.

- Each `\n` is preceeded by a result line.
