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

|top100  |BURL    |Servo   |Servo+  |Ada     |Ada+    |
|:-------|-------:|-------:|-------:|-------:|-------:|
|Sum     |  16.54s|  55.74s|  60.09s|  20.44s|  25.31s|
|Min     | 13.50µs| 20.08µs| 18.74µs| 49.64µs| 49.91µs|
|Max     |  1.06ms|  6.51ms|  6.55ms|  2.77ms|  2.83ms|
|Mean    |165.34µs|557.20µs|600.66µs|204.36µs|253.07µs|
|Factor  |   1.000|   3.370|   3.633|   1.236|   1.531|

Using Smythp's [reddit links dataset](https://github.com/smythp/reddit_links_dataset) (test.db):

|reddit  |BURL    |Servo   |Servo+  |Ada     |Ada+    |
|:-------|-------:|-------:|-------:|-------:|-------:|
|Sum     | 153.04s| 444.18s| 486.29s| 162.73s| 210.11s|
|Min     | 16.39µs| 25.03µs| 26.36µs| 54.49µs| 54.21µs|
|Max     | 10.36ms| 49.83ms| 50.22ms| 19.83ms| 19.89ms|
|Mean    |153.04µs|444.18µs|486.29µs|162.72µs|210.11µs|
|Factor  |   1.000|   2.902|   3.178|   1.063|   1.373|
