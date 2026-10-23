# Better URL Bench

A basic benchmarker for [Better URL](../better-url), including comparing it with Servo's URL crate and Ada's URL crate.

## Findings

Please note that this is comparing against Servo's URL crate (`url`) at version 2.5.8 and Ada's URL crate (`ada-url`) at version 4.0.0.

Presumably future versions of those crates will have optimizations that make the below numbers wrong.

### Parsing

#### URLs

These benchmarks are done with Better URL's `psl` feature enabled, which adds a lookup into the Public Suffix List to every URL with a domain host.

This enables APIs that are very useful in URL Cleaner and I'm honestly not sure how to make Better URL Bench not use that feature given URL Cleaner Engine unconditionally enables it.

If you use Better URL with the `psl` feature disabled, your performance factors should be closer to (but not quite due to Reasons) the "With making XYZ compute the SchemeDetails and HostDetails" numbers.

That, said, in general:

- Better URL's parser is nearly universally faster than Servo's parser.

- Better URL's parser is on average an acceptable fraction the speed of Ada's parser.

  - However, the relative performance varies WILDLY depending on the URL.

    - For short URLs, Better URL's parser is significantly slower than Ada's parser.

    - For long URLs, Better URL's parser can be significantly faster than Ada's parser.

    - The inflection point seems to be around 80 characters.

Using Ada's [`top100.txt`](https://github.com/ada-url/url-various-datasets/blob/main/top100/top100.txt):

- Better URL's parser averages 3.25x the speed of Servo's parser.

  - With making Servo compute the SchemeDetails and HostDetails, this goes to 3.52x.

- Better URL's parser averages 1.19x the speed of Ada's parser.

  - With making Ada compute the SchemeDetails and HostDetails, this goes to 1.49x.

Using Smythp's [reddit links dataset](https://github.com/smythp/reddit_links_dataset) (test.db):

- Better URL's parser averages 2.80x the speed of Servo's parser.

  - With making Servo compute the SchemeDetails and HostDetails, this goes to 3.07x.

- Better URL's parser averages 1.02x the speed of Ada's parser.

  - With making Ada compute the SchemeDetails and HostDetails, this goes to 1.34x.
