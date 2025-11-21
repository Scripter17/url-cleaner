# URL Cleaner Site API

A basic overview of the API of URL Cleaner Site's API.

For a more detailed understanding of the types used, see the crate documentations for [URL Cleaner Engine](../engine) and [URL Cleaner Site Types](../site-types).

For a typed API you can make clients with, see [URL Cleaner Site Types](../site-types).

## `/info`

A GET endpoint that returns the following information as JSON.

<!--cmd echo '```Rust'; cat site-types/src/info.rs | grep -vP '^\s*#' | grep -oPz '\n(///.+\n)+pub [\s\S]+?\}\n' | tail -n +2 | sed 's/\x00//g'; echo '```'-->
```Rust
/// Info about a URL Cleaner Site server.
pub struct ServerInfo<'a> {
    /// The link to the source code.
    pub source_code: Cow<'a, BetterUrl>,
    /// The version.
    pub version: Cow<'a, str>,
    /// The max payload size.
    pub max_payload: u64,
    /// The [`UnthreaderMode`] used when unthreading.
    pub unthreader_mode: UnthreaderMode
}
```
<!--/cmd-->

## `/cleaner`

A GET endpoint that returns the loaded `Cleaner` from before any profiles were applied.

## `/profiles`

A GET endpoint that returns the loaded `ProfilesConfig`.

## `/clean`

A POST endpoint that takes a `CleanPayload` and returns a `SmallCleanResult`, both as JSON.

`SmallCleanResult` and `CleanResult` serialize and deserialize identically.

Generally `SmallLazyTaskConfig`s are just strings.

See [`/clean_ws`](#clean_ws) for a WebSocket API.

<!--cmd echo '```Rust'; cat site-types/src/clean.rs | grep -vP '^\s*#' | grep -oPz '\n(///.+\n)+pub [\s\S]+?\}\n' | tail -n +2 | sed 's/\x00//g'; echo '```'-->
```Rust
/// Used to construct a [`Job`].
pub struct CleanPayload<'a> {
    /// The [`LazyTaskConfig`]s to use.
    pub tasks: Vec<SmallLazyTaskConfig<'a>>,
    /// The [`CleanPayloadConfig`] with `#[serde(flatten)]` applied.
    pub config: CleanPayloadConfig
}

/// [`CleanResult`] but small.
pub type SmallCleanResult = Result<SmallCleanSuccess, CleanError>;

/// [`CleanSuccess`] but small.
pub struct SmallCleanSuccess {
    /// The [`Task`] results.
    pub urls: Vec<Result<String, String>>
}

/// The success state of doing a [`JobConfig`].
pub struct CleanSuccess {
    /// The [`Task`] results.
    pub urls: Vec<Result<BetterUrl, String>>
}

/// The error state of doing a [`JobConfig`].
pub struct CleanError {
    /// The HTTP status code.
    pub status: u16,
    /// The error message.
    pub message: String
}

/// When used in `/clean_ws`, each field is sent as a query parameter with JSON values.
pub struct CleanPayloadConfig {
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

Authentication is sent as a username and password in the URL like `http://username:password@127.0.0.1/clean`.

This is the same for both [`/clean`](#clean) and [`/clean_ws`](#clean_ws).

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

## `/clean_ws`

Like [`/clean`](#clean) but uses WebSockets.

The `CleanPayloadConfig` is set with a query parameter for each field either omitted for the default value or set to a JSON string.

Tasks are sent as strings with any number of lines. Each line contains one task.

Results are returned in the same order they are recieved as strings of lines, each containing `Ok` or `Err`, a tab, then the result payload. Every line, even the last one, is followed by a newline.

Only the order in which result lines are sent is defined. A group of 3 tasks may be returned as a group of 2 results and a group of 1 result.

Users should be careful to only close the connection once either all results are recieved or once further results are no longer needed.
