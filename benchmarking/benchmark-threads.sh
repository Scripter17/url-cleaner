#!/usr/bin/bash

THREAD_COUNTS=( 0 1 2 4 8 16 )
THREAD_QUEUES=( 1 10 100 1000 )
JSON=( "" --json )

benchmark_args=( )
url_cleaner_args=( )
place=self
mode=
thread_counts_are_reset=0
thread_queues_are_reset=0

rm -f thread-benchmark-*

now=$(date +%s)

for arg in "$@"; do
  shift
  case "$place" in
    self) case "$arg" in
      --thread-counts) mode=thread_counts ;;
      --thread-queues) mode=thread_queues ;;
      --) place=benchmark ;;
      *) case "$mode" in
        thread_counts) if [ $thread_counts_are_reset -eq 0 ]; then THREAD_COUNTS=( ); thread_counts_are_reset=1; fi; THREAD_COUNTS=( ${THREAD_COUNTS[@]} "$arg" ) ;;
        thread_queues) if [ $thread_queues_are_reset -eq 0 ]; then THREAD_QUEUES=( ); thread_queues_are_reset=1; fi; THREAD_QUEUES=( ${THREAD_QUEUES[@]} "$arg" ) ;;
      esac ;;
    esac ;;
    benchmark) case "$arg" in
      --) place=url_cleaner ;;
      *) benchmark_args=( ${benchmark_args[@]} "$arg" )
    esac ;;
    url_cleaner) case "$arg" in
      --) echo "Arg error."; exit 1 ;;
      *) url_cleaner_args=( ${url_cleaner_args[@]} "$arg" )
    esac ;;
  esac
done

for thread_count in "${THREAD_COUNTS[@]}"; do
  for thread_queue in "${THREAD_QUEUES[@]}"; do
    for json in "${JSON[@]}"; do
      ./benchmark.sh --features experiment-parallel --out-file "thread-benchmark-$now-$thread_count-$thread_queue-$json.tar.gz" ${benchmark_args[@]} -- --threads $thread_count --thread-queue $thread_queue $json ${url_cleaner_args[@]}
    done
  done
done

tar -czf "thread-benchmarks-$now.tar.gz" thread-benchmark-*
