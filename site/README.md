# URL Cleaner Site

[![Crates.io Version](https://img.shields.io/crates/v/url-cleaner-site)](https://crates.io/crates/url-cleaner-site/)

[Documentation for URL Cleaner in general](../README.md)

An HTTP/HTTPS/WS/WSS server to allow using URL Cleaner remotely, such as via a userscript in a web browser.

Licensed under the Affero General Public License V3 or later.

https://www.gnu.org/licenses/agpl-3.0.html



# Usage

By default, Site will listen on IP `127.0.0.1` at port `9149`.

To allow other computers on your local network to use your instance of Site, run Site with `--ip 0.0.0.0`.

## TLS

See the output of `url-cleaner-site keygen --help` for a basic guide on generating and using TLS.

### Installing the certificate

#### Linux

```Bash
sudo cp urlcs-tls-things/urlcs-ca.crt /usr/local/share/ca-certificates/
sudo update-ca-certificates
```

#### Firefox

On my machine, Firefox seems to ignore the local CA store. In this case:

1. Open `https://localhost:9149`, or whatever URL is given to you when URL Cleaner Site starts.

2. Click "Advanced...".

3. Click 'Accept the Risk and Continue".

4. Remember that this only trusts `urlcs.crt` for specifically `localhost` (or whatever host was given to you by URL Cleaner Site).

#### Ios

1. Get the `urlcs-ca.crt` file onto your iphone and open it such that you get a popup with "Profile Downloaded".

2. Open settings then tap the "Profile Downloaded" button.

3. Tap "URL Cleaner Site" under "Downloaded Profile".

4. Tap "Install" in the top right, authenticate, tap "Install" in the top right, tap "Install" at the bottom, then tap "Done".

5. Go back one level (just under "General"), scroll to the top, tap "About", scroll to the bottom, tap "Certificate Trust Settings", find "URL Cleaner Site Certificate Authority" under "Enable Full Trust For Root Certificate", then enable it.



# Userscript

URL Cleaner Site Userscript lets you automatically clean every link on every webpage you visit.

First you should install a userscript extension such as
[Greasemonkey] (for [Firefox][gm-f]),
[Tampermonkey] (for [chrome][tm-c], [edge][tm-e], [Firefox][tm-f], [safari][tm-s], and [opera][tm-o]),
or [Userscripts] (for [safari][us-s]).

Next, to get Site Userscript, you can either:

- Use Site's `userscript` subcommand.

- GET the `/userscript` endpoint of your Site instance.

  - Can be done with Site CLIent using the `get` subcommand.

Afterwards, add the userscript to your userscript manager.

## Known problems

### Ios

Because apple doesn't know how to do anything, [Userscripts] is unable to connect to HTTP instances of URL Cleaner Site on HTTPS websites.

The solution is to make URL Cleaner Site use TLS. Sorry.

### Privacy

In addition to the normal privacy concerns detailed [here](../README.md#privacy):

- It's basically unavoidable that websites will be able to tell that you're using URL Cleaner Site Userscript and, most likely, which specific version and profile/ParamsDiff you're using.

- URL Cleaner has two features that are usually disabled but are specifically enabled by Site Userscript.

  - Thread hiding forces Jobs to not do multiple redirect chains at the same time.

    - This prevents websites from knowing how many worker threads your Site instance uses per job.

  - Cache hiding makes reading from the cache take at about as long as the initial operation.

    - This prevents websites from knowing you've seen certain redirect URLs before.



# API

## `/info`

A GET endpoint that returns the following information as JSON.

```Rust
struct Info {
    /** The version.                       */ version          : String,
    /** The link to the source code.       */ source_code      : String,
    /** If `/clean` requires a password.   */ requires_password: bool,
    /** If the `http` feature is enabled.  */ supports_http    : bool,
    /** If the `cache` feature is enabled. */ supports_cache   : bool,
}
```

## `/cleaner`

A GET endpoint that returns the loaded `Cleaner` before any profiles are applied.

## `/profiles`

A GET endpoint that returns the loaded `ProfilesConfig`.

## `/userscript`

A GET endpoint that returns a copy of URL Cleaner Site Userscript with instance info pre-filled.

## `/clean`

Either an HTTP POST/PUT duplex or a WebSocket stream.

The `JobConfig`, defined below, is sent as JSON in either the `job_config` query parameter XOR the `X-Job-Config` header.

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
    /// Defaults to [`false`].
    pub brief_unchanged: bool,
    /// If [`true`], error lines are replaced with `-`.
    ///
    /// Defaults to [`false`].
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

Follows the [standard format](../format.md)'s chunks mode with the following additional guarantees:

- Task messages can be either binary or text.

- Result messages are text.

- Result lines are delimited with `\n`.

- Result messages are never empty.

### HTTP

Follows the [standard format](../format.md)'s lines mode with the following additional guarantees:

- Every result line is succeeded by a `\n`.

- Every `\n` is preceded by a result line.

[Greasemonkey]: https://www.greasespot.net/
[gm-f]: https://addons.mozilla.org/en-US/firefox/addon/greasemonkey/

[Tampermonkey]: https://www.tampermonkey.net/
[tm-c]: https://chromewebstore.google.com/detail/dhdgffkkebhmkfjojejmpbldmpobfkfo
[tm-e]: https://microsoftedge.microsoft.com/addons/detail/iikmkjmpaadaobahmlepeloendndfphd
[tm-f]: https://addons.mozilla.org/en-US/firefox/addon/tampermonkey/
[tm-s]: https://apps.apple.com/app/tampermonkey/id6738342400
[tm-o]: https://addons.opera.com/en/extensions/details/tampermonkey-beta/

[Userscripts]: https://github.com/quoid/userscripts
[us-s]: https://apps.apple.com/us/app/userscripts/id1463298887
