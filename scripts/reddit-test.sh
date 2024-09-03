#!/usr/bin/bash

for domain in $(grep -oE "\w+(\.\w+)+" ../default-config.json | sort -u); do
  echo "Domain: $domain"
  for url in $(curl --retry 6 --retry-delay 10 -s "https://old.reddit.com/domain/$domain/.json" | jq -s ".[-1].data.children[0:10][].data.url" -r); do
    result=$(../target/release/url-cleaner --config ../default-config.json "$url" --json)
    if echo $result | jq -e ".urls[0].Err" > /dev/null; then
      echo "$url -> $result"
    fi
  done
done
