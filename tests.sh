#!/usr/bin/bash

build=1
build_ndf=1
cargo_test=1
order_test=1
offline_tests=1
online_tests=1
clippy=1
powerset=1

POWERSET_DEPTH=2

mode=

for arg in "$@"; do
  shift
  case "$arg" in
    --no-cargo-test)    cargo_test=0    ;;
    --no-build)         build=0         ;;
    --no-build-ndf)     build_ndf=0     ;;
    --no-test-order)    test_order=0    ;;
    --no-offline-tests) offline_tests=0 ;;
    --no-online-tests)  online_tests=0  ;;
    --no-clippy)        clippy=0        ;;
    --no-powerset)      powerset=0      ;;

    --only-cargo-test)    cargo_test=1 ; build=0 ; build_ndf=0 ; order_test=0 ; offline_tests=0 ; online_tests=0 ; clippy=0 ; powerset=0 ;;
    --only-build)         cargo_test=0 ; build=1 ; build_ndf=0 ; order_test=0 ; offline_tests=0 ; online_tests=0 ; clippy=0 ; powerset=0 ;;
    --only-build-ndf)     cargo_test=0 ; build=0 ; build_ndf=1 ; order_test=0 ; offline_tests=0 ; online_tests=0 ; clippy=0 ; powerset=0 ;;
    --only-test-order)    cargo_test=0 ; build=0 ; build_ndf=0 ; order_test=1 ; offline_tests=0 ; online_tests=0 ; clippy=0 ; powerset=0 ;;
    --only-offline-tests) cargo_test=0 ; build=0 ; build_ndf=0 ; order_test=0 ; offline_tests=1 ; online_tests=0 ; clippy=0 ; powerset=0 ;;
    --only-online-tests)  cargo_test=0 ; build=0 ; build_ndf=0 ; order_test=0 ; offline_tests=0 ; online_tests=1 ; clippy=0 ; powerset=0 ;;
    --only-clippy)        cargo_test=0 ; build=0 ; build_ndf=0 ; order_test=0 ; offline_tests=0 ; online_tests=0 ; clippy=1 ; powerset=0 ;;
    --only-powerset)      cargo_test=0 ; build=0 ; build_ndf=0 ; order_test=0 ; offline_tests=0 ; online_tests=0 ; clippy=0 ; powerset=1 ;;

    --powerset-depth) mode=powerset_depth ;;

    --*) echo "Unknwon option $arg"; exit 1 ;;

    *) case "$mode" in
      powerset_depth) POWERSET_DEPTH=$arg ;;
      *) echo "Modal argument without mode"; exit 1 ;;
    esac
  esac
done

if [ $cargo_test -eq 1 ]; then
  cargo test
  if [ $? -ne 0 ]; then echo "Failed at cargo test"; exit 1; fi
fi

if [ $build -eq 1 ]; then
  cargo build
  if [ $? -ne 0 ]; then echo "Failed at build"; exit 1; fi
fi

if [ $build_ndf -eq 1 ]; then
  cargo build --no-default-features
  if [ $? -ne 0 ]; then echo "Failed at build no default features"; exit 1; fi
fi

if [ $order_test -eq 1 ]; then
  x=$(seq -f "https://%0.0f.com/" 10)
  if [ "$(cargo run -- $x)" != "$x" ]; then exit "Failed at order test"; exit 1; fi
fi

if [ $offline_tests -eq 1 ]; then
  cargo run -- --tests tests.json
  if [ $? -ne 0 ]; then echo "Failed at offline tests"; exit 1; fi
fi

if [ $online_tests -eq 1 ]; then
  cargo run -- --tests network-tests.json
  if [ $? -ne 0 ]; then echo "Failed at online tests"; exit 1; fi
fi

if [ $clippy -eq 1 ]; then
  RUSTFLAGS=-Awarnings cargo clippy
  if [ $? -ne 0 ]; then echo "Failed at clippy"; exit 1; fi
fi

if [ $powerset -eq 1 ]; then
  RUSTFLAGS=-Awarnings cargo hack test --feature-powerset --depth $POWERSET_DEPTH
  if [ $? -ne 0 ]; then echo "Failed at test powerset"; exit 1; fi
fi
