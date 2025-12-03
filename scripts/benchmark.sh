#!/usr/bin/bash

cargo +nightly run -p url-cleaner-benchmarking -- suite --cli --site-http --site-ws --hyperfine --massif | tee benchmarks.md
