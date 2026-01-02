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
    pub max_payload: u64,
    /// Whether or not you need a password to clean URLs.
    pub password_required: bool
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

- The `CleanConfig` is sent in the `config` query parameter XOR the `X-Config` header.

- The response body is a stream of result lines if and only if the response status is 200.

- The request and response are streamed in parallel with their bodies using implicit chunking.

### `/clean_ws`

A WebSocket endpoint.

- The `CleanConfig` is sent in the `config` query parameter.

 - WebSockets does not formally support custom request headers. Setting `X-Config` anyway *may* work but may break at any time.

- The request and response messages are streamed in parallel using explicit chunking.
