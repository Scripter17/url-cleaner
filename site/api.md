# URL Cleaner Site API

A basic overview of the API of URL Cleaner Site's API.

For a CLI client for Site, see [URL Cleaner Site CLIent](../site-client).

## `/info`

A GET endpoint that returns the following information as JSON.

```Rust
/// Info about a URL Cleaner Site server.
pub struct Info {
    /// The link to the source code.
    pub source_code: String,
    /// The version.
    pub version: String,
    /// The [`AuthMode`].
    pub auth_mode: AuthMode,
}

/// The type of [`AuthInfo`] being used in a format that can be sent to users.
pub enum AuthMode {
    /// [`AuthInfo::None`].
    None,
    /// [`AuthInfo::Password`].
    Password,
    /// [`AuthInfo::Userinfo`].
    Userinfo,
}
```

## `/cleaner`

A GET endpoint that returns the loaded `Cleaner`.

## `/profiles`

A GET endpoint that returns the loaded `ProfilesConfig`.

## `/clean`

Either a WebSocket or HTTP POST/PUT duplex.

- The `JobConfig` is sent in the `config` query parameter XOR the `X-Config` header.

## `/userscript`

A GET endpoint that returns a copy of URL Cleaner Site Userscript with instance info pre-filled using the request's `Host` header.

### WebSocket

- Task and result messages contain only full lines.

- Task messages can be either binary or text.

- For performance reasons, task messages should each contain multiple task lines.

- Result messages are text.

- Result messages contain only result lines.

- Every result line, except the last, is succeeded by a `\n`.

- Every `\n` is preceeded by a result line.

- Consequently, result messages have no empty lines.

- Consequently, there are no empty result messages.

- Providing a task line, waiting for its result line, then providing another task line will never deadlock.

### HTTP

- HTTP frames may or may not split lines arbitrarily.

- Each result line is succeeded by a `\n`.

- Each `\n` is preceeded by a result line.

- Consequently, the result stream has no empty lines.

- Providing a task line, waiting for its result line, then providing another task line will never deadlock.
