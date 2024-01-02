#!/usr/bin/bash

rm output*

URLS=("https://x.com?a=2" "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id")

cargo build -r

hyperfine -N -n "No urls" -w 5 "../target/release/url-cleaner" --export-json "output--0"
for url in "${URLS[@]}"; do
  echo $url > stdin
  for num in $(seq 3); do
    out="output-$(echo $url | rg / -r=-)-$num"
    hyperfine -N -n "$url - $(cat stdin | wc -l)" -w 5 --input ./stdin "../target/release/url-cleaner" --export-json "$out"

    printf "$(cat stdin)\n%0.s" {1..100} > stdin
  done
done
