#!/usr/bin/bash

# Calculates the value of the "rd_www_subdomain" NamedPartitioning based on handled domains.
# domains.sh outputs all domains in the NamedPartitioning, meaning this can be used to correct the NP while preserving correct manual insertions.

cd $(dirname "$0")/..

get_list=1

for arg in "$@"; do
  shift
  case "$arg" in
    --dont-get) get_list=0 ;;
    *) echo "???"; exit 1 ;;
  esac
done

if [ "$get_list" -eq 1 ]; then
  domains=$($(dirname $0)/domains.sh | sed -E 's/(.+)/\1\nwww.\1/')

  echo "Getting redirects for $(echo "$domains" | wc -l) domains." > /dev/stderr

  for domain in $domains; do
    headers=$(curl -m 5 -s "https://$domain" -o /dev/null -D -)
    redirect=$(echo "$headers" | grep -i '^location' | sed -E 's/^[^:]+:\s*//')
    if [ "$redirect" ]; then
      echo "$domain -> $redirect"
    fi
  done > www-subdomains-result.txt
fi

echo     '{'
echo -ne '\t\t\t\t"ensure": '; cat www-subdomains-result.txt | grep -P '^(\S+) -> https?://www\.\1' | sed -E 's/^(\S+).+/\1/'      | jq -sRc 'split("\n")[:-1]' | sed 's/\[/[\n\t\t\t\t\t/; s/\]/\n\t\t\t\t]/' | $(dirname $0)/indent-thing.sh; echo ','
echo -ne '\t\t\t\t"remove": '; cat www-subdomains-result.txt | grep -P '^www\.(\S+) -> https?://\1' | sed -E 's/^www\.(\S+).+/\1/' | jq -sRc 'split("\n")[:-1]' | sed 's/\[/[\n\t\t\t\t\t/; s/\]/\n\t\t\t\t]/' | $(dirname $0)/indent-thing.sh; echo
echo -ne '\t\t\t}'

cd - &> /dev/null
