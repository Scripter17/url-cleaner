# URL Cleaner Site API

A basic overview of the API of URL Cleaner Site's API.

For a more detailed understanding of the types used, see the crate documentations for [URL Cleaner Engine](../engine) and [URL Cleaner Site Types](../site-types).

For a typed API you can make clients with, see [URL Cleaner Site Types](../site-types).

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

Either a WebSocket or HTTP POST/PUT.

- The `CleanConfig` is sent in the `config` query parameter XOR the `X-Config` header.

- The request and response messages/frames are streamed in parallel.

- WebSocket messages use explicit chunking.

- HTTP frames use implicit chunking.
