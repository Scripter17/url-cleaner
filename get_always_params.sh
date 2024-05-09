#!/usr/bin/bash

# Grabs query parameters from various sources and prints a JSON fragment to insert into the default config.

rm -f temp.txt
curl https://raw.githubusercontent.com/AdguardTeam/AdguardFilters/master/TrackParamFilter/sections/general_url.txt | grep ^\$removeparam | awk -F = '{print $2}' >> temp.txt
echo >> temp.txt
curl https://raw.githubusercontent.com/brave/brave-core/master/components/query_filter/utils.cc | grep -E \\s\{12\}\" | grep -oE \\w+ >> temp.txt
echo >> temp.txt
cat default-config.json | grep always -A 1000 | grep -P "\s{6}" | grep -oE \\w+ >> temp.txt
cat temp.txt | grep -v ^utm_ | sort -u | grep -E . | sed -E "s/^|$/\"/g" | paste -sd , | sed "s/,/, /g" | fold -sw 120
rm temp.txt
