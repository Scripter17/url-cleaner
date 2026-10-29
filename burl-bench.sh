#!/usr/bin/bash

mkdir -p better-url-bench/out

cargo run --bin urlc-tool -- build burl-bench

tabs 15;

echo;       cat better-url-bench/data/idna-ascii.txt   | target/release/better-url-bench idna ascii   --all | tee better-url-bench/out/A2A.txt    | faketty cut -f 2,3,6         | target/release/better-url-bench monitor --markdown --title A2A                --columns BURL Servo Ada
echo;       cat better-url-bench/data/idna-unicode.txt | target/release/better-url-bench idna ascii   --all | tee better-url-bench/out/U2A.txt    | faketty cut -f 2,3,6         | target/release/better-url-bench monitor --markdown --title U2A                --columns BURL Servo Ada
echo;       cat better-url-bench/data/idna.txt         | target/release/better-url-bench idna ascii   --all | tee better-url-bench/out/D2A.txt    | faketty cut -f 2,3,6         | target/release/better-url-bench monitor --markdown --title D2A                --columns BURL Servo Ada
echo;       cat better-url-bench/data/idna-ascii.txt   | target/release/better-url-bench idna unicode --all | tee better-url-bench/out/A2U.txt    | faketty cut -f 2,3,6         | target/release/better-url-bench monitor --markdown --title A2U                --columns BURL Servo Ada
echo;       cat better-url-bench/data/idna-unicode.txt | target/release/better-url-bench idna unicode --all | tee better-url-bench/out/U2U.txt    | faketty cut -f 2,3,6         | target/release/better-url-bench monitor --markdown --title U2U                --columns BURL Servo Ada
echo;       cat better-url-bench/data/idna.txt         | target/release/better-url-bench idna unicode --all | tee better-url-bench/out/D2U.txt    | faketty cut -f 2,3,6         | target/release/better-url-bench monitor --markdown --title D2U                --columns BURL Servo Ada
echo;       cat better-url-bench/data/hosts.txt        | target/release/better-url-bench parse host         | tee better-url-bench/out/hosts.txt  | faketty cut -f 2-12          | target/release/better-url-bench monitor --markdown --title Hosts  --no-factor --columns File SpecialNotFile NonSpecial Domain Ipv4 Ipv6 Opaque Empty ServoNormal ServoOpaque
echo; time (cat better-url-bench/data/top100.txt       | target/release/better-url-bench parse url --all    | tee better-url-bench/out/top100.txt | faketty cut -f 2,3,6,9,12,15 | target/release/better-url-bench monitor --markdown --title top100             --columns BURL Servo Servo+ Ada Ada+)
echo; time (cat better-url-bench/data/reddit.txt       | target/release/better-url-bench parse url --all    | tee better-url-bench/out/reddit.txt | faketty cut -f 2,3,6,9,12,15 | target/release/better-url-bench monitor --markdown --title reddit             --columns BURL Servo Servo+ Ada Ada+)
