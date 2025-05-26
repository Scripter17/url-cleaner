#!/usr/bin/bash

cd $(dirname "$0")

cat ../engine/default-cleaner.json |\
  jq '
    (.. | try select(contains({part: "Domain"}) or contains({part: "Host"})).map | try keys[]),
    (.params.named_partitionings.hwwwwdpafqdnp_categories[][])
  ' -r |\
  sort -u |\
  grep -Pv '\.(onion|i2p)$'

cd - &> /dev/null
