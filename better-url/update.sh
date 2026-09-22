#! /usr/bin/bash

curl 'https://raw.githubusercontent.com/web-platform-tests/wpt/refs/heads/master/url/resources/urltestdata.json'   -o src/tests/data/urltestdata.json
curl 'https://raw.githubusercontent.com/web-platform-tests/wpt/refs/heads/master/url/resources/setters_tests.json' -o src/tests/data/setters_tests.json
curl 'https://raw.githubusercontent.com/web-platform-tests/wpt/refs/heads/master/url/resources/IdnaTestV2.json'    -o src/tests/data/IdnaTestV2.json
curl -L 'https://www.unicode.org/Public/latest/idna/IdnaMappingTable.txt' -o src/util/parts/host/domain/IdnaMappingTable.txt
