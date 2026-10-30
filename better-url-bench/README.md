# Better URL Bench

A basic benchmarker for [Better URL](../better-url), including comparing it with Servo's URL crate and Ada's URL crate.

Licensed under the Affero General Public License V3 or later.

https://www.gnu.org/licenses/agpl-3.0.html

# Findings

Please note that this is comparing against Servo's URL crate (`url`) at version 2.5.8 and Ada's URL crate (`ada-url`) at version 4.0.0.

Presumably future versions of those crates will have optimizations that make the below numbers wrong.

## Parsing URLs

These benchmarks are done with Better URL's `psl` feature enabled, which adds a lookup into the Public Suffix List to every URL with a domain host.

This prevents accidentally misleading users. If you use Better URL with the `psl` feature disabled the below benchmarks will be slightly more in my favor.

That said:

Using Ada's [top100.txt](https://github.com/ada-url/url-various-datasets/blob/main/top100/top100.txt):

|top100  |BURL    |Servo   |Servo+  |Ada     |Ada+    |
|:-------|-------:|-------:|-------:|-------:|-------:|
|Sum     |  16.43s|  55.31s|  59.57s|  20.18s|  25.05s|
|Min     | 13.38µs| 19.43µs| 18.14µs| 49.42µs| 48.15µs|
|Max     |  1.03ms|  6.41ms|  6.48ms|  2.66ms|  2.71ms|
|Mean    |164.22µs|552.89µs|595.54µs|201.76µs|250.40µs|
|Factor  |   1.000|   3.367|   3.627|   1.229|   1.525|

Using Smythp's [reddit links dataset](https://github.com/smythp/reddit_links_dataset) (test.db):

|reddit  |BURL    |Servo   |Servo+  |Ada     |Ada+    |
|:-------|-------:|-------:|-------:|-------:|-------:|
|Sum     | 150.77s| 436.58s| 477.98s| 158.84s| 205.94s|
|Min     | 16.73µs| 24.65µs| 25.79µs| 54.25µs| 53.65µs|
|Max     | 10.37ms| 50.15ms| 50.05ms| 19.60ms| 19.66ms|
|Mean    |150.76µs|436.58µs|477.98µs|158.84µs|205.94µs|
|Factor  |   1.000|   2.896|   3.170|   1.054|   1.366|
