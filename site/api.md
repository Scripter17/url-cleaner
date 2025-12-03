# URL Cleaner Site API

A basic overview of the API of URL Cleaner Site's API.

For a more detailed understanding of the types used, see the crate documentations for [URL Cleaner Engine](../engine) and [URL Cleaner Site Types](../site-types).

For a typed API you can make clients with, see [URL Cleaner Site Types](../site-types).

## `/info`

A GET endpoint that returns the following information as JSON.

<!--cmd echo '```Rust'; cat site-types/src/info.rs | grep -vP '^\s*#' | grep -oPz '\n(///.+\n)+pub [\s\S]+?\}\n' | tail -n +2 | sed 's/\x00//g'; echo '```'-->
```Rust
/// Info about a URL Cleaner Site server.
pub struct ServerInfo {
    /// The link to the source code.
    pub source_code: String,
    /// The version.
    pub version: String,
    /// The max payload size.
    pub max_payload: u64
}
```
<!--/cmd-->

## `/cleaner`

A GET endpoint that returns the loaded `Cleaner` from before any profiles were applied.

## `/profiles`

A GET endpoint that returns the loaded `ProfilesConfig`.

## Cleaning

### `/clean`

A POST endpoint.

- The `CleanConfig` is sent either in the `config` query parameter XOR the `X-Config` header.

- The body of the request and response (if successful) follow [the standard format](../format.md) with no additional guarantees.

- If processing the request failed, a JSON encoded `CleanError` is returned instead.

The maximum size of a body is set with the `--max-payload` CLI argument and exposed in the [`/info`](#info) endpoint under the `max_payload` field.

By default this value is 25MiB.

### `/clean_ws`

A WebSocket endpoint.

- The `CleanConfig` is sent in the `config` query parameter. Unfortunately WebSocket doesn't support custom headers.

- The body of each message follow [the standard format](../format.md) with no additional guarantees.

### Types

<!--cmd echo '```Rust'; cat site-types/src/clean.rs | grep -vP '^\s*#' | grep -oPz '\n(///.+\n)+pub [\s\S]+?\}\n' | tail -n +2 | sed 's/\x00//g'; echo '```'-->
```Rust
/// The error state of doing a [`JobConfig`].
pub struct CleanError {
    /// The HTTP status code.
    pub status: u16,
    /// The error message.
    pub message: String
}

/// Given as JSON text in either the `config` query parameter XOR the `X-Config` header.
pub struct JobConfig {
    /// The [`JobContext`] to use.
    ///
    /// Defaults to [`JobContext::default`].
    pub context: JobContext,
    /// The [`Profile`] to use.
    ///
    /// Applied before [`Self::params_diff`].
    ///
    /// Defaults to [`None`].
    pub profile: Option<String>,
    /// The [`ParamsDiff`] to use.
    ///
    /// Applied after [`Self::profile`].
    ///
    /// Defaults to [`None`].
    pub params_diff: Option<ParamsDiff>,
    /// If [`true`], enable reading from the cache.
    ///
    /// Exists unconditionally even when the URL Cleaner Site has the `cache` feature disabled.
    ///
    /// It's just easier like this.
    ///
    /// Defaults to [`true`].
    pub read_cache: bool,
    /// If [`true`], enable writing to the cache.
    ///
    /// Exists unconditionally even when the URL Cleaner Site has the `cache` feature disabled.
    ///
    /// It's just easier like this.
    ///
    /// Defaults to [`true`].
    pub write_cache: bool,
    /// If [`true`], enable cache delays.
    ///
    /// Exists unconditionally even when the URL Cleaner Site has the `cache` feature disabled.
    ///
    /// It's just easier like this.
    ///
    /// Defaults to [`false`].
    pub cache_delay: bool,
    /// If [`true`], enable unhtreading.
    ///
    /// Defaults to [`false`].
    pub unthread: bool
}
```
<!--/cmd-->
