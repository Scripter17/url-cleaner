#! /usr/bin/bash

echo "Updating Better URL's data"

cd "$(dirname "$0")"

./src/tests/data/update.sh
