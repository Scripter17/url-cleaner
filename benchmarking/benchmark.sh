#!/usr/bin/bash

cd $(dirname "$0")

if [ -t 0 ]; then
  URLS=(\
    "https://x.com?a=2"\
    "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id"\
    "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8"\
  )
else
  URLS=( )
fi
NUMS=( 0 1 10 100 1000 10000 )

if ls | grep -qF '.out'; then
  rm *.out*
fi

compile=1
hyperfine=1
callgrind=0
massif=0
ignore_failure=

mode=

features=

out="benchmarks-$(date +%s).tar.gz"

for arg in "$@"; do
  shift
  case "$arg" in
    --no-compile)      mode=        ; compile=0                          ;;
    --only-compile)    mode=        ; hyperfine=0; callgrind=0; massif=0 ;;
    --no-hyperfine)    mode=        ; hyperfine=0                        ;;
    --callgrind)       mode=        ; callgrind=1                        ;;
    --massif)          mode=        ; massif=1                           ;;
    --ignore-failure)  mode=        ; ignore_failure=--ignore-failure    ;;
    --all)             mode=        ; hyperfine=1; callgrind=1; massif=1 ;;
    --urls)            mode=urls    ; URLS=( )                           ;;
    --nums)            mode=nums    ; NUMS=( )                           ;;
    --features)        mode=features                                     ;;
    --out)             mode=out                                          ;;
    --)                break ;;
    --*)               echo Unknown option \"$arg\"; exit 1 ;;
    *) case "$mode" in
      urls) URLS=( ${URLS[@]} "$arg" ) ;;
      nums) NUMS=( ${NUMS[@]} "$arg" ) ;;
      features) features=(--features "$arg") ;;
      out) out="$arg" ;;
      "") echo "Modal argument without mode"; exit 1 ;;
    esac
  esac
done

if [ ! -t 0 ]; then
  readarray -t stdin_urls < /dev/stdin
  URLS=( "${URLS[@]}" "${stdin_urls[@]}" )
fi

if [  $hyperfine -eq 1                   ] && ! which -s hyperfine; then echo 'Hyperfine not found; Please run `cargo install hyperfine`.'                         ; exit 2; fi
if [  $hyperfine -eq 1                   ] && ! which -s jq       ; then echo 'Jq not found; Please install it. Also please learn it it'"'"'s a really handy tool.'; exit 2; fi
if [  $hyperfine -eq 1                   ] && ! which -s bat      ; then echo 'Bat not found; Please run `cargo install bat`.'                                     ; exit 2; fi
if [[ $callgrind -eq 1 || $massif -eq 1 ]] && ! which -s valgrind ; then echo 'Valgrind not found; Please install it.'                                             ; exit 2; fi

if [ $compile -eq 1 ]; then
  cargo build -r ${features[@]} --config profile.release.strip=false --config profile.release.debug=2
  if [ $? -ne 0 ]; then exit 3; fi
fi

COMMAND="../target/release/url-cleaner --cleaner ../default-cleaner.json $@"

if [ $hyperfine -eq 1 ]; then
  touch stdin
  hyperfine \
    -L num $(IFS=, ; echo "${NUMS[*]}") \
    -L url $(IFS=, ; echo "${URLS[*]}") \
    --prepare "bash -c \"yes '{url}' | head -n {num} > stdin\"" \
    --max-runs 100 \
    --warmup 20 \
    --input stdin \
    -N \
    "$COMMAND" \
    --sort command \
    --export-json "hyperfine.out.json" \
    --command-name ""\
    $ignore_failure
  rm stdin
  ql=$(cat hyperfine.out.json | grep -oP '(?<="num": ")\d+' | wc -L)
  pl=$(cat hyperfine.out.json | jq '.results[].mean * 1000 | floor' | wc -L)
  cat hyperfine.out.json |\
    jq 'reduce .results[] as $result ({}; .[$result.parameters.url][$result.parameters.num] = ($result.mean * 1000000 | floor / 1000 | tonumber))' |\
    sed -E "/^    / {\
      /\./! s/(,?)$/.\1/;\
      :a s/(\.[0123456789]{0,2})(,?$)/\10\2/; ta\
      :b s/( \".{0,$ql}):/\1 :/; tb\
      :c s/:(.{0,$pl}\.)/: \1/; tc\
    }" |\
    tee hyperfine.out-summary.json |\
    bat -ppl json
fi

for url in "${URLS[@]}"; do
  file_safe_url=$(echo $url | head -c 50 | sed "s/\//-/g")
  for num in "${NUMS[@]}"; do
    if [ $callgrind -eq 1 ]; then echo "Callgrind - $num - $url"; yes "$url" | head -n $num | valgrind --tool=callgrind --separate-threads=yes --callgrind-out-file="callgrind.out-$file_safe_url-$num-%p" $COMMAND &> /dev/null; fi
    if [ $massif    -eq 1 ]; then echo "Massif    - $num - $url"; yes "$url" | head -n $num | valgrind --tool=massif                           --massif-out-file="massif.out-$file_safe_url-$num-%p"       $COMMAND &> /dev/null; fi
  done
done

if ls | grep -qF '.out'; then
  tar -czf $out *.out*
  echo "Benchmark details compiled and compressed into $out"
else
  echo "No benchmark details generated."
fi
