# Better URL Bench

A basic benchmarker for [Better URL](../better-url), including comparing it with Servo's URL crate and Ada's URL crate.

## Findings

Please note that Better URL's parser also computes a HostDetails, which provides details for the URL's host.

For domain hosts, this is based on the public suffix of the domain, and thus requires a lookup into the Public Suffix List.

This is used to provide APIs like `BetterUrl::domain_prefix`.

### Parsing

In general:

- Better URL's parser is near universally faster than Servo's parser.

- Better URL's parser is on average an acceptable fraction the speed of Ada's parser.

  - This is due mostly to Better URL having faster percent encoding.

    - If this text is still here and [ada/ada#1230](https://github.com/ada-url/ada/pull/1230) is in a release (basically the ada_url crate has a version above 4.0.0), the below numbers for Ada are wrong.

- The relative performance of Better URL compared to Servo and Ada varies WILDLY depending on the URL.

  - For short URLs, Better URL's parser is significantly slower than Ada's parser.

  - For long URLs, Better URL's parser can be significantly faster than Ada's parser.

  - The inflection point is somewhere around 100 characters.

Using Ada's [`top100.txt`](https://github.com/ada-url/url-various-datasets/blob/main/top100/top100.txt), `parse url --servo --ada --num 10000`, and my personal laptop:

- Better URL's parser averages 2.22x the speed of Servo's parser.

  - With making Servo compute the SchemeDetails and HostDetails, this goes to 2.55x.

- Better URL's parser averages 0.79x the speed of Ada's parser.

  - With making Ada compute the SchemeDetails and HostDetails, this goes to 1.06x.

Using smythp's [reddit links dataset](https://github.com/smythp/reddit_links_dataset) (test.db):

- Better URL's parser averages 1.93x the speed of Servo's parser.

  - With making Servo compute the SchemeDetails and HostDetails, this goes to 2.22x.

- Better URL's parser averages 0.68x the speed of Ada's parser.

  - With making Ada compute the SchemeDetails and HostDetails, this goes to 0.96x.

### Setters

Unsurprisingly Better URL's type based API allows setters to be dramatically faster than Servo and Ada even with the excessive allocations I haven't gotten around to removing.
