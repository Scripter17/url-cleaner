#!/usr/bin/bash

cd $(dirname "$0")

cat "$1" | jq -c '.sets[] | [{task_config: .tests[].task_config, params_diff: .params_diff, job_context: .job_context}][]' > benchmarks.jsonl

count=$(cat benchmarks.jsonl | wc -l)

# TASK_CONFIGS=$(cat thing.json | jq -cr '[.[].task_config | tojson | gsub(","; "COMMA")] | join(",")')
# EXTRA_ARGS=$(  cat thing.json | jq -cr '[.[] | if .params_diff then "--params-diff-str " + (.params_diff | tojson) else "" end + if .job_context then "--job_context-diff-str " + (.job_context | tojson) else "" end] | join(",")')

echo "Doing $count benchmarks"

touch stdin.txt command.txt
hyperfine \
  --command-name ""\
  -P i 1 $count \
  --prepare '
    yes "$(cat benchmarks.jsonl | jq -scr ".[{i} - 1].task_config")" | head -n 10000 > stdin.txt
    cat benchmarks.jsonl | jq -scr "
      \"../../target/release/url-cleaner --cleaner ../../engine/default-cleaner.json\" +
      (.[{i} - 1] |
        if .params_diff then \" --params-diff-string \" + (.params_diff | tojson) else \"\" end +
        if .job_context then \" --job-context-string \" + (.job_context | tojson) else \"\" end
      )
    " > command.txt
  ' \
  --runs 100 \
  --warmup 100 \
  --input stdin.txt \
  --sort command \
  --export-json "hyperfine.out.json" \
  --style color \
  '$(cat command.txt)'
rm stdin.txt command.txt

paste <(cat hyperfine.out.json | jq '.results[].mean * 1000000 | floor / 1000') benchmarks.jsonl | sort -V | tee tests-summary.txt

rm benchmarks.jsonl
