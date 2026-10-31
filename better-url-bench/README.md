# Better URL Bench

A basic benchmarker for [Better URL](../better-url), including comparing it with Servo's URL crate and Ada's URL crate.

Licensed under the Affero General Public License V3 or later.

https://www.gnu.org/licenses/agpl-3.0.html



# Findings

Please note that this is comparing against Servo's URL crate (`url`) at version 2.5.8 and Ada's URL crate (`ada-url`) at version 4.0.0.

Presumably future versions of those crates will have optimizations that make the below numbers wrong.

## Parsing URLs

On the same laptop I do [URL Cleaner's benchmarks](../benchmarks.md) on, parsing each URL in each dataset 1000 times gives these results:

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

## Methodology

- For each datapoint, URLs are repeatedly parsed in groups of 1000 (a "subdatapoint") until the 8 most recent groups of 1000 are all within 5% of each other.

  - The datapoint returned is the average of those 8 most recent subdatapoints.

  - This mostly eliminates noise where background processes on my computer cause random hiccups.

  - Datapoints are calculated individually for each URL and for each parser.

    - A more complete breakdown of timing info is generated to let me diagnose performance issues, but for practicality only summarized tables are shown above.

## Columns

- BURL refers to Better URL, specifically with the `psl` feature enabled.

  - The `psl` feature makes ever URL with a domain host do a lookup into the [Public Suffix List](https://publicsuffix.org/) and allows for APIs like `BetterUrl::domain_origin`.

- Servo and Ada refer to Servo's URL crate and Ada's URL crate with zero additions.

- Servo+ and Ada+ refer to Servo's URL crate and Ada's URL crate "plus" Better URL's `DomainHostDetails::parse_unchecked`.

  - This should give a reasonably accurate simulation of the slowdown they would have if PSL support was added to them.

## Rows

- "Sum" refers to the sum of each parser's datapoints.

- "Min" and "Max" refer to the minimum and maximum values of each parser's datapoints.

- "Mean" refers to the average amount of time for each parser's datapoints.

- "Factor" refers to how much slower each parser is than Better URL by dividing their "Sum" rows.

  - A number above 1 means Better URL is faster and a number below 1 means Better URL is slower.
