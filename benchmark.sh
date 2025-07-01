#!/usr/bin/bash

cd $(dirname "$0")

mode=

CLI=1
CLIARGS=( --no-compile )

SITE=1
SITEARGS=( --no-compile )

COMPILE=1

for arg in "$@"; do
  case "$mode" in
    "") case "$arg" in
      --no-compile) COMPILE=0 ; CLIARGS=( "${CLIARGS[@]}" --no-compile ) ; SITEARGS=( "${SITEARGS[@]}" --no-compile ) ;;
      --only-compile) CLI=0 ; SITE=0 ;;

      --all-args) mode=all ;;

      --no-cli) CLI=0 ;;
      --cli-args) mode=cli ;;

      --no-site) SITE=0 ;;
      --site-args) mode=site ;;
    esac ;;
    all) case "$arg" in
      ";") mode= ;;
      *) CLIARGS=( "${CLIARGS[@]}" "$arg" ); SITEARGS=( "${SITEARGS[@]}" "$arg" ) ;;
    esac ;;
    cli) case "$arg" in
      ";") mode= ;;
      *) CLIARGS=( "${CLIARGS[@]}" "$arg" ) ;;
    esac ;;
    site) case "$arg" in
      ";") mode= ;;
      *) SITEARGS=( "${SITEARGS[@]}" "$arg" ) ;;
    esac ;;
  esac
done

if [ "$mode" != "" ]; then echo "Unfinished $mode args. Add a ; argument at the end."; exit 1; fi

if [ $COMPILE -eq 1 ]; then
  cargo build -r ${features[@]} --config profile.release.strip=false --config profile.release.debug=2
  if [ $? -ne 0 ]; then exit 3; fi
fi

if [ "$CLI" -eq 1 ]; then
  echo "Benchmarking the CLI."
  cli/benchmarking/benchmark.sh "${CLIARGS[@]}"
  if [ $? -ne 0 ]; then exit $?; fi
fi

if [ "$SITE" -eq 1 ]; then
  echo "Benchmarking the site."
  site/benchmarking/benchmark.sh "${SITEARGS[@]}"
  if [ $? -ne 0 ]; then exit $?; fi
fi
