#!/usr/bin/bash

cd $(dirname "$0")

URLS=(\
  "https://x.com?a=2"\
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id"\
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8"\
)
NUMS=( 0 1 10 100 1000 10000 )

if ls | grep -qF '.out'; then
  rm *.out*
fi

compile=1
hyperfine=1

mode=
just_set_mode=0
urls_are_reset=0
nums_are_reset=0
server=http://localhost:9149

features=

out="benchmarks-$(date +%s).tar.gz"

for arg in "$@"; do
  shift
  case "$arg" in
    --no-compile)      compile=0    ;;
    --https)           protocol=https;;
    --no-hyperfine)    hyperfine=0  ;;
    --all)             hyperfine=1; callgrind=1; massif=1 ;;
    --urls)            mode=urls    ; just_set_mode=1 ;;
    --nums)            mode=nums    ; just_set_mode=1 ;;
    --features)        mode=features; just_set_mode=1 ;;
    --server)          mode=server  ; just_set_mode=1 ;;
    --protocol)        mode=protocol; just_set_mode=1 ;;
    --out)             mode=out     ; just_set_mode=1 ;;
    --)                break ;;
    --*)               echo Unknown option \"$arg\"; exit 1 ;;
    *) case "$mode" in
      urls) if [ $urls_are_reset -eq 0 ]; then URLS=( ); urls_are_reset=1; fi; URLS=( ${URLS[@]} "$arg" ) ;;
      nums) if [ $nums_are_reset -eq 0 ]; then NUMS=( ); nums_are_reset=1; fi; NUMS=( ${NUMS[@]} "$arg" ) ;;
      features) features=(--features "$arg") ;;
      server) server="$arg" ;;
      protocol) server=$(echo $server | sed -E "s/\w+/$arg/") ;;
      out) out="$arg" ;;
      "") echo "Modal argument without mode"; exit 1 ;;
    esac
  esac
  if [[ "$arg" =~ ^"--" && $just_set_mode -eq 0 ]]; then mode=; fi
  just_set_mode=0
done

if [  $hyperfine -eq 1 ] && ! which -s hyperfine; then echo 'Hyperfine not found; Please run `cargo install hyperfine`.'                         ; exit 2; fi
if [  $hyperfine -eq 1 ] && ! which -s jq       ; then echo 'Jq not found; Please install it. Also please learn it it'"'"'s a really handy tool.'; exit 2; fi
if [  $hyperfine -eq 1 ] && ! which -s bat      ; then echo 'Bat not found; Please run `cargo install bat`.'                                     ; exit 2; fi

if [ $compile -eq 1 ]; then
  if [ -e ../../target/release/url-cleaner-site ]; then
    old_mtime=$(stat -c %Y ../../target/release/url-cleaner-site)
  else
    old_mtime=0
  fi
  cargo build -r ${features[@]} --config profile.release.strip=false --config profile.release.debug=2
  if [ $? -ne 0 ]; then exit 3; fi
  if [ $old_mtime -lt $(stat -c %Y ../../target/release/url-cleaner-site) ]; then
    read -p "Press enter once you've (re)started URL Cleaner Site using the newly compiled binary." < /dev/tty
  fi
fi

COMMAND="curl --json @- $server/clean -f"

if [ $hyperfine -eq 1 ]; then
  curl $server &> /dev/null
  case $? in
    0) ;;
    1) echo 'Server protocol mismatch. Perhaps you forgot to specify `--protocol https`?'; exit 4 ;;
    7) echo 'Server not found. Perhaps it failed to start because of a broken config?'   ; exit 4 ;;
    *) echo "Unknown error when accessing server."                                       ; exit 4 ;;
  esac

  touch stdin
  hyperfine \
    -L num $(echo "${NUMS[@]}" | sed "s/ /,/g") \
    -L url $(echo "${URLS[@]}" | sed "s/ /,/g") \
    --prepare "bash -c \"yes '\\\"{url}\\\"' | head -n {num} | jq -sc '{tasks: .}' > stdin\"" \
    --warmup 100 \
    --runs 100 \
    --input stdin \
    -N \
    "$COMMAND" \
    --sort command \
    --export-json "hyperfine.out.json" \
    --command-name ""\
    --style color
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
    bat -pl json
fi

if ls | grep -qF '.out'; then
  tar -czf $out *.out*
  echo "Benchmark details compiled and compressed into $out"
else
  echo "No benchmark details generated."
fi
