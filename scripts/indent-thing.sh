#!/usr/bin/bash

# Sort, deduplicated, fold, and re-indent an array.
# Mostly in that order.

cd $(dirname "$0")/..

width=160
mode=
for arg in "$@"; do
  shift
  case "$arg" in
    --width) mode=width ;;
    -w) mode=width ;;
    -*) echo "???"; exit 1 ;;
    *) case "$mode" in
      width) width="$arg"
    esac ;;
  esac
done

input=$(cat)

tabs1=$(echo "$input" | grep -oP '\t+(?=")' -m 1)
tabs1c=$(echo -n "$tabs1" | wc -c)
tabs2=$(echo "$input" | grep -oP '\t+(?=\])' -m 1)
tabs2c=$(echo -n "$tabs2" | wc -c)

{
  echo '['
  echo "$input" | jq '.[]' -r | sort -u | sed -Ez 's/([^\n]+)/"\1"/g; s/\n$//; s/\n/, /g' | fold -sw $width | sed "s/^/$tabs1/; s/ $//"
  echo
  echo -n "$tabs2]"
}

cd - &> /dev/null
