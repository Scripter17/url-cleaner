#!/usr/bin/bash

cat benchmarking/hyperfine.out-* | jq -s ". | map({(.results[0].parameters.url): .results | map({(.parameters.num): {mean: .mean, min: .min, max: .max}})})"
