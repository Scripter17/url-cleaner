# The default cleaner

The default cleaner is included by in URL Cleaner Engine and is intended for normal everyday use.

The default cleaner is intended to always obey the following rules:

- Cleaning a URL shouldn't cause any "meaningful semantic changes"<sup>[definition?]</sup> when opening the result.
  - Insignificant details like the item categories navbar on amazon listings being slightly different are insignificant.
- URLs that are "semantically valid"<sup>[definition?]</sup> shouldn't ever return an error or become semantically invalid.
  - Basically, if opening a pre-clean URL doesn't return a an error, opening the post-clean URL shouldn't return an error.
- It should always be both deterministic and idempotent.
  - This falls apart the second network connectivity is involved. Exceedingly long redirect chains, netowrk connectivity issues, etc. are allowed to break this intent.
- The `command` and `custom` features, as well as any features starting with `debug` or `experiment` are never expected to be enabled.
  - All other features are expected to be enabled.
    - Some may happen to not be required, but changes that make them required aren't considered breaking.
- When the `no_network` flag is set, the default cleaner should NEVER make ANY network requests.
  - The `no_network` flag should be equivalent to compiling URL Cleaner without network support and removing all network stuff from the default cleaner.

Currently no guarantees are made, though when the above rules are broken it is considered a bug and I'd appreciate being told about it.

Additionally, these rules may be changed at any time for any reason. Usually just for clarification.

## Params

Cleaners use params to customize what exactly actions do, such as letting you choose whether or not to change `x.com` URLs to `vxtwitter.com` URLs.

The default cleaner contains many flags and vars to control behavior I often want but don't want to make default. If there's a reasonably common task you sometimes want to do, I may be willing to integrate it into the default cleaner.

And yes I know the environment vars section shouldn't be listed under params. I don't want to refactor the script to generate markdown from cleaner docs.

<!--cmd scripts/gen-docs.py-->
### Flags

