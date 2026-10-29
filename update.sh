#!/usr/bin/bash

cd "$(dirname "$0")"

echo "Updating Better URL Bench's public suffix list data"
curl 'https://publicsuffix.org/list/public_suffix_list.dat' -so better-url-bench/src/public_suffix_list.dat

echo "Updating Better URL's data"
./better-url/update.sh
