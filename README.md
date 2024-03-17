# Url Cleaner

A configurable URL cleaner built in Rust.

## Basic usage

By default, compiling URL Cleaner includes the [`default-config.json`](default-config.json) file in the binary. Because of this, URL Cleaner can be used simply with `url-cleaner "https://example.com/of?a=dirty#url"`.

The default config shouldn't ever change the semantics of a URL. Opening a URL before and after cleaning should always give the same result. (except for stuff like the categories amazon puts in one of its 7 billion navbars but do you *really* care about that?)  
Because websites tend to not document what parts of their URLs are and aren't necessary, the default config almost certainly runs into issues when trying to clean niche URLs like advanced search queries or API endpoints.  
If you find any instance of the default config changing the meaning/result of a URL, please open an [issue](https://github.com/Scripter17/url-cleaner/issues).

Additionally, if you find any example of a malformed URL that can be unambiguously transformed into what was intended (`https://abc.tumblr.com.tumblr.com` -> `https://abc.tumblr.com` and `https://bsky.app/profile/abc` -> `https://bsky.app/profile/abc.bsky.social`), please open an issue.  
Since these are somewhat common when social media sites have dedicated fields for other social medias, it's worth handling these.

## Variables

Variables let you specify behaviour with the `--var name=value --var name2=value2` command line syntax.
Various variables are included in the default config for things I want to do frequently.

- `twitter-embed-domain`: The domain to use for twitter URLs when the `discord-compatibility` flag is specified. Defaults to `vxtwitter.com`.
- `breezewiki-domain`: The domain to use to turn `fandom.com` URLs into [BreezeWiki](https://breezewiki.com/) URLs. Defaults to `breezewiki.com`
- `tor2web-suffix`: The suffix to append to the end of `.onion` domains if the flag `tor2web` is set. Should not start with `.` as that's added automatically. Left unset by default.

If a variable is specified in a config's `"params"` field, it can be unspecified using `--unvar var1 --unvar var2`.

## Flags

Flags let you specify behaviour with the `--flag name --flag name2` command line syntax.
Various flags are included in the default config for things I want to do frequently.

- `bypass.vip`: Use [bypass.vip](https://bypass.vip) to expand various link shorteners that are too complex and/or obscure for me to implement.
- `no-unmangle`: Disable turning `https://user.example.com.example.com` into `https://user.example.com` and `https://example.com/https://example.com/abc` and `https://example.com/xyz/https://example.com/abc` into `https://example.com/abc`.
- `no-https-upgrade`: Disable replacing `http://` URLs with `https://` URLs.
- `unmobile`: Convert `https://m.example.com`, `https://mobile.example.com`, `https://abc.m.example.com`, and `https://abc.mobile.example.com` into `https://example.com` and `https://abc.example.com`.
- `youtube-unshort`: Turns `https://youtube.com/shorts/abc` URLs into `https://youtube.com/watch?v=abc` URLs.
- `discord-external`: Replace `images-ext-1.discordapp.net` URLs with the original images they refer to.
- `discord-compatibility`: Sets the domain of `twitter.com` URLs to the domain specified by the `twitter-embed-domain` variable.
- `breezewiki`: Sets the domain of `fandom.com` and [BreezeWiki](https://breezewiki.com/) URLs to the domain specified by the `breezewiki-domain` variable.
- `unbreezewiki`: Turn [BreezeWiki](https://breezewiki.com/) URLs into `fandom.com` URLs.
- `onion-location`: Send an HTTP GET request to the url and apply the [`Onion-Location`](https://community.torproject.org/onion-services/advanced/onion-location/) response header if found.
- `tor2web`: Append the suffix specified by the `tor2web-suffix` variable to `.onion` domains.
- `tor2web2tor`: Replace `**.onion.**` domains with `**.onion` domains.

Flags can be added to configs by using the `FlagSet` condition and specified at runtime by doing `--flag flag1 --flag flag2`.

If a flag is set in a config's `"params"` field, it can be unset using `--unflag flag1 --unflag flag1`.

## Custom rules

Although proper documentation of the config schema is pending me being bothered to do it, the `url_cleaner` crate itself is well documented and the structs and enums are (I think) fairly easy to understand.  
The main files you want to look at are [`conditions.rs`](src/rules/conditions.rs) and [`mappers.rs`](src/rules/mappers.rs).  
Additionally [`url_part.rs`](src/types/url_part.rs), [`string_location.rs`](src/types/string_location.rs), and [`string_modification.rs`](src/types/string_modification.rs) are very important for more advanced rules.

Until I publish URL Cleaner on crates.io/docs.rs, you can read the documentation by `git clone`ing this repository and running `cargo doc --no-deps` in the root directory then viewing the files in `target/doc/url_cleaner` in a web browser.  
If the URL in your web browser looks like `file:///run/...` and the webpage is white with numbers on the left side, you should run `python3 -m http.server` in the root directory and open `http://localhost:8000/target/doc/url_cleaner/`.

### Tips for people who don't know Rust's syntax:

- [`Option<...>`](https://doc.rust-lang.org/std/option/enum.Option.html) just means a value can be `null` in the JSON. `{"abc": "xyz"}` and `{"abc": null}` are both valid states for a `abc: Option<String>` field.
- [`Box<...>`](https://doc.rust-lang.org/std/boxed/struct.Box.html) has no bearing on JSON syntax or possible values. It's just used so Rust can put types inside themselves.
- [`Vec<...>`](https://doc.rust-lang.org/std/vec/struct.Vec.html) and [`HashSet<...>`](https://doc.rust-lang.org/std/collections/struct.HashSet.html) are written as lists in JSON.
- [`[...; N]`](https://doc.rust-lang.org/std/primitive.array.html) is written as a list of length `N`.
- [`HashMap<..., ...>`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) and [`HeaderMap`](https://docs.rs/reqwest/latest/reqwest/header/struct.HeaderMap.html) are written as dictionaries in JSON. Please note that header names are always lowercase.
- Fields preceded by [`#[serde(default)]`](https://serde.rs/field-attrs.html#default) or [`#[serde(default = "...")]`](https://serde.rs/field-attrs.html#default--path) can be omitted from config files. The defaults are almost always what you want.
- [`u8`](https://doc.rust-lang.org/std/primitive.u8.html), [`u16`](https://doc.rust-lang.org/std/primitive.u16.html), [`u32`](https://doc.rust-lang.org/std/primitive.u32.html), [`u64`](https://doc.rust-lang.org/std/primitive.u64.html), [`u128`](https://doc.rust-lang.org/std/primitive.u128.html), and [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) are unsigned (non-negative) integers. [`i8`](https://doc.rust-lang.org/std/primitive.i8.html), [`i16`](https://doc.rust-lang.org/std/primitive.i16.html), [`i32`](https://doc.rust-lang.org/std/primitive.i32.html), [`i64`](https://doc.rust-lang.org/std/primitive.i64.html), [`i128`](https://doc.rust-lang.org/std/primitive.i128.html), and [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) are signed integers. [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) is a [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) on 32-bit computers and [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) on 64-bit computers. Likewise [`isize`](https://doc.rust-lang.org/std/primitive.isize.html) is [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) and [`i64`](https://doc.rust-lang.org/std/primitive.i64.html) under the same conditions. In practice, if a number makes sense to be used in a field then it'll fit.
- A [`StringSource`](src/types/string_source.rs) is usually just written as a string. To see how it can be used to get URL parts or variables see [`string_source.rs`](src/types/string_source.rs).
- If a field starts with [`r#`](https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html) (like `r#else`) you write it without the [`r#`](https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html) (like `"else"`). [`r#`](https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html) is just Rust syntax for "this isn't a keyword".
- [`StringSource`](src/types/string_source.rs), [`GlobWrapper`](src/glue/glob.rs), [`RegexWrapper`](src/glue/regex.rs), [`RegexParts`](src/glue/regex/regex_parts.rs), and [`CommandWrapper`](src/glue/command.rs) can be either strings or structs. The main limits are that [`RegexWrapper`](src/glue/regex.rs) and [`RegexParts`](src/glue/regex/regex_parts.rs) don't do any handling of [`/.../i`](https://en.wikipedia.org/wiki/Regex#Delimiters)-style syntax and [`CommandWrapper`](src/glue/command.rs) doesn't do any argument parsing. If you need those, use the map syntax.

Furthermore, regex support uses the [regex](https://docs.rs/regex/latest/regex/index.html) crate, which does not support look-around and backreferences.  
Certain common regex operations are not possible to express without those, but this should never come up in practice.

### Custom rule performance

A few commits before the one that added this text, I moved the "always remove these query parameters" rule to the bottom.  
That cut the runtime for amazon URLs in half.

The reason is fairly simple: Instead of removing some of the query then removing all of it, if you remove all of it first then the "remove these parameters" does nothing.

While I have done my best to ensure URL Cleaner is as fast as I can get it, that does not mean you shouldn't be careful with rule order.

I know to most people in most cases, 10k URLs in 120ms versus 10k URLs in 60ms is barely noticeable, but that kind of thinking is why video games require mortgages.

## Anonymity

Because most people don't use URL Cleaner, using URL Cleaner can let websites correlate information similar to URL tracking parameters.  
If you're the only person without a tracking parameter in links, it's fairly easy to distinguish you from everyone else.

Additionally, expanding short links or otherwise using rules that send HTTP requests could nullify and perceived benefit URL Cleaner provides.  
If a `t.co` link points to `example.com?utm_source=twitter`, `example.com` will see your IP access `example.com?utm_source=twitter` even though the `utm_source` is removed by a later rule.  
For specifically link shorteners (other HTTP requests including those used for `bypass.vip` currently don't cache), the cache makes this a one-time-per-link issue, but if you're cleaning untrusted input PLEASE use some form of VPN or proxy. Preferably Tor if none of the websites you clean throw a hissy fit at Tor.

As with Tor, protests, and, really, everything, safety comes in numbers.

## MSRV

The Minimum Supported Rust Version is the latest stable release. URL Cleaner may or may not work on older versions, but there's no guarantee.

## Untrusted input

Although URL Cleaner has various feature flags that can be disabled to make handling untrusted input safer, no guarantees are made. Especially if the config file being used is untrusted.  
That said, if you find something to be unnecessarily unsafe, please open an issue so it can be fixed.

(Note that URL Cleaner doesn't use any `unsafe` code. I mean safety in terms of IP leaks and stuff.)

## Backwards compatibility

URL Cleaner is currently in heavy flux so expect library APIs and the config schema to change at any time for any reason.

## Default config sources

The people and projects I have stolen various parts of the default config from.

- [Mozilla Firefox's Extended Tracking Protection's query stripping](https://firefox-source-docs.mozilla.org/toolkit/components/antitracking/anti-tracking/query-stripping/index.html)
- [Brave Browser's query filter](https://github.com/brave/brave-core/blob/master/components/query_filter/utils.cc)
- [AdGuard's Tracking Parameters Filter](https://github.com/AdguardTeam/AdguardFilters/blob/master/TrackParamFilter/sections)
