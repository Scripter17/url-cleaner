#!/usr/bin/bash

x=$(dirname "$0")

$x/target/release/url-cleaner "$(xclip -selection clipboard -o)" --config $x/default-config.json | tr -d '\n' | xclip -selection clipboard &
