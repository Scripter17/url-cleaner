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

For how tasks and results are formatted, see [this](../format.md).

### `/clean`

An HTTP POST endpoint where a `CleanConfig` is sent as JSON in the `clean` query parameter.

### `/clean_ws`

A WebSocket endpoint where a `CleanConfig` is sent as JSON in the `clean` query parameter.

Tasks are sent as either text or binary messages and their results are returned as text messages.

Each message is treated as a separate stream of lines.

The distribution of result lines in result messages is not guaranteed and should not be relied upon.

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

/// Config for a `/clean` or `/clean_ws` payload.
pub struct CleanConfig {
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

## Authenticaton

Authentication can be setup by giving a JSON `Accounts` file to `--accounts`.

Authentication is sent as a username and password in the URL like `http://username:password@example.com`.

<!--cmd echo '```Rust'; cat site-types/src/auth.rs | grep -vP '^\s*#' | grep -oPz '\n(///.+\n)+pub [\s\S]+?\}\n' | tail -n +2 | sed 's/\x00//g'; echo '```'-->
```Rust
/// Accounts to control who can use a URL Cleaner Site instance.
pub struct Accounts {
    /// If [`true`], allow "guest" users.
    ///
    /// Defaults to [`true`].
    pub allow_guest: bool,
    /// A map of usernames to passwords.
    ///
    /// Defaults to an empty [`HashMap`].
    pub users: HashMap<String, String>
}

/// A username and password.
pub enum Auth {
    /// Guest
    Guest,
    /// User
    User {
        /// The username.
        username: String,
        /// The password.
        password: String
    }
```
<!--/cmd-->
