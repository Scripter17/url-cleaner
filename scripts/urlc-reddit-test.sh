#!/usr/bin/bash

for domain in $(grep -oE "\w+\.\w+" ../default-config.json | sort -u); do
  echo "Domain: $domain"
  for url in $(curl --retry 6 --retry-delay 10 -s "https://old.reddit.com/domain/$domain/.json" | jq ".data.children[0:10][].data.url" -r); do
    echo $url
    ../target/release/url-cleaner --config ../default-config.json "$url"
    echo
  done
done
