# The default cleaner

The default cleaner is included by in URL Cleaner Engine and is intended for normal everyday use.

The default cleaner is intended to always obey the following rules:

- Cleaning a URL shouldn't cause any "meaningful semantic changes"<sup>[definition?]</sup> when opening the result.
  - Insignificant details like the item categories navbar on amazon listings being slightly different are insignificant.
- URLs that are "semantically valid"<sup>[definition?]</sup> shouldn't ever return an error or become semantically invalid.
  - Basically, if opening a pre-clean URL doesn't return a an error, opening the post-clean URL shouldn't return an error.
- It should always be both deterministic and idempotent.
  - This falls apart the second network connectivity is involved. Exceedingly long redirect chains, netowrk connectivity issues, etc. are allowed to break this intent.
- Shuffling the input list of URLs should always give the same (equally shuffled) output.
- Only the `default-cleaner` feature is expected to be enabled, and it is always expected to be enabled.
- When the `no_network` flag is set, the default cleaner should NEVER make ANY network requests.
  - The `no_network` flag should be equivalent to compiling URL Cleaner without network support and removing all network stuff from the default cleaner.
- Opening a URL outputted by the default cleaner with the `no_network` flag disabled and the `bypass_vip`/any future related flags enebled should never result in a redirect.

Currently no guarantees are made, though when the above rules are broken it is considered a bug and I'd appreciate being told about it.

Additionally, these rules may be changed at any time for any reason. Usually just for clarification.

## Params

Cleaners use params to customize what exactly actions do, such as letting you choose whether or not to change `x.com` URLs to `vxtwitter.com` URLs.

The default cleaner contains many flags and vars to control behavior I often want but don't want to make default. If there's a reasonably common task you sometimes want to do, I may be willing to integrate it into the default cleaner.

And yes I know the environment vars section shouldn't be listed under params. I don't want to refactor the script to generate markdown from cleaner docs.

<!--cmd scripts/gen-docs.py-->
### Flags

- `keep_affiliate`: Don't remove affiliate info from affiliate links.
- `bypass_vip`: Use [bypass.vip](https://bypass.vip) to expand various complicated/otherwise unsupported redirect sites.
- `embed_compatibility`: Replace twitter, bluesky, and pixiv hosts with their respective `*_embed_host` vars.
- `keep_http`: Disable upgrading `http` URLs to `https`. See the `nh_keep_http` set if you only want to not upgrade specific hosts.
- `no_network`: Don't make any network requests. Some redirect websites will still work because they include the destination in the URL.
- `expand_dangerous_redirects`: Expand redirects known to always leak sender info.
- `remove_unused_search_query`: Remove search queries from URLs that aren't search results (for example, posts).
- `tor2web2tor`: Change `**.onion.**` hosts to `**.onion`.
- `unmobile`: Remove `m` and `mobile` segments from the subdomain. Should be expanded to query params.
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
- `tumblr_unsubdomain_blog`: Change `blog.tumblr.com` to `tumblr.com/blog`.
- `youtube_remove_sub_confirmation`: Remove the `sub_confirmation` query paramerer in `youtube.com` URLs.
- `youtube_unembed`: Change `youtube.com/embed/abc` to `youtube.com/watch?v=abc`.
- `youtube_unlive`: Change `youtube.com/live/abc` to `youtube.com/watch?v=abc`.
- `youtube_unshort`: Change `youtube.com/shorts/abc` to `youtube.com/watch?v=abc`.
- `youtube_unplaylist`: Remove the `list` query param from `youtube.com/watch` URLs.

### Vars

- `bluesky_embed_host`: The host to use for Bluesky when the `embed_compatibility` flag is set. Defaults to `fxbsky.com`.
- `pixiv_embed_host`: The host to use for pixiv when the `embed_compatibility` flag is set. Defaults to `phixiv.com`.
- `twitter_embed_host`: The host to use for twitter when the `embed_compatibility` flag is set. Defaults to `vxtwitter.com`.
- `breezewiki_host`: The host to replace fandom/known Breezewiki hosts with when the `breezewiki` flag is enabled. Defaults to `breezewiki.com`.
- `invidious_host`: The host to replace youtube/known Invidious hosts with when the `invidious` flag is enabled. Defaults to `yewtu.be`.
- `nitter_host`: The host to replace twitter/known Nitter hosts with when the `nitter` flag is enabled. Defaults to `nitter.net`.
- `bypass_vip_api_key`: The API key used for [bypass.vip](https://bypass.vip). Overrides the `URL_CLEANER_BYPASS_VIP_API_KEY` environment var.

### Environment Vars

- `URL_CLEANER_BYPASS_VIP_API_KEY`: The API key used for [bypass.vip](https://bypass.vip). Can be overridden with the `bypass_vip_api_key` var.

### Sets

- `utps`: Universal tracking parameters to remove from all URLs whose RegDomain isn't in the `rd_keep_utps` set. See the `utp_prefixes` for a list of prefixes only used for universal tracking parameters.
- `nh_keep_http`: The `NormalizedHost`s to not upgrade from `http` to `https`.
- `rd_keep_mobile`: The `RegDomain`s to not apply the `unmobile` flag to.

### Lists

- `utp_prefixes`: Prefixes only used for universal tracking parameters. See the `utps` set for specific query params to always remove.

### Named Partitionings

- `nh_categories`: Categories of similar websites with shared cleaning methods.
- `rd_www_subdomain`: The `RegDomain`s to ensure/remove `www` `Subdomain`s.
- `dm_www_subdomain`: The `DomainMiddle`s to ensure/remove `www` `Subdomain`s.
- `dm_expand_mode`: How to handle redirect `DomainMiddle`s.
- `nh_expand_mode`: How to handle redirect `NormalizedHost`s.
- `rd_expand_mode`: How to handle redirect `RegDomain`s,

### Task Context

#### Vars

- `handle`: The handle to replace the user ID with.
- `contact_info_site_name`: The name oe the website this URL is contact info for.
- `link_text`: The text of the link the job came from.
- `redirect_shortcut`: The destination of a redirect as specified by some part of the source. For example, the link's text.
<!--/cmd-->
