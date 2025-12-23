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

- `client_type`: Whether the client is a desktop or mobile device.
  - Unset to keep URLs as-is.
  - `desktop` to change mobile URLs to desktop URLs.
  - `mobile` to change dekstop URLs to mobile URLs.
- `bypass_vip_api_key`: The API key used for [bypass.vip](https://bypass.vip). Overrides the `URL_CLEANER_BYPASS_VIP_API_KEY` environment var.
- `bluesky_mode`: Decides what host to replace Bluesky/Bluesky embed hosts with.
  - Unset to set Bluesky and Bluesky embed hosts to `bsky.app`.
  - `canon` to always set it to `bsky.app`.
  - `embed` to always set it to the `bluesky_embed_host` var.
- `bluesky_embed_host`: The Bluesky embed host to use. Defaults to `fxbsky.com`.
- `fandom_mode`: Decides what to replace Fandom/Breezewiki URLs with.
  - Unset to keep fandom URLs as-is and change Breezewiki URLs to the `breezewiki_host` var.
  - `canon` to set to fandom.
  - `breezewiki` to set to the `breezewiki_host` var.
- `breezewiki_host`: The Breezewiki host to use. Defaults to `breezewiki.com`.
- `furaffinity_mode`: Decides what host to replace reddit hosts with.
  - Unset to keep as-is.
  - `canon` to set to `furaffinity.net`.
  - `sfw` to set to `sfw.furaffinity.net`.
  - `nsfw` to set to `www.furaffinity.net`.
- `pixiv_mode`: Decides what host to replace pixiv/pixiv embed hosts with.
  - Unset to set pixiv and pixiv embed hosts to `www.pixiv.net`.
  - `canon` to always set it to `www.pixiv.net`.
  - `embed` to always set it to the `pixiv_embed_host` var.
- `pixiv_embed_host`: The pixiv embed host to use. Defaults to `phixiv.com`.
- `reddit_mode`: Decides what host to replace reddit hosts with.
  - Unset to keep as-is.
  - `old` to set to `old.reddit.com`.
  - `new` to set to `www.reddit.com`.
- `twitter_mode`: Decides what host to replace twitter/twitter embed/Nitter hosts with.
  - Unset to set twitter and twitter embed hosts to `x.com` and Nitter hosts to the `nitter_host` var.
  - `canon` to always set it to `x.com`.
  - `embed` to always set it to the `twitter_embed_host` var.
  - `nitter` to always set it to the `nitter_host` var. Defaults to unset.
- `twitter_embed_host`: The twitter embed host to use.. Defaults to `fixupx.com`.
- `nitter_host`: The Nitter host to use. Defaults to `nitter.net`.
- `youtube_mode`: Decides what host to replace youtube/Invidious hosts with.
  - Unset to set youtube hosts to `www.youtube.com` and Invidious hosts to the `invidious_host` var.
  - `canon` to always set it to `www.youtube.com`.
  - `invidious` to always set it to the `invidious_host` var.
- `invidious_host`: The Invidious host to use. Defaults to `yewtu.be`.

### Environment Vars

- `URL_CLEANER_BYPASS_VIP_API_KEY`: The API key used for [bypass.vip](https://bypass.vip). Can be overridden with the `bypass_vip_api_key` var.

### Sets

- `utps`: Universal tracking parameters to remove from all URLs whose RegDomain isn't in the `rd_keep_utps` set. See the `utp_prefixes` for a list of prefixes only used for universal tracking parameters.
- `nh_keep_http`: The `NormalizedHost`s to not upgrade from `http` to `https`.

### Lists

- `utp_prefixes`: Prefixes only used for universal tracking parameters. See the `utps` set for specific query params to always remove.

### Maps


### Partitionings

- `nh_categories`: Categories of similar websites with shared cleaning methods.
- `rd_www_subdomain`: The `RegDomain`s to ensure/remove `www` `Subdomain`s.
- `dm_www_subdomain`: The `DomainMiddle`s to ensure/remove `www` `Subdomain`s.
- `dm_expand_mode`: How to handle redirect `DomainMiddle`s.
- `nh_expand_mode`: How to handle redirect `NormalizedHost`s.
- `rd_expand_mode`: How to handle redirect `RegDomain`s,
- `mobilizer`: Which part at which value makes a `NormalizedHost`/`RegDomain` a mobile URL.

## Job context


### Flags


### Vars


## Task context


### Flags


### Vars

- `unmangle_mode`: The unmangle mode to use.
- `site`: The name of the website used in the contact_info unmangle_mode.
- `text`: The text of the link used in the contact_info unmangle_mode.
