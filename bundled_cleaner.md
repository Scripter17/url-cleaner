# The bundled cleaner

The bundled cleaner; Meant for general purpose every day URL Cleaning.

Licensed under the AGPL 3.0 or later.

## Params

### Flags

- `keep_affiliate`: Don't remove affiliate info from affiliate links.
- `bypass_vip`: Use [bypass.vip](https://bypass.vip) to expand various complicated/otherwise unsupported redirect sites.
- `keep_http`: Disable upgrading `http` URLs to `https`. See the `nh_keep_http` set if you only want to not upgrade specific hosts.
- `no_network`: Don't make any network requests. Some redirect websites will still work because they include the destination in the URL.
- `remove_unused_search_query`: Remove search queries from URLs that aren't search results (for example, posts).
- `tor2web2tor`: Change `**.onion.**` hosts to `**.onion`.
- `discord_unexternal`: Change `images-ext-*.discordapp.net` URLs to the original images they refer to.
- `instagram_unprofilecard`: Change `instagram.com/username/profilecard` to `instagram.com/username`.
- `tumblr_unsubdomain_blog`: Change `blog.tumblr.com` to `tumblr.com/blog`.
- `youtube_unplaylist`: Remove the `list` query param from `youtube.com/watch` URLs.
- `youtube_unshort`: Change `youtube.com/shorts/abc` to `youtube.com/watch?v=abc`.

### Vars

Please note that the presence of required vars and validity of varianted vars are only checked when asserting suitability.

Cleaners that break the "invariants" here can be parsed and used, but will likely exhibit unintended behavior.

- `client_type`: Whether the client is a desktop or mobile device.
  - Required: false.
  - Unset: Keep URLs as-is.
  - `desktop`: to change mobile URLs to desktop URLs.
  - `mobile`: to change dekstop URLs to mobile URLs.
- `bypass_vip_api_key`: The API key used for [bypass.vip](https://bypass.vip). Overrides the `URL_CLEANER_BYPASS_VIP_API_KEY` environment var.
  - Required: false.
- `bluesky_mode`: Decides what host to replace Bluesky/Bluesky embed hosts with.
  - Required: false.
  - Unset: Set Bluesky and Bluesky embed hosts to `bsky.app`.
  - `canon`: Always set it to `bsky.app`.
  - `embed`: Always set it to the `bluesky_embed_host` var.
- `bluesky_embed_host`: The Bluesky embed host to use.
  - Required: true.
  - Default: `fxbsky.app`.
- `fandom_mode`: Decides what to replace Fandom/Breezewiki URLs with.
  - Required: false.
  - Unset: Keep fandom URLs as-is and change Breezewiki URLs to the `breezewiki_host` var.
  - `canon`: Set to fandom.
  - `breezewiki`: Set to the `breezewiki_host` var.
- `breezewiki_host`: The Breezewiki host to use.
  - Required: true.
  - Default: `breezewiki.com`.
- `furaffinity_mode`: Decides what host to replace reddit hosts with.
  - Required: false.
  - Unset: Keep as-is.
  - `canon`: Set to `www.furaffinity.net`.
  - `sfw`: Set to `sfw.furaffinity.net`.
  - `nsfw`: Set to `www.furaffinity.net`.
- `pixiv_mode`: Decides what host to replace pixiv/pixiv embed hosts with.
  - Required: false.
  - Unset: Set pixiv and pixiv embed hosts to `www.pixiv.net`.
  - `canon`: Always set it to `www.pixiv.net`.
  - `embed`: Always set it to the `pixiv_embed_host` var.
- `pixiv_embed_host`: The pixiv embed host to use.
  - Required: true.
  - Default: `phixiv.net`.
- `reddit_mode`: Decides what host to replace reddit hosts with.
  - Required: false.
  - Unset: Keep as-is.
  - `canon`: Set to `www.reddit.com`.
  - `old`: Set to `old.reddit.com`.
  - `new`: Set to `www.reddit.com`.
- `twitter_mode`: Decides what host to replace twitter/twitter embed/Nitter hosts with.
  - Required: false.
  - Unset: Set twitter and twitter embed hosts to `x.com` and Nitter hosts to the `nitter_host` var.
  - `canon`: Always set it to `x.com`.
  - `embed`: Always set it to the `twitter_embed_host` var.
  - `nitter`: Always set it to the `nitter_host` var. Defaults to unset.
- `twitter_embed_host`: The twitter embed host to use.
  - Required: true.
  - Default: `fixupx.com`.
- `nitter_host`: The Nitter host to use.
  - Required: true.
  - Default: `nitter.net`.
- `youtube_mode`: Decides what host to replace youtube/Invidious hosts with.
  - Required: false.
  - Unset: Set youtube hosts to `www.youtube.com` and Invidious hosts to the `invidious_host` var.
  - `canon`: Always set it to `www.youtube.com`.
  - `invidious`: Always set it to the `invidious_host` var.
- `invidious_host`: The Invidious host to use.
  - Required: true.
  - Default: `yewtu.be`.

### Sets

- `utps`: Universal tracking parameters to remove from all URLs without explicit allowances. See the `utp_prefixes` list for a list of prefixes only used for universal tracking parameters.
- `nh_keep_http`: The `NormalizedHost`s to not upgrade from `http` to `https`.

### Lists

- `utp_prefixes`: Prefixes only used for universal tracking parameters. See the `utps` set for specific query params to always remove.

### Partitionings

- `nh_categories`: Categories of similar websites with shared cleaning methods.
- `rd_www_subdomain`: The `RegDomain`s to ensure/remove `www` `Subdomain`s.
- `dm_www_subdomain`: The `DomainMiddle`s to ensure/remove `www` `Subdomain`s.
- `dm_expand_mode`: How to handle redirect `DomainMiddle`s.
- `nh_expand_mode`: How to handle redirect `NormalizedHost`s.
- `rd_expand_mode`: How to handle redirect `RegDomain`s,
- `mobilizer`: Which part at which value makes a `NormalizedHost`/`RegDomain` a mobile URL.

## Task context

### Vars

- `unmangle_mode`: The unmangle mode to use.
- `site`: The name of the website used in the contact_info unmangle_mode.
- `text`: The text of the link used in the contact_info unmangle_mode.

## Environment Vars

- `URL_CLEANER_BYPASS_VIP_API_KEY`: The API key used for [bypass.vip](https://bypass.vip). Can be overridden with the `bypass_vip_api_key` var.

