#!/usr/bin/bash

cd $(dirname "$0")

URLS=(\
  "https://x.com?a=2"\
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id"\
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8"\
)
NUMS=( 0 1 10 100 1000 10000 )

rm -f *.out*

compile=1
hyperfine=1
valgrind=1
callgrind=1
cachegrind=1
massif=1
dhat=1
memcheck=1
an_only_is_set=0
mode=
just_set_mode=0
urls_are_reset=0
nums_are_reset=0
features=
out_file="benchmarks-$(date +%s).tar.gz"

for arg in "$@"; do
  shift
  case "$arg" in
    --no-compile)      compile=0 ;;
    --no-hyperfine)    hyperfine=0 ;;
    --no-valgrind)     valgrind=0 ;;
    --no-callgrind)    callgrind=0 ;;
    --no-cachegrind)   cachegrind=0 ;;
    --no-massif)       massif=0 ;;
    --no-dhat)         dhat=0 ;;
    --no-memcheck)     memcheck=0 ;;
    --only-hyperfine)  if [ $an_only_is_set -eq 0 ]; then an_only_is_set=1             ; valgrind=0                                                         ; else echo "Error: Multiple --only- flags were set."; exit 1; fi ;;
    --only-valgrind)   if [ $an_only_is_set -eq 0 ]; then an_only_is_set=1; hyperfine=0                                                                     ; else echo "Error: Multiple --only- flags were set."; exit 1; fi ;;
    --only-callgrind)  if [ $an_only_is_set -eq 0 ]; then an_only_is_set=1; hyperfine=0                         ; cachegrind=0; massif=0; dhat=0; memcheck=0; else echo "Error: Multiple --only- flags were set."; exit 1; fi ;;
    --only-cachegrind) if [ $an_only_is_set -eq 0 ]; then an_only_is_set=1; hyperfine=0            ; callgrind=0              ; massif=0; dhat=0; memcheck=0; else echo "Error: Multiple --only- flags were set."; exit 1; fi ;;
    --only-massif)     if [ $an_only_is_set -eq 0 ]; then an_only_is_set=1; hyperfine=0            ; callgrind=0; cachegrind=0          ; dhat=0; memcheck=0; else echo "Error: Multiple --only- flags were set."; exit 1; fi ;;
    --only-dhat)       if [ $an_only_is_set -eq 0 ]; then an_only_is_set=1; hyperfine=0            ; callgrind=0; cachegrind=0; massif=0        ; memcheck=0; else echo "Error: Multiple --only- flags were set."; exit 1; fi ;;
    --only-memcheck)   if [ $an_only_is_set -eq 0 ]; then an_only_is_set=1; hyperfine=0            ; callgrind=0; cachegrind=0; massif=0; dhat=0            ; else echo "Error: Multiple --only- flags were set."; exit 1; fi ;;
    --urls)            mode=urls    ; just_set_mode=1 ;;
    --nums)            mode=nums    ; just_set_mode=1 ;;
    --features)        mode=features; just_set_mode=1 ;;
    --out-file)        mode=out_file; just_set_mode=1 ;;
    --)                break ;;
    --*)               echo Unknown option \"$arg\"; exit 1 ;;
    *)                 if [ "$mode" == "urls" ]; then
                         if [ $urls_are_reset -eq 0 ]; then
                           URLS=( )
                           urls_are_reset=1
                         fi
                         URLS=( ${URLS[@]} "$arg" )
                       elif [ "$mode" == "nums" ]; then
                         if [ $nums_are_reset -eq 0 ]; then
                           NUMS=( )
                           nums_are_reset=1
                         fi
                         NUMS=( ${NUMS[@]} "$arg" )
                       elif [ "$mode" == "features" ]; then
                         features_arg=--features
                         features="$arg"
                         mode=
                       elif [ "$mode" == "out_file" ]; then
                         out_file="$arg"
                         mode=
                       else
                         echo "Modal arguments provided without a mode."
                         exit 1
                       fi ;;
  esac
  if [[ "$arg" =~ ^"--" && $just_set_mode -eq 0 ]]; then mode=; fi
  just_set_mode=0
