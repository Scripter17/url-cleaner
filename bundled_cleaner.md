# The Bundled Cleaner

The Bundled Cleaner is included by in URL Cleaner Engine and is intended for normal everyday use.

## Performance

See [benchmarks.md](benchmarks.md) for performance information.

## Params

Cleaners use params to customize what exactly actions do, such as letting you choose whether or not to change `x.com` URLs to `vxtwitter.com` URLs.

The Bundled Cleaner contains many flags and vars to control behavior I often want but don't want to make default. If there's a reasonably common task you sometimes want to do, I may be willing to integrate it into the default cleaner.

And yes I know the environment vars section shouldn't be listed under params. I don't want to refactor the script to generate markdown from cleaner docs.

<!--cmd scripts/gen-docs.py-->
### Flags

- `keep_affiliate`: Don't remove affiliate info from affiliate links.
- `bypass_vip`: Use [bypass.vip](https://bypass.vip) to expand various complicated/otherwise unsupported redirect sites.
- `embed_compatibility`: Replace twitter, bluesky, and pixiv hosts with their respective `*_embed_host` vars.
- `keep_http`: Disable upgrading `http` URLs to `https`. See the `nh_keep_http` set if you only want to not upgrade specific hosts.
- `no_network`: Don't make any network requests. Some redirect websites will still work because they include the destination in the URL.
- `remove_unused_search_query`: Remove search queries from URLs that aren't search results (for example, posts).
- `tor2web2tor`: Change `**.onion.**` hosts to `**.onion`.
- `unmobile`: Remove parts of URLs that tell websites to expect you to be on a mobile device.
- `mobile`: The inverse of the `unmobile` flag. Sets parts of URLs that tell websites to expect you to be on a mobile device.
- `breezewiki`: Change fandom/known Breezewiki hosts to the `breezewiki_host` var.
- `unbreezewiki`: Change known Breezewiki hosts to `fandom.com`.
- `invidious`: Change youtube/known Invidious hosts to the `invidious_host` var.
- `uninvidious`: Change known Invidious hosts to `youtube.com`.
- `nitter`: Change twitter/known Nitter hosts `nitter_host` var.
- `unnitter`: Change known Nitter hosts to `x.com`.
- `discord_unexternal`: Change `images-ext-*.discordapp.net` URLs to the original images they refer to.
- `furaffinity_sfw`: Change `furaffinity.net` to `sfw.furaffinity.net`.
- `furaffinity_unsfw`: Change `sfw.furaffinity.net` to `furaffinity.net`.
- `instagram_unprofilecard`: Change `instagram.com/username/profilecard` to `instagram.com/username`.
- `reddit_new`: Change `old.reddit.com` to `www.reddit.com`.
- `reddit_old`: Change `www.reddit.com` and `new.reddit.com` to `old.reddit.com`
- `tumblr_unsubdomain_blog`: Change `blog.tumblr.com` to `tumblr.com/blog`.
- `youtube_remove_sub_confirmation`: Remove the `sub_confirmation` query paramerer in `youtube.com` URLs.
- `youtube_unembed`: Change `youtube.com/embed/abc` to `youtube.com/watch?v=abc`.
- `youtube_unlive`: Change `youtube.com/live/abc` to `youtube.com/watch?v=abc`.
- `youtube_unshort`: Change `youtube.com/shorts/abc` to `youtube.com/watch?v=abc`.
- `youtube_unplaylist`: Remove the `list` query param from `youtube.com/watch` URLs.

### Vars

- `bluesky_embed_host`: The host to use for Bluesky when the `embed_compatibility` flag is set. Defaults to `fxbsky.com`.
- `pixiv_embed_host`: The host to use for pixiv when the `embed_compatibility` flag is set. Defaults to `phixiv.com`.
- `twitter_embed_host`: The host to use for twitter when the `embed_compatibility` flag is set. Defaults to `fixupx.com`.
- `breezewiki_host`: The host to replace fandom/known Breezewiki hosts with when the `breezewiki` flag is enabled. Defaults to `breezewiki.com`.
- `invidious_host`: The host to replace youtube/known Invidious hosts with when the `invidious` flag is enabled. Defaults to `yewtu.be`.
- `nitter_host`: The host to replace twitter/known Nitter hosts with when the `nitter` flag is enabled. Defaults to `nitter.net`.
- `bypass_vip_api_key`: The API key used for [bypass.vip](https://bypass.vip). Overrides the `URL_CLEANER_BYPASS_VIP_API_KEY` environment var.

### Environment Vars

- `URL_CLEANER_BYPASS_VIP_API_KEY`: The API key used for [bypass.vip](https://bypass.vip). Can be overridden with the `bypass_vip_api_key` var.

### Sets

- `utps`: Universal tracking parameters to remove from all URLs whose RegDomain isn't in the `rd_keep_utps` set. See the `utp_prefixes` for a list of prefixes only used for universal tracking parameters.
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

### Task Context

#### Vars

- `contact_info_site_name`: The name oe the website this URL is contact info for.
- `link_text`: The text of the link the job came from.
<!--/cmd-->
