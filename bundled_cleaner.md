# The bundled cleaner

The bundled cleaner; Meant for general purpose every day URL Cleaning.

Licensed under the AGPL 3.0 or later.

## Params

### Flags

- `breezewiki`: Change fandom/known Breezewiki hosts to the `breezewiki_host` var.
- `bypass_vip`: Use [bypass.vip](https://bypass.vip) to expand various complicated/otherwise unsupported redirect sites.
- `discord_unexternal`: Change `images-ext-*.discordapp.net` URLs to the original images they refer to.
- `embed_compatibility`: Replace twitter, bluesky, and pixiv hosts with their respective `*_embed_host` vars.
- `furaffinity_sfw`: Change `furaffinity.net` to `sfw.furaffinity.net`.
- `furaffinity_unsfw`: Change `sfw.furaffinity.net` to `furaffinity.net`.
- `instagram_unprofilecard`: Change `instagram.com/username/profilecard` to `instagram.com/username`.
- `invidious`: Change youtube/known Invidious hosts to the `invidious_host` var.
- `keep_affiliate`: Don't remove affiliate info from affiliate links.
- `keep_http`: Disable upgrading `http` URLs to `https`. See the `nh_keep_http` set if you only want to not upgrade specific hosts.
- `mobile`: The inverse of the `unmobile` flag. Sets parts of URLs that tell websites to expect you to be on a mobile device.
- `nitter`: Change twitter/known Nitter hosts `nitter_host` var.
- `no_network`: Don't make any network requests. Some redirect websites will still work because they include the destination in the URL.
- `reddit_new`: Change `old.reddit.com` to `www.reddit.com`.
- `reddit_old`: Change `www.reddit.com` and `new.reddit.com` to `old.reddit.com`
- `remove_unused_search_query`: Remove search queries from URLs that aren't search results (for example, posts).
- `tor2web2tor`: Change `**.onion.**` hosts to `**.onion`.
- `tumblr_unsubdomain_blog`: Change `blog.tumblr.com` to `tumblr.com/blog`.
- `unbreezewiki`: Change known Breezewiki hosts to `fandom.com`.
- `uninvidious`: Change known Invidious hosts to `youtube.com`.
- `unmobile`: Remove parts of URLs that tell websites to expect you to be on a mobile device.
- `unnitter`: Change known Nitter hosts to `x.com`.
- `youtube_remove_sub_confirmation`: Remove the `sub_confirmation` query paramerer in `youtube.com` URLs.
- `youtube_unembed`: Change `youtube.com/embed/abc` to `youtube.com/watch?v=abc`.
- `youtube_unlive`: Change `youtube.com/live/abc` to `youtube.com/watch?v=abc`.
- `youtube_unplaylist`: Remove the `list` query param from `youtube.com/watch` URLs.
- `youtube_unshort`: Change `youtube.com/shorts/abc` to `youtube.com/watch?v=abc`.

### Vars

- `bluesky_embed_host`: The host to use for Bluesky when the `embed_compatibility` flag is set. Defaults to `fxbsky.com`.
- `breezewiki_host`: The host to replace fandom/known Breezewiki hosts with when the `breezewiki` flag is enabled. Defaults to `breezewiki.com`.
- `bypass_vip_api_key`: The API key used for [bypass.vip](https://bypass.vip). Overrides the `URL_CLEANER_BYPASS_VIP_API_KEY` environment var.
- `invidious_host`: The host to replace youtube/known Invidious hosts with when the `invidious` flag is enabled. Defaults to `yewtu.be`.
- `nitter_host`: The host to replace twitter/known Nitter hosts with when the `nitter` flag is enabled. Defaults to `nitter.net`.
- `pixiv_embed_host`: The host to use for pixiv when the `embed_compatibility` flag is set. Defaults to `phixiv.com`.
- `twitter_embed_host`: The host to use for twitter when the `embed_compatibility` flag is set. Defaults to `fixupx.com`.

### Environment Vars

- `URL_CLEANER_BYPASS_VIP_API_KEY`: The API key used for [bypass.vip](https://bypass.vip). Can be overridden with the `bypass_vip_api_key` var.

### Sets

- `nh_keep_http`: The `NormalizedHost`s to not upgrade from `http` to `https`.
- `utps`: Universal tracking parameters to remove from all URLs whose RegDomain isn't in the `rd_keep_utps` set. See the `utp_prefixes` for a list of prefixes only used for universal tracking parameters.

### Lists

- `utp_prefixes`: Prefixes only used for universal tracking parameters. See the `utps` set for specific query params to always remove.

### Maps


### Partitionings

- `dm_expand_mode`: How to handle redirect `DomainMiddle`s.
- `dm_www_subdomain`: The `DomainMiddle`s to ensure/remove `www` `Subdomain`s.
- `mobilizer`: Which part at which value makes a `NormalizedHost`/`RegDomain` a mobile URL.
- `nh_categories`: Categories of similar websites with shared cleaning methods.
- `nh_expand_mode`: How to handle redirect `NormalizedHost`s.
- `rd_expand_mode`: How to handle redirect `RegDomain`s,
- `rd_www_subdomain`: The `RegDomain`s to ensure/remove `www` `Subdomain`s.

## Job context


### Flags


### Vars


## Task context


### Flags


### Vars

- `site`: The name of the website used in the contact_info unmangle_mode.
- `text`: The text of the link used in the contact_info unmangle_mode.
- `unmangle_mode`: The unmangle mode to use.
