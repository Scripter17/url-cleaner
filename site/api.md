# URL Cleaner Site API

A basic overview of the API of URL Cleaner Site's API.

For a CLI client for Site, see [URL Cleaner Site CLIent](../site-client).

## `/info`

A GET endpoint that returns the following information as JSON.

```Rust
/// Info about the instance.
struct Info {
    /** The version.                       **/ version          : &'static str,
    /** The link to the source code.       **/ source_code      : &'static str,
    /** If `/clean` requires a password.   **/ requires_password: bool,
    /** If the `http` feature is enabled.  **/ supports_http    : bool,
    /** If the `cache` feature is enabled. **/ supports_cache   : bool,
}
```

## `/cleaner`

A GET endpoint that returns the loaded `Cleaner`.

## `/profiles`

A GET endpoint that returns the loaded `ProfilesConfig`.

## `/userscript`

A GET endpoint that returns a copy of URL Cleaner Site Userscript with instance info pre-filled using the request's `Host` header.

## `/clean`

Either a WebSocket or HTTP POST/PUT duplex.

- The `JobConfig` is sent as JSON in the `config` query parameter XOR the `X-Config` header.

- Providing a task line, waiting for its result line, and only then providing another task line will never deadlock.

The JobConfig format:

```Rust
pub struct JobConfig {
    /// The password to use.
    ///
    /// Defaults to [`None`].
    pub password: Option<String>,
    /// The [`JobContext`] to use.
    ///
    /// Defaulted.
    pub context: JobContext,
    /// The profile to use.
    ///
    /// Defaults to [`None`].
    pub profile: Option<String>,
    /// The [`ParamsDiff`] to use on top of the profile.
    ///
    /// Defaulted.
    pub params_diff: ParamsDiff,
    /// If [`true`], unchanged lines are replaced with `=`.
    ///
    /// Defaults to false.
    pub brief_unchanged: bool,
    /// If [`true`], error lines are replaced with `-`.
    ///
    /// Defaults to false.
    pub brief_error: bool,
    /// If [`true`], hide the threads.
    ///
    /// Defaults to [`false`].
    pub hide_threads: bool,
    /// If [`false`], disable the HTTP Client.
    ///
    /// Defaults to [`true`].
    pub http: bool,
    /// If [`false`], disable reading from the cache.
    ///
    /// Defaults to [`true`].
    pub read_cache: bool,
    /// If [`false`], disable writing to the cache.
    ///
    /// Defaults to [`true`].
    pub write_cache: bool,
    /// If [`true`], hide the cache.
    ///
    /// Defaults to [`false`].
    pub hide_cache: bool,
}
```

### WebSocket

- Task and result messages contain only full lines.

- Task messages can be either binary or text.

- All result lines will be returned before responding to a close frame.

    - For annoying performance reasons, this means task messages should each contain multiple task lines.

- Result messages are text.

- Result messages contain only result lines.

- Each `\n` is succeeded by a result line.

- Each `\n` is preceeded by a result line.

    - Consequently, result messages have no empty lines.

        - Consequently, there are no empty result messages.

### HTTP

- HTTP frames may or may not split lines arbitrarily.

- Each result line is succeeded by a `\n`.

- Each `\n` is preceeded by a result line.
