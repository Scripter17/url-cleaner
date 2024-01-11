#!/usr/bin/bash

rm output*

URLS=("https://x.com?a=2" "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id" "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8")

# cargo build -r --no-default-features --features "http commands cache-redirects stdin default-rules minify-included-strings regex glob"
cargo build -r

for url in "${URLS[@]}"; do
  echo $url -\> $(../target/release/url-cleaner $url)
  for num in $(seq 0 2); do
    yes $url | head -n $((100**$num)) > stdin

    lines=$(cat stdin | wc -l)
    out="output-$(echo $url | rg / -r=-)-$lines"
    hyperfine -N -n "$url - $lines" -w 5 --input ./stdin "../target/release/url-cleaner" --export-json "$out"
  done
done
