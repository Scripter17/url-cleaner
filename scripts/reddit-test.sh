#!/usr/bin/bash

echo "Getting domains"
domains=$(
  cat default-config.json |\
    jq '
      (.. | try select(contains({part: "Domain"}) or contains({part: "Host"})).map | try keys[]),
      (.params.named_partitionings.hwwwwdp_categories[][])
    ' -r |\
    sort -u |\
    grep -Pv '\.(onion|i2p)$'
)

for domain in $domains; do
  echo "Getting data for $domain" > /dev/stderr
  curl --retry 6 --retry-delay 10 -s "https://old.reddit.com/domain/$domain/.json"
  if [ $? -ne 0 ]; then echo "Couldn't get data for $domain. Ignoring"; fi
done | jq -s '[.[] | select(keys | contains(["error"]) | not)]' > reddit-data.json

echo "Extracting inputs"
cat reddit-data.json | jq '
  [
    ["https://reddit.com" + .[].data.children[].data.permalink],
    [.[].data.children[].data.url | gsub("&amp;"; "&") | gsub("%25"; "%")]
  ] | transpose | map({source: .[0], url: .[1]})' > reddit-inputs.json

echo "Cleaning URLs"
cat reddit-inputs.json | jq '.[].url' -r | target/release/url-cleaner --config default-config.json --cache-path reddit-cache.sqlite --json > reddit-outputs.json

echo "Compiling results"
cat reddit-outputs.json |\
  jq --rawfile inputs reddit-inputs.json\
    '
      [
        [$inputs | fromjson | .[].source],
        [$inputs | fromjson | .[].url],
        .Ok.urls
      ] | transpose | map({source: .[0], url: .[1], result: .[2]})
    ' > reddit-results.json
