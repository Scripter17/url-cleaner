# URL Cleaner

Websites often put unique identifiers into URLs so that, when you send a link to a friend and they open it, the website knows it was you who sent it to them.  
As most people do not understand and therefore cannot consent to this, it is polite to remove the spyware query parameters before sending URLs to people.  
URL Cleaner is an extremely versatile tool designed to make this process as comprehensive, fast, and easy as possible.

## Anonymity

In theory, if you're the only one sharing posts from a website without URL trackers, the website could realize that and track you in the same way.  
In practise you are very unlikely to be the only one sharing clean URLs. Search engines generally provide URLs without trackers<sup>[citation needed]</sup>, some people manually remove trackers, and some websites like [vxtwitter.com](https://github.com/dylanpdx/BetterTwitFix) automatically strip URL trackers.

However, for some websites the default config strips more stuff than search engines. In this case anonymity does fall back to many people using URL Cleaner and providing cover for each other.

As with Tor, protests, and anything else where privacy matters, safety comes in numbers.

## Basic usage

By default, compiling URL Cleaner includes the [`default-config.json`](default-config.json) file in the binary. Because of this, URL Cleaner can be used simply with `url-cleaner "https://example.com/of?a=dirty#url"`.

### The default config

The default config is intended to always obey the following rules:

- "Meaningful semantic changes"<sup>[definition?]</sup> should only ever occur as a result of a flag being enabled.
- URLs that are "semantically valid"<sup>[definition?]</sup> should never throe an error.
- Outside of long (>10)/cyclic redirects/shortlinks, it should always be idempotent.

Currently no guarantees are made.

Additionally, these rules may be changed at any time for any reason.

#### Flags

Flags let you specify behaviour with the `--flag name --flag name2` command line syntax.

Various flags are included in the default config for things I want to do frequently.

- `no-http`: Don't make any HTTP requests.
- `minimize`: Remove non-essential parts of URLs that are likely not tracking related.
- `no-unmangle`: Disable turning `https://user.example.com.example.com` into `https://user.example.com` and `https://example.com/https://example.com/abc` and `https://example.com/xyz/https://example.com/abc` into `https://example.com/abc`.
- `no-unmangle-path-is-url`: Don't convert `https://example1.com/https://example2.com/user` URLs to `https://example2.com/abc` URLs.
- `no-unmangle-second-path-segment-is-url`: Don't convert `https://example1.com/profile/https://example2.com/profile/user` URLs to `https://example2.com/profile/user` URLs.
- `no-unmangle-subdomain-starting-with-www-segment`: Don't convert `https://www.username.example.com` URLs to `https://username.example.com` URLs.
- `no-https-upgrade`: Disable replacing `http://` URLs with `https://` URLs.
- `assume-cctld-is-shortlink`: Treat all domains ending in a 2 letter top level domain as shortlinks. Yes this does cause issues with stuff like google.
- `assume-1-dot-2-is-shortlink`: Treat all that match the Regex `^.\...$` as shortlinks. Let's be real, they all are.
- `unmobile`: Convert `https://m.example.com`, `https://mobile.example.com`, `https://abc.m.example.com`, and `https://abc.mobile.example.com` into `https://example.com` and `https://abc.example.com`.
- `youtube-unshort`: Turns `https://youtube.com/shorts/abc` URLs into `https://youtube.com/watch?v=abc` URLs.
- `discord-unexternal`: Replace `images-ext-1.discordapp.net` URLs with the original images they refer to.
- `discord-compatibility`: Sets the domain of `twitter.com` URLs to the domain specified by the `twitter-embed-domain` variable.
- `deadname-twitter`: Change `x.com` URLs to `twitter.com` URLs.
- `breezewiki`: Sets the domain of `fandom.com` and [BreezeWiki](https://breezewiki.com/) URLs to the domain specified by the `breezewiki-domain` variable.
- `unbreezewiki`: Turn [BreezeWiki](https://breezewiki.com/) URLs into `fandom.com` URLs.
- `onion-location`: Send an HTTP GET request to the url and apply the [`Onion-Location`](https://community.torproject.org/onion-services/advanced/onion-location/) response header if found. Because this runs on all URLs and the response isn't cached, it's advised you don't use this.
- `tor2web`: Append the suffix specified by the `tor2web-suffix` variable to `.onion` domains.
- `tor2web2tor`: Replace `**.onion.**` domains with `**.onion` domains.

If a flag is enabled in a config's `"params"` field, it can be disabled using `--unflag flag1 --unflag flag1`.

#### Variables

Variables let you specify behaviour with the `--var name=value --var name2=value2` command line syntax.

Various variables are included in the default config for things that have multiple useful values.

- `twitter-embed-domain`: The domain to use for twitter URLs when the `discord-compatibility` flag is specified. Defaults to `vxtwitter.com`.
- `breezewiki-domain`: The domain to use to turn `fandom.com` and BreezeWiki URLs into [BreezeWiki](https://breezewiki.com/) URLs. Defaults to `breezewiki.com`
- `tor2web-suffix`: The suffix to append to the end of `.onion` domains if the flag `tor2web` is enabled. Should not start with `.` as that's added automatically. Left unset by default.

If a variable is specified in a config's `"params"` field, it can be unspecified using `--unvar var1 --unvar var2`.

#### Sets

Sets let you check if a string is one of many specific strings very quickly.

Various sets are included in the default config.

- `breezewiki-hosts`: Hosts to replace with the `breezewiki-domain` variable when the `breezewiki` flag is enabled. `fandom.com` is always replaced and is therefore not in this set.
- `shortlink-hosts`: Hosts that are considered shortlinks in the sense that they return HTTP 3xx status codes. URLs with hosts in this set (as well as URLs with hosts that are "www." then a host in this set) will have the `ExpandShortLink` mapper applied.
- `utps`: The set of "universal tracking parameters" that are always removed for any URL with a host not in the `utp-host-whitelist` set.
    Please note that the UTP rule in the default config also removes any parmeter starting with `cm_mmc`, `__s`, `at_custom`, and `utm_` and thus parameters starting with those can be omitted from this set.
- `utps-host-whitelist`: Hosts to never remove universal tracking parameters from.
- `https-upgrade-host-blacklist`: Hosts to not upgrade from `http` to `https` even when the `no-https-upgrade` flag isn't enabled.

Sets can have elements inserted into them using `--insert-into-set name1 value1 value2 --insert-into-set name2 value3 value4`.

Sets can have elements removed from them using `--remove-from-set name1 value1 value2 --remove-from-set name2 value3 value4`.

#### Lists

Lists allow you to iterate over strings for things like checking if another string contains any of them.

Currently only one list is included in the default config:

- `utp-prefixes`: If a query parameter starts with any of the strings in this list (such as `utm_`) it is removed.

#### Citations

The people and projects I have stolen various parts of the default config from.

- [Mozilla Firefox's Extended Tracking Protection's query stripping](https://firefox-source-docs.mozilla.org/toolkit/components/antitracking/anti-tracking/query-stripping/index.html)
- [Brave Browser's query filter](https://github.com/brave/brave-core/blob/master/components/query_filter/utils.cc)
- [AdGuard's Tracking Parameters Filter](https://github.com/AdguardTeam/AdguardFilters/blob/master/TrackParamFilter/sections)

## Custom rules

Although proper documentation of the config schema is pending me being bothered to do it, the `url_cleaner` crate itself is well documented and the structs and enums are (I think) fairly easy to understand.  
The main files you want to look at are [`conditions.rs`](src/rules/conditions.rs) and [`mappers.rs`](src/rules/mappers.rs).  
Additionally [`url_part.rs`](src/types/url_part.rs), [`string_location.rs`](src/types/string_location.rs), and [`string_modification.rs`](src/types/string_modification.rs) are very important for more advanced rules.

Until I publish URL Cleaner on crates.io/docs.rs, you can read the documentation by `git clone`ing this repository and running `cargo doc --no-deps` in the root directory then viewing the files in `target/doc/url_cleaner` in a web browser.  
If the URL in your web browser looks like `file:///run/...` and the webpage is white with numbers on the left side, you should run `python3 -m http.server` in the cloned folder and open `http://localhost:8000/target/doc/url_cleaner/`.

### Footguns

There are various things in/about URL Cleaner that I or many consider stupid but for various reasons cannot/should not be fixed. These include but are not limited to:

- There are various `UrlPart` variants that use "suffix" semantics (treating `google.co.uk` the same as `google.com`).
    For this, URL Cleaner uses the [`psl`](https://docs.rs/psl/latest/psl/) crate which in turn uses [Mozilla's Public Suffix List](https://publicsuffix.org/).
    Mozilla's PSL includes `blogspot.com` (and possibly a few other suffixes one may expect to be a domain) which has various counterintuitive side effects that are documented in affected `UrlPart` variants.
    This is not the fault of the maintainers of the psl crate, arguably not the fault of Mozilla, and slightly less arguably not even the fault of google/the people at google who submitted `blogspot.com` to Mozilla's PSL,
    Also, this is not the fault of the URL spec as suffix semantics aren't actually a part of them. The TLD of `google.co.uk` *is* `uk`.
    I have decided to not diverge from the behavior the `psl` crate exhibits and have instead decided to make sure people know about this.
    I may or may not add a way to remove certain suffixes from the public suffix list URL Cleaner uses.

### Referemce for people who don't know Rust's syntax:

- [`Option<...>`](https://doc.rust-lang.org/std/option/enum.Option.html) just means a value can be `null` in the JSON. `{"abc": "xyz"}` and `{"abc": null}` are both valid states for a `abc: Option<String>` field.
- [`Box<...>`](https://doc.rust-lang.org/std/boxed/struct.Box.html) has no bearing on JSON syntax or possible values. It's just used so Rust can put types inside themselves.
- [`Vec<...>`](https://doc.rust-lang.org/std/vec/struct.Vec.html) and [`HashSet<...>`](https://doc.rust-lang.org/std/collections/struct.HashSet.html) are written as lists.
- [`HashMap<..., ...>`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) and [`HeaderMap`](https://docs.rs/reqwest/latest/reqwest/header/struct.HeaderMap.html) are written as maps.
    - [`HeaderMap`](https://docs.rs/reqwest/latest/reqwest/header/struct.HeaderMap.html) keys are always lowercase.
- [`u8`](https://doc.rust-lang.org/std/primitive.u8.html), [`u16`](https://doc.rust-lang.org/std/primitive.u16.html), [`u32`](https://doc.rust-lang.org/std/primitive.u32.html), [`u64`](https://doc.rust-lang.org/std/primitive.u64.html), [`u128`](https://doc.rust-lang.org/std/primitive.u128.html), and [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) are unsigned (never negative) integers. [`i8`](https://doc.rust-lang.org/std/primitive.i8.html), [`i16`](https://doc.rust-lang.org/std/primitive.i16.html), [`i32`](https://doc.rust-lang.org/std/primitive.i32.html), [`i64`](https://doc.rust-lang.org/std/primitive.i64.html), [`i128`](https://doc.rust-lang.org/std/primitive.i128.html), and [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) are signed (maybe negative) integers. [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) is a [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) on 32-bit computers and [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) on 64-bit computers. Likewise [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) is [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) and [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) under the same conditions. In practice, if a number makes sense to be used in a field then it'll fit.
- If a field starts with [`r#`](https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html) you write it without the [`r#`](https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html) (like `"else"`). [`r#`](https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html) is just Rust syntax for "this isn't a keyword".
- [`StringSource`](src/types/string_source.rs), [`GlobWrapper`](src/glue/glob.rs), [`RegexWrapper`](src/glue/regex.rs), [`RegexParts`](src/glue/regex/regex_parts.rs), and [`CommandWrapper`](src/glue/command.rs) can be written as both strings and maps.
    - [`RegexWrapper`](src/glue/regex.rs) and [`RegexParts`](src/glue/regex/regex_parts.rs) don't do any handling of [`/.../i`](https://en.wikipedia.org/wiki/Regex#Delimiters)-style syntax.
    - [`CommandWrapper`](src/glue/command.rs) doesn't do any argument parsing.
- [`#[serde(default)]`](https://serde.rs/field-attrs.html#default) and [`#[serde(default = "...")]`](https://serde.rs/field-attrs.html#default--path) allow for a field to be omitted when the desired value is almost always the same.
    - For [`Option<...>`](https://doc.rust-lang.org/std/option/enum.Option.html) fields, the default is `null`.
    - For [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) fields, the default is `false`.
- [`#[serde(skip_serializing_if = "...")]`](https://serde.rs/field-attrs.html#skip_serializing_if) lets the `--print-config` CLI flag omit uneccesary details (like when a field's value is its default value).
- [`#[serde(from = "...")]`](https://serde.rs/container-attrs.html#from), [`#[serde(into = "...")]`](https://serde.rs/container-attrs.html#into), [`#[serde(remote = "...")]`](https://serde.rs/container-attrs.html#remote), [`#[serde(serialize_with = "...")]`](https://serde.rs/field-attrs.html#serialize_with), [`#[serde(deserialize_with = "...")]`](https://serde.rs/field-attrs.html#deserialize_with), and [`#[serde(with = "...")]`](https://serde.rs/field-attrs.html#with) are implementation details that can be mostly ignored.
- [`#[serde(remote = "Self")]`](https://serde.rs/container-attrs.html#remote) is a very strange way to allow a struct to be deserialized from a map or a string. See [serde_with#702](https://github.com/jonasbb/serde_with/issues/702#issuecomment-1951348210) for details.

Additionally, regex support uses the [regex](https://docs.rs/regex/latest/regex/index.html) crate, which doesn't support look-around and backreferences.  
Certain common regex operations are not possible to express without those, but this should never come up in practice.

## MSRV

The Minimum Supported Rust Version is the latest stable release. URL Cleaner may or may not work on older versions, but there's no guarantee.

## Untrusted input

Although URL Cleaner has various feature flags that can be disabled to make handling untrusted input safer, no guarantees are made. Especially if the config file being used is untrusted.  
That said, if you notice any rules that use but don't actually need HTTP requests, please let me know.

(Note that URL Cleaner doesn't use any `unsafe` code. I mean safety in terms of IP leaks and stuff.)

## Backwards compatibility

URL Cleaner is currently in heavy flux so expect library APIs and the config schema to change at any time for any reason.

## CLI

### Parsing output

Unless `Mapper::(e|)Print(ln|)` or a `Debug` variant is used, the following should always be true:

1. Input URLs are a list of URLs starting with URLs provided as command line arguments then each line of the STDIN.

2. The nth line of STDOUT corresponds to the nth input URL.

3. If the nth line of STDOUT is empty, then something about reading/parsing/cleaning the URL failed.

4. The nth non-empty line of STDERR corresponds to the nth empty line of STDOUT.

    1. Currently empty STDERR lines are not printed when a URL succeeds. While it would make parsing the output easier it would cause visual clutter on terminals. While this will likely never change by default, parsers should be sure to follow 4 strictly in case this is added as an option.

#### JSON output

The `--json`/`-j` flag can be used to have URL Cleaner output JSON instead of lines.

The exact format is currently in flux.

If a `Mapper::Print(ln|)` is used, this is not guaranteed to be valid JSON.

## Panic policy

URL Cleaner should only ever panic under the following circumstances:

- Parsing the CLI arguments failed.

- Loading/parsing the config failed.

- Printing the config failed.

- Testing the config failed.

- Reading from/writing to STDIN/STDOUT/STDERR has a catastrophic error.

- Running out of memory resulting in a standard library function/method panicking. This should be extremely rare.

Outside of these cases, URL Cleaner should never panic. However as this is equivalent to saying "URL Cleaner has no bugs", no actual guarantees can be made.

## Funding

URL Cleaner does not accept donations. If you feel the need to donate please instead donate to [The Tor Project](https://donate.torproject.org/) and/or [The Internet Archive](https://archive.org/donate/).
