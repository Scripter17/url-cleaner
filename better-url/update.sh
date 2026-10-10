#! /usr/bin/bash

echo 'Updating Unicode'
curl 'https://www.unicode.org/Public/latest/idna/IdnaMappingTable.txt' -Lso src/util/parts/host/domain/IdnaMappingTable.txt

./src/tests/data/update.sh
