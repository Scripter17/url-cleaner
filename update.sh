#!/usr/bin/bash

echo "Updating URL Cleaner's data"

cd "$(dirname "$0")"

./better-url-bench/update.sh
./better-url/update.sh
