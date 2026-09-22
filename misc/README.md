# Miscelanious stuff

## `www-data.txt`

[`www-data.txt`](www-data.txt) is a TSV with columns of the domain origin, what happens when getting `https://{domain_origin}`, and what happens when getting `https://www.{domain_origin}`.

Basically if the second or third column is `Swap`, `ClientError`, `ServerError`, or `NetworkError` and the other is `Stay`, that indicates the domain origin should have an entry in the Bundled Cleaner's `do_www_prefix` partitioning.

It includes every domain origin from [`../better-url-bench/data`](../better-url-bench/data).

Included here because it is SUPER annoying to wait for `urlc-tool tasks www` to finish. Don't expect it to be kept up to date.
