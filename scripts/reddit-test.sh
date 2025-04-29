#!/usr/bin/bash

MODE=
DOMAINS=( )
PAGES=1
AUTO_DOMAINS=1
GET_DATA=1
EXTRACT_DATA=1

for arg in "$@"; do
  shift
  case "$arg" in
    --pages) MODE=pages ;;
    --domains) MODE=domains; AUTO_DOMAINS=0 ;;
    --reclean) GET_DATA=0; EXTRACT_DATA=0 ;;
    --*) echo '???'; exit 1 ;;
    *) case "$MODE" in
      pages) PAGES=$arg ;;
      domains) DOMAINS=( ${DOMAINS[@]} "$arg" ) ;;
      "") echo "Modal argument without mode"; exit 1 ;;
    esac
  esac
done

if [ $AUTO_DOMAINS -eq 1 ]; then
  echo "Getting domains"
  readarray -t DOMAINS < <(
    cat default-config.json |\
      jq '
        (.. | try select(contains({part: "Domain"}) or contains({part: "Host"})).map | try keys[]),
        (.params.named_partitionings.hwwwwdpafqdnp_categories[][])
      ' -r |\
      sort -u |\
      grep -Pv '\.(onion|i2p)$'
  )
fi

if [ $GET_DATA -eq 1 ]; then
  for domain in "${DOMAINS[@]}"; do
    after=
    for page in $(seq $PAGES); do
      echo "Getting page $page for $domain" > /dev/stderr
      data=$(curl --retry 6 --retry-delay 10 -s "https://old.reddit.com/domain/$domain/.json?after=$after&limit=100" -H 'User-Agent: Firefox')
      after=$(echo "$data" | jq '.data.after' -r)
      echo "$data"
      if [ "$after" == "null" -a $page -ne $PAGES ]; then
        echo "$domain only has $page pages" > /dev/stderr
        break
      fi
    done
  done | tee reddit-data-raw.json | jq -s '[.[] | select(keys | contains(["error"]) | not)]' > reddit-data.json 2>&1
fi

if [ $EXTRACT_DATA -eq 1 ]; then
  echo "Extracting inputs"
  cat reddit-data.json | jq '
    [
      ["https://reddit.com" + .[].data.children[].data.permalink],
      [.[].data.children[].data.url | gsub("&amp;"; "&") | gsub("%25"; "%")]
    ] | transpose | map({source: .[0], url: .[1]}) | unique_by(.url) | sort_by(.url)' > reddit-inputs.json
fi

echo "Cleaning URLs"
cat reddit-inputs.json | jq '.[].url' -r | target/release/url-cleaner --config default-config.json --read-cache false --write-cache false --json > reddit-outputs.json

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