- `bypass_vip`: Use [bypass.vip](https://bypass.vip) to expand linkvertise and some other links.
- `embed_compatibility`: Sets the domain of twitter domiains (and supported twitter redirects like `vxtwitter.com`) to the variable `twitter_embed_host` and `bsky.app` to the variable `bsky_embed_host`.
- `keep_lang`: Keeps language query parameters.
- `no_https_upgrade`: Disable upgrading `http` URLs to `https`.
- `no_network`: Don't make any HTTP requests. Some redirect websites will still work because they include the destination in the URL.
- `remove_unused_search_query`: Remove search queries from URLs that aren't search results (for example, posts).
- `tor2web2tor`: Replace `**.onion.**` domains with `**.onion` domains.
- `unmobile`: Convert `https://m.example.com`, `https://mobile.example.com`, `https://abc.m.example.com`, and `https://abc.mobile.example.com` into `https://example.com` and `https://abc.example.com`.
- `breezewiki`: Replace fandom/known Breezewiki hosts with the `breezewiki_host` variable.
- `unbreezewiki`: Replace Breezewiki hosts with fandom.com.
- `invidious`: Replace youtube/known Invidious hosts with the `invidious_host` variabel.
- `uninvidious`: Replace Invidious hosts with youtube.com
- `nitter`: Replace twitter/known Nitter hosts with the `nitter_host` variable.
- `unnitter`: Replace Nitter hosts with x.com.
- `discord_unexternal`: Replace `images-ext-1.discordapp.net` with the original images they refer to.
- `furaffinity_sfw`: Turn `furaffinity.net` into `sfw.furaffinity.net`
- `furaffinity_unsfw`: Turn `sfw.furaffinity.net` into `furaffinity.net`
- `instagram_unprofilecard`: Turns `https://instagram.com/username/profilecard` into `https://instagram.com/username`.
- `tumblr_unsubdomain_blog`: Changes `blog.tumblr.com` URLs to `tumblr.com/blog` URLs. Doesn't move `at` or `www` subdomains.
- `youtube_keep_sub_confirmation`: Don't remove the `sub_confirmation` query param from youtube.com URLs.
- `youtube_unembed`: Turns `https://youtube.com/embed/abc` into `https://youtube.com/watch?v=abc`.
- `youtube_unlive`: Turns `https://youtube.com/live/abc` into `https://youtube.com/watch?v=abc`.
- `youtube_unplaylist`: Removes the `list` query parameter from `https://youtube.com/watch` URLs.
- `youtube_unshort`: Turns `https://youtube.com/shorts/abc` into `https://youtube.com/watch?v=abc`.

### Vars

- `bluesky_embed_host`: The domain to use for bluesky when the `embed_compatibility` flag is set. Defaults to `fxbsky.com`.
- `breezewiki_host`: The domain to replace fandom/Breezewiki domains with when the `breezewiki` flag is enabled
- `bypass_vip_api_key`: The API key used for [bypass.vip](https://bypass.vip)'s premium backend. Overrides the `URL_CLEANER_BYPASS_VIP_API_KEY` environment variable.
- `invidious_host`: The domain to replace twitter/Invidious domains with when the `invidious` flag is enabled
- `nitter_host`: The domain to replace twitter/nitter domains with when the `nitter` flag is enabled
- `pixiv_embed_host`: The domain to use for pixiv when the `embed_compatibility` flag is set. Defaults to `phixiv.com`.
- `twitter_embed_host`: The domain to use for twitter when the `embed_compatibility` flag is set. Defaults to `vxtwitter.com`.

### Environment Vars

- `URL_CLEANER_BYPASS_VIP_API_KEY`: The API key used for [bypass.vip](https://bypass.vip)'s premium backend. Can be overridden with the `bypass_vip_api_key` variable.

### Sets

- `bypass_vip_hwwwwdpafqdnps`: The `HostWithoutWWWDotPrefixAndFqdnPeriod`es of websites bypass.vip can expand.
- `email_link_format_1_hosts`: (TEMPORARY NAME) Hosts that use unknown link format 1.
- `https_upgrade_host_blacklist`: Hosts to never upgrade from `http` to `https`.
- `redirect_hwwwwdpafqdnps`: Hosts that are considered redirects in the sense that they return HTTP 3xx status codes. URLs with hosts in this set (as well as URLs with hosts that are "www." then a host in this set) will have the `ExpandRedirect` action applied.
- `remove_empty_fragment_reg_domain_blacklist`: The RegDomains to not remove an empty fragment (the #stuff at the end (but specifically just a #)) from.
- `remove_empty_query_reg_domain_blacklist`: The RegDomains to not remove an empty query from.
- `remove_fqdn_period_reg_domain_blacklist`: The RegDomains to not remove remove the [fully qualified domain](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period from.
- `unmobile_reg_domain_blacklist`: Effectively unsets the `unmobile` flag for the specified `RegDomain`s.
- `utps`: The set of "universal tracking parameters" that are always removed for any URL with a host not in the `utp_host_whitelist` set. Please note that, in addition to all values in this set, any value starting with a value in the `utp_prefixes` set are also removed.
- `utps_reg_domain_whitelist`: RegDomains to never remove universal tracking parameters from.

### Lists

- `utp_prefixes`: If a query parameter starts with any of the strings in this list (such as `utm_`) it is removed.

### Maps

- `hwwwwdpafqdnp_lang_query_params`: The name of the `HostWithoutWWWDotPrefixAndFqdnPeriod`'s language query parameter.

### Named Partitionings

- `hwwwwdpafqdnp_categories`: Categories of similar websites with shared cleaning methods.
- `www_subdomain_handling`: What to do instead of removing `www` subdomains.
- `domain_middle_expand_mode`: How to handle redirect `DomainMiddle`s,
- `hwwwwdpafqdnp_expand_mode`: How to handle redirect `HostWithoutWWWDotPrefix`s.
- `reg_domain_expand_mode`: How to handle redirect `RegDomain`s,

### Job Context

#### Vars

- `SOURCE_HOST`: The `Host` of the "source" of the jobs. Usually the webpage it came from.
- `SOURCE_REG_DOMAIN`: The `RegDomain` of the "source" of the jobs, Usually the webpage it came from.

### Task Context

#### Vars

- `bsky_handle`: The handle of an `@user.bsky.social`, used to replace the `/did:plc:12345678` in the URL with the actual handle.
- `faci_site_name`: For furaffinity contact info links, the name of the website the contact info is for. Used for unmangling.
- `link_text`: The text of the link the job came from.
- `redirect_shortcut`: For links that use redirect sites but have the final URL in the link's text/title/whatever, this is used to avoid sending that HTTP request.
- `twitter_handle`: The handle of the twitter user for a `/i/web/status/` and `/i/user/` twitter URLs.
<!--/cmd-->