done

if [ $hyperfine -eq 1 ] && ! which -s hyperfine; then echo 'Hyperfine not found; Please run `cargo install hyperfine`.'                         ; exit 2; fi
if [ $hyperfine -eq 1 ] && ! which -s jq       ; then echo 'Jq not found; Please install it. Also please learn it it'"'"'s a really handy tool.'; exit 2; fi
if [ $hyperfine -eq 1 ] && ! which -s bat      ; then echo 'Bat not found; Please run `cargo install bat`.'                                     ; exit 2; fi
if [ $valgrind  -eq 1 ] && ! which -s valgrind ; then echo 'Valgrind not found; Please install it.'                                             ; exit 2; fi

if [ $compile -eq 1 ]; then
  cargo build -r $features_arg $features --config profile.release.strip=false --config profile.release.debug=2
  if [ $? -ne 0 ]; then exit 3; fi
fi

COMMAND="../target/release/url-cleaner --config ../default-config.json $@"

if [ $hyperfine -eq 1 ]; then
  touch stdin
  hyperfine \
    -L num $(echo "${NUMS[@]}" | sed "s/ /,/g") \
    -L url $(echo "${URLS[@]}" | sed "s/ /,/g") \
    --prepare "bash -c \"yes '{url}' | head -n {num} > stdin\"" \
    --max-runs 100 \
    --warmup 20 \
    --input stdin \
    -N \
    "$COMMAND" \
    --sort command \
    --export-json "hyperfine.out.json" \
    --command-name ""
  rm stdin
  cat hyperfine.out.json |\
    jq 'reduce .results[] as $result ({}; .[$result.parameters.url][$result.parameters.num] = ($result.mean * 1000000 | floor / 1000 | tonumber))' |\
    sed -E ":a /^    .{0,7}\s\S/ s/:/: /g ; ta :b /^    .{,11}\./ s/:/: /g ; tb ; :c /^    .+\..{0,2}(,|$)/ s/,|$/0&/g ; tc" |\
    tee hyperfine.out-summary.json |\
    bat -pl json
fi

if [ $valgrind -eq 1 ]; then
  for url in "${URLS[@]}"; do
    file_safe_in_url=$(echo $url | head -c 50 | sed "s/\//-/g")
      for num in "${NUMS[@]}"; do
        if [ $callgrind -eq 1 ]; then
          echo "Callgrind  - $num - $url"
          yes "$url" | head -n $num | valgrind --quiet --tool=callgrind  --scheduling-quantum=1000 --callgrind-out-file="callgrind.out-$file_safe_in_url-$num-%p"   $COMMAND > /dev/null
        fi
        if [ $cachegrind -eq 1 ]; then
          echo "Cachegrind - $num - $url"
          yes "$url" | head -n $num | valgrind --quiet --tool=cachegrind --scheduling-quantum=1000 --cachegrind-out-file="cachegrind.out-$file_safe_in_url-$num-%p" $COMMAND > /dev/null
        fi
        if [ $massif -eq 1 ]; then
          echo "Massif     - $num - $url"
          yes "$url" | head -n $num | valgrind --quiet --tool=massif     --scheduling-quantum=1000 --massif-out-file="massif.out-$file_safe_in_url-$num-%p"         $COMMAND > /dev/null
        fi
        if [ $dhat -eq 1 ]; then
          echo "Dhat       - $num - $url"
          yes "$url" | head -n $num | valgrind --quiet --tool=dhat       --scheduling-quantum=1000 --dhat-out-file="dhat.out-$file_safe_in_url-$num-%p"             $COMMAND > /dev/null
        fi
        if [ $memcheck -eq 1 ]; then
          echo "Memcheck   - $num - $url"
          yes "$url" | head -n $num | valgrind --quiet --tool=memcheck   --scheduling-quantum=1000                                                                  $COMMAND > /dev/null 2> "memcheck.out-$file_safe_in_url-$num"
        fi
      done
  done
fi

tar -czf $out_file *.out*

echo "Benchmark details compiled and compressed into $out_file"
