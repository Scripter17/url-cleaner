# URL Cleaner Site API

A basic overview of the API of URL Cleaner Site's API.

For a more detailed understanding of the types used, see the crate documentations for [URL Cleaner Engine](../engine) and [URL Cleaner Site Types](../site-types).

For a typed API, see [URL Cleaner Site Types](../site-types).

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

A POST endpoint that takes a `CleanPayload` and returns a `CleanResult`, both as JSON.

Generally `LazyTaskConfig`s are just strings.

<!--cmd echo '```Rust'; cat site-types/src/clean.rs | grep -vP '^\s*#' | grep -oPz '\n(///.+\n)+pub [\s\S]+?\}\n' | tail -n +2 | sed 's/\x00//g'; echo '```'-->
```Rust
/// Used to construct a [`Job`].
pub struct CleanPayload<'a> {
    /// The [`LazyTaskConfig`]s to use.
    pub tasks: Vec<LazyTaskConfig<'a>>,
    /// The authentication to use.
    ///
    /// Defaults to [`None`].
    pub auth: Option<Auth>,
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
    /// Defaults to [`true`].
    pub read_cache: bool,
    /// If [`true`], enable writing to the cache.
    ///
    /// Defaults to [`true`].
    pub write_cache: bool,
    /// If [`true`], enable cache delays.
    ///
    /// Defaults to [`false`].
    pub cache_delay: bool,
    /// If [`true`], enable unhtreading.
    ///
    /// Defaults to [`false`].
    pub unthread: bool
}

/// The [`Result`] returned by the `/clean` route.
pub type CleanResult = Result<CleanSuccess, CleanError>;

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
```
<!--/cmd-->

## Authenticaton

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
pub struct Auth {
    /// The username.
    pub username: String,
    /// The password.
    pub password: String
}
```
<!--/cmd-->
