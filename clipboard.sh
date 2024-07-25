#!/usr/bin/bash

# A simple bash script meant to be called via a keyboard shortcut
# Requires xclip be installed.
# TODO: Check if xclip works on wayland and, if not, modify this script to work on wayland

x=$(dirname "$0")

$x/target/release/url-cleaner "$(xclip -selection clipboard -o)" --config $x/default-config.json | tr -d '\n' | xclip -selection clipboard & # The & keeps xclip alive which is apparently important on KDE Plasma.
