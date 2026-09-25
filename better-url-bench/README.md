# Better URL Bench

A basic benchmarker for [Better URL](../better-url), including comparing it with Servo's URL crate and Ada's URL crate.

## Findings

Please note that this is comparing against Servo's URL crate (`url`) at version 2.5.8 and Ada's URL crate (`ada-url`) at version 4.0.0.

Presumably future versions of those crates will have optimizations that make the below numbers wrong.

### Parsing

#### URLs

Please note that Better URL's parser also computes a HostDetails, which provides details for the URL's host.

For domain hosts, this is based on the public suffix of the domain, and thus requires a lookup into the Public Suffix List.

This is used to provide APIs like `BetterUrl::domain_prefix`, but causes performance overhead that you may or may not want to factor in when comparing with Servo and Ada.

That, said, in general:

- Better URL's parser is nearly universally faster than Servo's parser.

- Better URL's parser is on average an acceptable fraction the speed of Ada's parser.

  - However, the relative performance varies WILDLY depending on the URL.

    - For short URLs, Better URL's parser is significantly slower than Ada's parser.

    - For long URLs, Better URL's parser can be significantly faster than Ada's parser.

    - The inflection point seems to be around 100 characters.

Using Ada's [`top100.txt`](https://github.com/ada-url/url-various-datasets/blob/main/top100/top100.txt), `parse url --servo --ada --num 10000`, and my personal laptop:

- Better URL's parser averages 2.25x the speed of Servo's parser.

  - With making Servo compute the SchemeDetails and HostDetails, this goes to 2.56x.

- Better URL's parser averages 0.91x the speed of Ada's parser.

  - With making Ada compute the SchemeDetails and HostDetails, this goes to 1.16x.

Using smythp's [reddit links dataset](https://github.com/smythp/reddit_links_dataset) (test.db):

- Better URL's parser averages 1.93x the speed of Servo's parser.

  - With making Servo compute the SchemeDetails and HostDetails, this goes to 2.18x.

- Better URL's parser averages 0.77x the speed of Ada's parser.

  - With making Ada compute the SchemeDetails and HostDetails, this goes to 1.04x.

### Setters

Obviously when using Better URL's type based APIs, BetterUrl::set_query is able to use Query's invariant that it's a valid query literal to just use a String::replace.

It's often a *bit* more detailed than that, but the effect is still that Better URL polishes the floor with Servo and Ada.

However, when using plain strings, which I imagine is the most likely case, Better URL is either faster by a much smaller margin or, in some cases, *slower* than Servo and/or Ada.

For example, I found that when setting a URL's path to `abcdef   ghijkl` Better URL is only about 80% as fast as Servo.
Replacing it with `/abcdef   ghijkl`, despite still requiring a percent encoding, makes Better URL slightly faster than Servo.

Additionally, setting part of a part, such as a query parameter or path segment, is going to be much faster (and simpler) because the rest of the part doesn't need to be re-validated.

More detailed numbers are pending me being bothered to do all that.
