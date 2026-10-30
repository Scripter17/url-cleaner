#!/usr/bin/bash

echo "Updating Better URL's test data"

cd "$(dirname "$0")"

echo 'Updating tests from WPT'
curl 'https://raw.githubusercontent.com/web-platform-tests/wpt/refs/heads/master/url/resources/urltestdata.json'   -so urltestdata.json
curl 'https://raw.githubusercontent.com/web-platform-tests/wpt/refs/heads/master/url/resources/setters_tests.json' -so setters_tests.json
curl 'https://raw.githubusercontent.com/web-platform-tests/wpt/refs/heads/master/url/resources/IdnaTestV2.json'    -so IdnaTestV2.json
curl 'https://raw.githubusercontent.com/web-platform-tests/wpt/refs/heads/master/url/resources/toascii.json'       -so toascii.json

echo 'Updating IDNA from Ada'
curl 'https://raw.githubusercontent.com/ada-url/idna/refs/heads/main/tests/fixtures/to_unicode_alternating.txt'    -so to_unicode_alternating.txt
curl 'https://raw.githubusercontent.com/ada-url/idna/refs/heads/main/tests/fixtures/to_ascii_alternating.txt'      -so to_ascii_alternating.txt
curl 'https://raw.githubusercontent.com/ada-url/idna/refs/heads/main/tests/fixtures/to_ascii_invalid.txt'          -so to_ascii_invalid.txt

echo 'Updating other from Ada'
curl 'https://raw.githubusercontent.com/ada-url/ada/refs/heads/main/tests/wpt/ada_extra_urltestdata.json'          -so ada_extra_urltestdata.json
curl 'https://raw.githubusercontent.com/ada-url/ada/refs/heads/main/tests/wpt/ada_extra_setters_tests.json'        -so ada_extra_setters_tests.json
curl 'https://raw.githubusercontent.com/ada-url/ada/refs/heads/main/tests/wpt/utf8_punycode_alternating.txt'       -so utf8_punycode_alternating.txt
