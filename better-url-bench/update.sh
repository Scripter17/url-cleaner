#! /usr/bin/bash

echo "Updating Better URL Bench's public suffix list data"

cd "$(dirname "$0")"

curl 'https://publicsuffix.org/list/public_suffix_list.dat' -so src/public_suffix_list.dat
