# URL Cleaner's threat model

VERY work in progress. I've never made a serious attempt at writing a threat model before.

Currently just a series of known possible issues.


# Redirect shortcuts

Some websites have redirect URLs that look like `https://example.com/REDIRECT_ID?destinatio=...`. For these websites, the default config currently uses the `destination` query parameter to get the final URL without sending an HTTP request.

This introduces the possibility of people changing the query to send people who don't use URL Cleaner and people who do to different destinations.

This can allow malicious emails/social media posts/messages to bypass spam/mallink filters for people known to use URL Cleaner.

## Next steps

- Make the shortcut optional. I'm leaning towards it being opt-out since it's such a speedup and sending HTTP requests is such a leak.
  - It's probably fine to always use twitter's alt text containing the destination?


# How obvious should URL Cleaner be?

Currently the default config makes no effort at hiding the fact you're using URL Cleaner. An extreme example of this is trimming amazon product listings from multi-kilobyte keyspam to just `https://amamzon.com/dp/PRODUCT_ID`.

It's likely not possible for real-world users to maintain ambiguity as to whether or not they're cleaning URLs, and it's unclear how bad not bothering hiding it is.

In the specific case of amazon product listing, normal cleaning already strips out so much that we may as well also minimize away the product name.

I think the default config should aim more towards normalization than minimization when there's a distinction to be made.

## Next steps

- Research how other projects normalize URLs and try to blend in with them.


# Caching

When sending an HTTP request, for example to expand a redirect, the result is cached and future cleanings of that URL use the cached value.

If a website knows you use URL Cleaner and knows when you send a request to the redirect URL, this can let it know you've seen that redirect before.

This can allow correlation attacks between pseudoidentities because the default config and cache don't support partitioning the cache in that way.

If a cached mapper returns an error, the mapper is re-run every subsequent time the URL is encountered. This allows `https://t.co/invalid-link` to always get an HTTP request.

## Next steps

- Support partitioning the cache. Probably enabled with a flag.
  - How should the partitions be decided? Definitely per RegDomain, but additional partitioning is required for things like mutliple browser profiles and container tabs. Maybe URL Cleaner Site's userscript can get some unique "parition ID" from the web browser?
- Look into shared caches?
  - Probably not possible without some PoW bullshit to avoid malicious inputs.
    - Can I leverage certificate trust chains, given most of this is for HTTP redirects?
  - Probably shouldn't be tied into URL Cleaner and instead a separate tool that just happens to work well with URL Cleaner.
- Make a tool that imports 301works lists to URL Cleaner's cache schema.
