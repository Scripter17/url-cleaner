# Better URL

A URL crate that's better than the one provided by Servo.

Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)

https://www.gnu.org/licenses/agpl-3.0.html

## Performance

According to [`urlc-tool bench url parse --servo --ada --num 10000`](../urlc-tool) with Ada's [`top100.txt`](https://github.com/ada-url/url-various-datasets/blob/main/top100/top100.txt), Better URL is about twice as fast as [Servo's URL crate](https://docs.rs/url/latest/url/) and within a few percent the speed of [Ada's URL crate](https://docs.rs/ada-url/latest/ada_url/index.html).

This has three main caveats:

1. Better URL's type based API allows you to modify existing URLs much cheaper.
   Setting a query parameter doesn't require revalidating the entire query.

2. Both Servo's and Ada's URL crate are intentionally handicapped by being forced to compute Better URL's `SchemeDetails` and `HostDetails` types.
   This is to ensure a fair comparison since `HostDetails` is required to provide Better URL's domain parts API.
   I promise the fact this handicap benefits me is incidental, but you can pass `--no-adjust` if this is an issue.

3. The speed factor varies wildly depending on the URL.
   Compared to Ada, Better URL's speed can get as low as 0.5x or as high as 2x.
   In general it seems Better URL is significantly slower than Ada for short URLs but significantly faster for long URLs.
