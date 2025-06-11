#!/usr/bin/bash

# Get all domains from the default cleaner.

cd $(dirname "$0")/..

cat engine/default-cleaner.json |\
  jq '
    (.actions | .. | objects | select(.part | try test("Domain|Host")).map | try keys[]),
    (.params | .. | objects | to_entries[] | select(.key | test("^(nh|dm|rd)_")).value | .. | strings)
  ' -r |\
  sort -u |\
  grep -F '.' |\
  grep -Pv '\.(onion|i2p)$'

cd - &> /dev/null
