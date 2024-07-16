#!/usr/bin/bash

URLS=(\
  "https://x.com?a=2"\
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id"\
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8"\
)
NUMS=(0 1 10 100 1000 10000)

rm -f hyperfine* callgrind* cachegrind* massif* dhat* memcheck*

no_compile=false
json=false
no_hyperfine=false
print_desmos_lists=false
no_valgrind=false
no_callgrind=false
no_cachegrind=false
no_massif=false
no_dhat=false
no_memcheck=false

for arg in "$@"; do
  shift
  case "$arg" in
    "--no-compile") no_compile=true ;;
    "--json") json=true ;;
    "--no-hyperfine") no_hyperfine=true ;;
    "--print-desmos-lists") print_desmos_lists=true ;;
    "--no-valgrind") no_valgrind=true ;;
    "--no-callgrind") no_callgrind=true ;;
    "--no-cachegrind") no_cachegrind=true ;;
    "--no-massif") no_massif=true ;;
    "--no-dhat") no_dhat=true ;;
    "--no-memcheck") no_memcheck=true ;;
    *) echo Unknwon option \"$arg\" && exit 1 ;;
  esac
done

if [ "$json" == "true" ]; then
  COMMAND="../target/release/url-cleaner --config ../default-config.json --json"
else
  COMMAND="../target/release/url-cleaner --config ../default-config.json"
fi

if [ "$no_compile" == "false" ]; then cargo build -r --config profile.release.strip=false --config profile.release.debug=2; fi

if [ $? -ne 0 ]; then exit; fi

for url in "${URLS[@]}"; do
  echo IN: $url
  echo OUT: $($COMMAND "$url")
  file_safe_in_url=$(echo $url | head -c 50 | sed "s/\//-/g")
  if [ "$no_hyperfine" == "false" ]; then
    touch stdin
    hyperfine\
      -L url "$url"\
      -L num $(echo "${NUMS[@]}" | sed "s/ /,/g")\
      --prepare "bash -c \"yes '$url' | head -n {num} > stdin\""\
      --max-runs 100\
      --warmup 20\
      --input stdin\
      -N\
      "$COMMAND"\
      --export-json "hyperfine-$file_safe_in_url.json"
    rm stdin
    if [ "$print_desmos_lists" == "true" ]; then
      echo "N=[$(echo "${NUMS[@]}" | sed "s/ /,/g")]"
      echo -n T= && cat "hyperfine-$file_safe_in_url.json" | jq "[.results[].mean]" -c
    fi
  fi
  if [ "$no_valgrind" == "false" ]; then
    for num in "${NUMS[@]}"; do
      if [ "$no_callgrind" = "false" ]; then
        echo "Callgrind  - $num"
        yes "$url" | head -n $num | valgrind --quiet --tool=callgrind --callgrind-out-file="callgrind.out-$file_safe_in_url-$num-%p"  $COMMAND > /dev/null
      fi
      if [ "$no_cachegrind" = "false" ]; then
        echo "Cachegrind - $num"
        yes "$url" | head -n $num | valgrind --quiet --tool=cachegrind --cachegrind-out-file="cachegrind.out-$file_safe_in_url-$num-%p" $COMMAND > /dev/null
      fi
      if [ "$no_massif" = "false" ]; then
        echo "Massif     - $num"
        yes "$url" | head -n $num | valgrind --quiet --tool=massif --massif-out-file="massif.out-$file_safe_in_url-$num-%p" $COMMAND > /dev/null
      fi
      if [ "$no_dhat" = "false" ]; then
        echo "Dhat       - $num"
        yes "$url" | head -n $num | valgrind --quiet --tool=dhat --dhat-out-file="dhat.out-$file_safe_in_url-$num-%p" $COMMAND > /dev/null
      fi
      if [ "$no_memcheck" = "false" ]; then
        echo "Memcheck   - $num"
        yes "$url" | head -n $num | valgrind --quiet --tool=memcheck $COMMAND > /dev/null 2> "memcheck.out-$file_safe_in_url-$num"
      fi
    done
  fi
done
