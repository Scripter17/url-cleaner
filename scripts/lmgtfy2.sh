#!/usr/bin/bash

curl https://lmgtfy2.com |\
  grep -oPz "(?<=\()\{(.|\n)+?(?=\);)" |\
  sed -E "s/(\w+): /\"\1\": /g" |\
  sed -E "s/\o0/\n/g"|\
  jq -s "[.[] | {(.identifier): (.searchTypes | map({(.identifier): .urlBase}) | add)}] | add"
