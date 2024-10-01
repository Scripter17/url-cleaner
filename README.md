# URL Cleaner

Websites often put unique identifiers into URLs so that when you send a link to a friend and they open it, the website knows it was you who sent it to them.  
As most people do not understand and therefore cannot consent to this, it is polite to remove the spytext query parameters before sending URLs to people.  
URL Cleaner is an extremely versatile tool designed to make this process as comprehensive, fast, and easy as possible.

## C dependencies

These packages are required on Kubuntu 2024.04 (and probably therefore all Debian based distros.):

- `libssl-dev` for the `http` feature flag.
- `libsqlite3-dev` for the `caching` feature flag.

There are likely plenty more dependencies required that various Linux distros may or may not pre-install.

If you can't compile it I'll try to help you out. And if you can make it work on your own please let me know so I can add to this list.

## Anonymity

In theory, if you're the only one on a website sharing posts from that website without URL trackers, the website could realize that and track you in the same way ("if no trackers then assume sender is Jared".).  
In practice you are very unlikely to be the only one using/sharing cleaned URLs. Search engines generally provide URLs without trackers<sup>[citation needed]</sup>, some people manually remove trackers, and some websites like [vxtwitter.com](https://github.com/dylanpdx/BetterTwitFix) automatically strip URL trackers.

However, for some websites the default config strips more stuff than search engines. In this case anonymity does fall back to many people using URL Cleaner and providing cover for each other.

As with Tor, protests, and anything else where privacy matters, safety comes in numbers.

### Canonicalization

There is a vague notion of "canonical" URLs. One where every URL with its semantics can be converted into the canonical form with no external details.

For example, `https://amazon.com/product-name-here/dp/PRODUCTID` can be "canonicalized" to `https://amazon.com/dp/PRODUCTID`.

I have not yet decided if I want to remove the `minimize` flag and/or just make it the default behavior.

Input on the matter would be appreciated.

## Basic usage

By default, compiling URL Cleaner includes the [`default-config.json`](default-config.json) file in the binary. Because of this, URL Cleaner can be used simply with `url-cleaner "https://example.com/of?a=dirty#url"`.

### The default config

The default config is intended to always obey the following rules:

- "Meaningful semantic changes"<sup>[definition?]</sup> should only ever occur as a result of a flag being enabled.
    - Insignificant details like the item categories navbar on amazon listings being slightly different are insignificant.
- URLs that are "semantically valid"<sup>[definition?]</sup> shouldn't ever throw an error.
    - URLs that aren't semantically valid also shouldn't ever throw an error but that is generally less important.
    - URLs that are semantically valid should never change semantics and/or become semantically invalid.
        - URLs that are semantically invalid may become semantically valid if there is an obvious way to do so. See the `unmangle` flag for details.
- Outside of long (>10)/infinite redirects/redirects, it should always be idempotent.
- Outside of redirect sites changing behavior, network connectivity issues, or other similarly difficult things to guarantee determinism for, it should always be deterministic.
- The `command` and feature, as well as any features starting with `debug` or `experiment` are never expected to be enabled.
    The `command` feature is enabled by default for convenience but, for situations where untrusted/user-provided configs have a chance to be run, should be disabled.

Currently no guarantees are made, though when the above rules are broken it is considered a bug and I'd appreciate being told about it.

Additionally, these rules may be changed at any time for any reason. Usually just for clarification.

<!--cmd scripts/gen-docs.sh-->
#### Flags

Flags let you specify behaviour with the `--flag name --flag name2` command line syntax.

Various flags are included in the default config for things I want to do frequently.

- `assume-1-dot-2-is-redirect`: Treat all hosts that match the Regex `^.\...$` as redirects. Let's be real, they all are.
- `breezewiki`: Sets the domain of `fandom.com` and [BreezeWiki](https://breezewiki.com/) to the domain specified by the `breezewiki-domain` variable.
- `bypass.vip`: Use [bypass.vip](https://bypass.vip) to expand linkvertise links. Currently untestable as the API is down.
- `deadname-twitter`: Change `x.com` to `twitter.com`.
- `discord-compatibility`: Sets the domain of `twitter.com` to the domain specified by the `twitter-embed-domain` variable.
- `discord-unexternal`: Replace `images-ext-1.discordapp.net` with the original images they refer to.
- `minimize`: Remove non-essential parts of that are likely not tracking related.
- `no-http`: Don't make any HTTP requests.
- `no-https-upgrade`: Disable replacing `http://` with `https://`.
- `no-unmangle`: Disable all unmangling.
- `no-unmangle-host-is-http-or-https`: Don't convert `https://https//example.com/abc` to `https://example.com/abc`.
- `no-unmangle-path-is-url`: Don't convert `https://example1.com/https://example2.com/user` to `https://example2.com/abc`.
- `no-unmangle-path-is-url-encoded-url`: Don't convert `https://example.com/https%3A%2F%2Fexample.com%2Fuser` to `https://example.com/user`.
- `no-unmangle-second-path-segment-is-url`: Don't convert `https://example1.com/profile/https://example2.com/profile/user` to `https://example2.com/profile/user`.
- `no-unmangle-subdomain-ends-in-not-subdomain`: Don't convert `https://profile.example.com.example.com` to `https://profile.example.com`.
- `no-unmangle-subdomain-starting-with-www-segment`: Don't convert `https://www.username.example.com` to `https://username.example.com`.
- `onion-location`: Replace hosts with results from the `Onion-Location` HTTP header if present.
- `tor2web`: Append the suffix specified by the `tor2web-suffix` variable to `.onion` domains.
- `tor2web2tor`: Replace `**.onion.**` domains with `**.onion` domains.
- `tumblr-unsubdomain-blog`: Changes `blog.tumblr.com` URLs to `tumblr.com/blog` URLs. Doesn't move `at` or `www` subdomains.
- `unbreezewiki`: Turn [BreezeWiki](https://breezewiki.com/) into `fandom.com`.
- `unmobile`: Convert `https://m.example.com`, `https://mobile.example.com`, `https://abc.m.example.com`, and `https://abc.mobile.example.com` into `https://example.com` and `https://abc.example.com`.
- `youtube-unplaylist`: Removes the `list` query parameter from `/watch` URLs.
- `youtube-unshort`: Turns `https://youtube.com/shorts/abc` into `https://youtube.com/watch?v=abc`.

If a flag is enabled in a config's `params` field, it can be disabled using `--unflag flag1 --unflag flag1`.

#### Variables

Variables let you specify behaviour with the `--var name value --var name2 value2` command line syntax.

Various variables are included in the default config for things that have multiple useful values.

- `breezewiki-domain`: The domain to use to turn `fandom.com` and BreezeWiki into [BreezeWiki](https://breezewiki.com/). Defaults to `breezewiki.com`
- `bypass.vip-api-key`: The API key used for [bypass.vip](https://bypass.vip)'s premium backend. Overrides the `URL_CLEANER_BYPASS_VIP_API_KEY` environment variable.
- `tor2web-suffix`: The suffix to append to the end of `.onion` domains if the flag `tor2web` is enabled. Should not start with `.` as that's added automatically. Left unset by default.
- `twitter-embed-domain`: The domain to use for twitter when the `discord-compatibility` flag is specified. Defaults to `vxtwitter.com`.

If a variable is specified in a config's `params` field, it can be unspecified using `--unvar var1 --unvar var2`.

#### Environment variables

There are some things you don't want in the config, like API keys, that are also a pain to repeatedly insert via `--var abc xyz`. For this, URL Cleaner does make use of native environment variables.

- `URL_CLEANER_BYPASS_VIP_API_KEY`: The API key used for [bypass.vip](https://bypass.vip)'s premium backend. Can be overridden with the `bypass.vip-api-key` variable.

#### Sets

Sets let you check if a string is one of many specific strings very quickly.

Various sets are included in the default config.

- `breezewiki-hosts`: Hosts to replace with the `breezewiki-domain` variable when the `breezewiki` flag is enabled. `fandom.com` is always replaced and is therefore not in this set.
- `bypass.vip-hosts`: Hosts to use bypass.vip for.
- `email-link-format-1-hosts`: (TEMPORARY NAME) Hosts that use unknown link format 1.
- `https-upgrade-host-blacklist`: Hosts to not upgrade from `http` to `https` even when the `no-https-upgrade` flag isn't enabled.
- `lmgtfy-hosts`: Hosts to replace with `google.com`.
- `redirect-hosts`: Hosts that are considered redirects in the sense that they return HTTP 3xx status codes. URLs with hosts in this set (as well as URLs with hosts that are "www." then a host in this set) will have the `ExpandRedirect` mapper applied.
- `unmangle-path-is-url-host-whitelist`: Effectively the `no-unmangle-path-is-url` for specified hosts.
- `unmangle-subdomain-ends-in-not-subdomain-not-subdomain-whitelist`: Effectively `no-unmangle-subdomain-ends-in-not-subdomain-not-subdomain-whitelist` for specified not subdomains.
- `unmangle-subdomain-starting-with-www-segment-not-subdomain-whitelist`: The `NotSubdomain`s to apply the `no-unmangle-subdomain-starting-with-www-segment` flag for.
- `unmobile-not-subdomain-blacklist`: The `NotSubdomain`s to never apply the `unmobile` flag for.
- `utps`: The set of "universal tracking parameters" that are always removed for any URL with a host not in the `utp-host-whitelist` set.
    Please note that the `utps` common mapper in the default config also removes any parameter starting with any string in the `utp-prefixes` list and thus parameters starting with those can be omitted from this set.
- `utps-host-whitelist`: Hosts to never remove universal tracking parameters from.

Sets can have elements inserted into them using `--insert-into-set name1 value1 value2 --insert-into-set name2 value3 value4`.

Sets can have elements removed from them using `--remove-from-set name1 value1 value2 --remove-from-set name2 value3 value4`.

#### Lists

Lists allow you to iterate over strings for things like checking if another string contains any of them.

Currently only one list is included in the default config:

- `utp-prefixes`: If a query parameter starts with any of the strings in this list (such as `utm_`) it is removed.

Currently there is no command line syntax for them. There really should be.
<!--/cmd-->

#### Citations

The people and projects I have stolen various parts of the default config from.

- [Mozilla Firefox's Extended Tracking Protection's query stripping](https://firefox-source-docs.mozilla.org/toolkit/components/antitracking/anti-tracking/query-stripping/index.html)
- [Brave Browser's query filter](https://github.com/brave/brave-core/blob/master/components/query_filter/utils.cc)
- [AdGuard's Tracking Parameters Filter](https://github.com/AdguardTeam/AdguardFilters/blob/master/TrackParamFilter/sections)
- [FastForward](https://github.com/FastForwardTeam/FastForward)

## Custom rules

Although proper documentation of the config schema is pending me being bothered to do it, the `url_cleaner` crate itself is reasonably well documented and the various types are (I think) fairly easy to understand.  
The main files you want to look at are [`conditions.rs`](src/rules/conditions.rs) and [`mappers.rs`](src/rules/mappers.rs).  
Additionally [`url_part.rs`](src/types/url_part.rs), [`string_location.rs`](src/types/string_location.rs), and [`string_modification.rs`](src/types/string_modification.rs) are very important for more advanced rules.

### Footguns

There are various things in/about URL Cleaner that I or many consider stupid but for various reasons cannot/should not be "fixed". These include but are not limited to:

- For `UrlPart`s and `Mapper`s that use "suffix" semantics (the idea that the '.co.uk" in "google.co.uk" is semantically the same as the ".com" in "google.com"'), the [psl](https://docs.rs/psl/latest/psl/) crate is used which in turn uses [Mozilla's Public Suffix List](https://publicsuffix.org/).
    Various suffixes are included that one may expect to be normal domains, such as blogspot.com.
    If for some reason a domain isn't working as expected, that may be the issue.

### Reference for people who don't know Rust's syntax:

- [`Option<...>`](https://doc.rust-lang.org/std/option/enum.Option.html) just means a value can be `null` in the JSON. `{"abc": "xyz"}` and `{"abc": null}` are both valid states for a `abc: Option<String>` field.
- [`Box<...>`](https://doc.rust-lang.org/std/boxed/struct.Box.html) has no bearing on JSON syntax or possible values. It's just used so Rust can put types inside themselves.
- [`Vec<...>`](https://doc.rust-lang.org/std/vec/struct.Vec.html) and [`HashSet<...>`](https://doc.rust-lang.org/std/collections/struct.HashSet.html) are written as lists.
- [`HashMap<..., ...>`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) and [`HeaderMap`](https://docs.rs/reqwest/latest/reqwest/header/struct.HeaderMap.html) are written as maps.
    - [`HeaderMap`](https://docs.rs/reqwest/latest/reqwest/header/struct.HeaderMap.html) keys are always lowercase.
- [`u8`](https://doc.rust-lang.org/std/primitive.u8.html), [`u16`](https://doc.rust-lang.org/std/primitive.u16.html), [`u32`](https://doc.rust-lang.org/std/primitive.u32.html), [`u64`](https://doc.rust-lang.org/std/primitive.u64.html), [`u128`](https://doc.rust-lang.org/std/primitive.u128.html), and [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) are unsigned (never negative) integers. [`i8`](https://doc.rust-lang.org/std/primitive.i8.html), [`i16`](https://doc.rust-lang.org/std/primitive.i16.html), [`i32`](https://doc.rust-lang.org/std/primitive.i32.html), [`i64`](https://doc.rust-lang.org/std/primitive.i64.html), [`i128`](https://doc.rust-lang.org/std/primitive.i128.html), and [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) are signed (maybe negative) integers. [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) is a [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) on 32-bit computers and [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) on 64-bit computers. Likewise [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) is [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) and [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) under the same conditions. In practice, if a number makes sense to be used in a field then it'll fit.
- [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html) is written as `{"secs": u64, "nanos": 0..1000000000}`.
- If a field starts with [`r#`](https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html) you write it without the [`r#`](https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html) (like `"else"`). [`r#`](https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html) is just Rust syntax for "this isn't a keyword".
- [`StringSource`](src/types/string_source.rs), [`GlobWrapper`](src/glue/glob.rs), [`RegexWrapper`](src/glue/regex.rs), [`RegexParts`](src/glue/regex/regex_parts.rs), and [`CommandWrapper`](src/glue/command.rs) can be written as both strings and maps.
    - [`RegexWrapper`](src/glue/regex.rs) and [`RegexParts`](src/glue/regex/regex_parts.rs) don't do any handling of [`/.../i`](https://en.wikipedia.org/wiki/Regex#Delimiters)-style syntax.
    - [`CommandWrapper`](src/glue/command.rs) doesn't do any argument parsing.
- [`#[serde(default)]`](https://serde.rs/field-attrs.html#default) and [`#[serde(default = "...")]`](https://serde.rs/field-attrs.html#default--path) allow for a field to be omitted when the desired value is almost always the same.
    - For [`Option<...>`](https://doc.rust-lang.org/std/option/enum.Option.html) fields, the default is `null`.
    - For [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) fields, the default is `false`.
- [`#[serde(skip_serializing_if = "...")]`](https://serde.rs/field-attrs.html#skip_serializing_if) lets the `--print-config` CLI flag omit unnecessary details (like when a field's value is its default value).
- [`#[serde(from = "...")]`](https://serde.rs/container-attrs.html#from), [`#[serde(into = "...")]`](https://serde.rs/container-attrs.html#into), [`#[serde(remote = "...")]`](https://serde.rs/container-attrs.html#remote), [`#[serde(serialize_with = "...")]`](https://serde.rs/field-attrs.html#serialize_with), [`#[serde(deserialize_with = "...")]`](https://serde.rs/field-attrs.html#deserialize_with), and [`#[serde(with = "...")]`](https://serde.rs/field-attrs.html#with) are implementation details that can be mostly ignored.
- [`#[serde(remote = "Self")]`](https://serde.rs/container-attrs.html#remote) is a very strange way to allow a struct to be deserialized from a map or a string. See [serde_with#702](https://github.com/jonasbb/serde_with/issues/702#issuecomment-1951348210) for details.

Additionally, regex support uses the [regex](https://docs.rs/regex/latest/regex/index.html) crate, which doesn't support look-around and backreferences.  
Certain common regex operations are not possible to express without those, but this should never come up in practice.

## MSRV

The Minimum Supported Rust Version is the latest stable release. URL Cleaner may or may not work on older versions, but there's no guarantee.

If this is an issue I'll do what I can to lower it but Diesel also keeps a fairly recent MSRV so you may lose caching.

## Untrusted input

Although URL Cleaner has various feature flags that can be disabled to make handling untrusted input safer, no guarantees are made. Especially if the config file being used is untrusted.  
That said, if you notice any rules that use but don't actually need HTTP requests or other data-leaky features, please let me know.

## CLI

### Parsing output

Note: [JSON output is supported](#json-output).

Unless a `Debug` variant is used, the following should always be true:

1. Input URLs are a list of URLs starting with URLs provided as command line arguments then each line of the STDIN.

2. The nth line of STDOUT corresponds to the nth input URL.

3. If the nth line of STDOUT is empty, then something about reading/parsing/cleaning the URL failed.

4. The nth non-empty line of STDERR corresponds to the nth empty line of STDOUT.

    1. Currently empty STDERR lines are not printed when a URL succeeds. While it would make parsing the output easier it would cause visual clutter on terminals. While this will likely never change by default, parsers should be sure to follow 4 strictly in case this is added as an option.

### JSON output

The `--json`/`-j` flag can be used to have URL Cleaner output JSON instead of lines.

The exact format is currently in flux.

### Exit code

Currently, the exit code is determined by the following rules:

- If no   cleanings work and none fail, returns 0. This only applies if no URLs are provided.
- If no   cleanings work and some fail, returns 1.
- If some cleanings work and none fail, returns 0.
- If some cleanings work and some fail, returns 2.

## Panic policy

URL Cleaner should only ever panic under the following circumstances:

- Parsing the CLI arguments failed.

- Loading/parsing the config failed.

- Printing the config failed. (Shouldn't be possible.)

- Testing the config failed.

- Reading from/writing to STDIN/STDOUT/STDERR has a catastrophic error.

- Running out of memory resulting in a standard library function/method panicking. This should be extremely rare.

- (Only possible when the `debug` feature is enabled) The mutex controlling debug printing indenting is poisoned and a lock is attempted.
    This should only be possible when URL Cleaner is used as a library.

Outside of these cases, URL Cleaner should never panic. However as this is equivalent to saying "URL Cleaner has no bugs", no actual guarantees can be made.

## Funding

URL Cleaner does not accept donations. If you feel the need to donate please instead donate to [The Tor Project](https://donate.torproject.org/) and/or [The Internet Archive](https://archive.org/donate/).
