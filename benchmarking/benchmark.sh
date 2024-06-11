#!/usr/bin/bash

rm -f hyperfine* callgrind*

URLS=("https://x.com?a=2" "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id" "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8")
COUNTS=(1 100 10000)
COMMAND="../target/release/url-cleaner --config ../default-config.json"

# cargo build -r --config profile.release.strip=false --config profile.release.debug=2

if [ $? -ne 0 ]; then exit; fi

measure () {
  url=$1
  count=$2
  out=$(echo -n "$url" | head -c 50 | sed "s/\//-/g" && echo -n "-$count")

  yes $url | head -n $count > stdin

  if [ $count -eq 0 ]; then
    hyperfine -N -n "$url - $count" -w 10 "$COMMAND" --export-json "hyperfine-$out.txt"
    valgrind --quiet --tool=callgrind $COMMAND > /dev/null
  else
    hyperfine -N -n "$url - $count" -w 10 --input ./stdin "$COMMAND" --export-json "hyperfine-$out.txt"
    cat stdin | valgrind --quiet --tool=callgrind $COMMAND > /dev/null
  fi
  mv callgrind.out.* "callgrind.out-$out"
}

measure "" 0
for url in "${URLS[@]}"; do
  echo IN: $url
  echo OUT: $($COMMAND "$url")
  for count in "${COUNTS[@]}"; do
    measure "$url" $count
  done
done
